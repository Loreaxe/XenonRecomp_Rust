pub fn sub_82F332C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F332C8 size=136
    let mut pc: u32 = 0x82F332C8;
    'dispatch: loop {
        match pc {
            0x82F332C8 => {
    //   block [0x82F332C8..0x82F33350)
	// 82F332C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F332CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F332D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F332D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F332D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F332DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F332E0: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F332E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F332E8: 419A0010  beq cr6, 0x82f332f8
	if ctx.cr[6].eq {
	pc = 0x82F332F8; continue 'dispatch;
	}
	// 82F332EC: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F332F0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F332F4: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F332F8: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F332FC: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82F33300: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F33304: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F33308: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F3330C: 409A0010  bne cr6, 0x82f3331c
	if !ctx.cr[6].eq {
	pc = 0x82F3331C; continue 'dispatch;
	}
	// 82F33310: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F33314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F33318: 4BF73569  bl 0x82ea6880
	ctx.lr = 0x82F3331C;
	sub_82EA6880(ctx, base);
	// 82F3331C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33320: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33324: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F33328: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82F3332C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33330: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F33334: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F33338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3333C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F33348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3334C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33350 size=144
    let mut pc: u32 = 0x82F33350;
    'dispatch: loop {
        match pc {
            0x82F33350 => {
    //   block [0x82F33350..0x82F333E0)
	// 82F33350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3335C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F33360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33364: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F33368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3336C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F33370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F33374: 388AF3D0  addi r4, r10, -0xc30
	ctx.r[4].s64 = ctx.r[10].s64 + -3120;
	// 82F33378: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3337C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F33380: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F33384: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33388: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3338C: 4E800421  bctrl
	ctx.lr = 0x82F33390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F33390: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33394: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F33398: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F3339C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F333A0: 3888ECC4  addi r4, r8, -0x133c
	ctx.r[4].s64 = ctx.r[8].s64 + -4924;
	// 82F333A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F333A8: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F333AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F333B0: 4E800421  bctrl
	ctx.lr = 0x82F333B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F333B4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F333B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F333BC: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F333C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F333C4: 4E800421  bctrl
	ctx.lr = 0x82F333C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F333C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F333CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F333D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F333D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F333D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F333DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F333E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F333E0 size=8
    let mut pc: u32 = 0x82F333E0;
    'dispatch: loop {
        match pc {
            0x82F333E0 => {
    //   block [0x82F333E0..0x82F333E8)
	// 82F333E0: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82F333E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F333E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F333E8 size=132
    let mut pc: u32 = 0x82F333E8;
    'dispatch: loop {
        match pc {
            0x82F333E8 => {
    //   block [0x82F333E8..0x82F3346C)
	// 82F333E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F333EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F333F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F333F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F333F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F333FC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F33400: 394BE78C  addi r10, r11, -0x1874
	ctx.r[10].s64 = ctx.r[11].s64 + -6260;
	// 82F33404: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F33408: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F3340C: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33410: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F33414: 419A0030  beq cr6, 0x82f33444
	if ctx.cr[6].eq {
	pc = 0x82F33444; continue 'dispatch;
	}
	// 82F33418: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F3341C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F33420: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F33424: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F33428: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F3342C: 409A0018  bne cr6, 0x82f33444
	if !ctx.cr[6].eq {
	pc = 0x82F33444; continue 'dispatch;
	}
	// 82F33430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33434: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F33438: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3343C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F33440: 4E800421  bctrl
	ctx.lr = 0x82F33444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F33444: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82F33448: 4BFE4221  bl 0x82f17668
	ctx.lr = 0x82F3344C;
	sub_82F17668(ctx, base);
	// 82F3344C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F33450: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F33454: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F33458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3345C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F33468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33470 size=20
    let mut pc: u32 = 0x82F33470;
    'dispatch: loop {
        match pc {
            0x82F33470 => {
    //   block [0x82F33470..0x82F33484)
	// 82F33470: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F33474: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33478: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F3347C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F33480: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33488 size=96
    let mut pc: u32 = 0x82F33488;
    'dispatch: loop {
        match pc {
            0x82F33488 => {
    //   block [0x82F33488..0x82F334E8)
	// 82F33488: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3348C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82F33490: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F33494: 392BE78C  addi r9, r11, -0x1874
	ctx.r[9].s64 = ctx.r[11].s64 + -6260;
	// 82F33498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F3349C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F334A0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F334A4: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82F334A8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F334AC: 388ABD10  addi r4, r10, -0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + -17136;
	// 82F334B0: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F334B4: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F334B8: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82F334BC: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82F334C0: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F334C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F334C8: 419A0010  beq cr6, 0x82f334d8
	if ctx.cr[6].eq {
	pc = 0x82F334D8; continue 'dispatch;
	}
	// 82F334CC: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F334D0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F334D4: B1450006  sth r10, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F334D8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F334DC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F334E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F334E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F334E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F334E8 size=16
    let mut pc: u32 = 0x82F334E8;
    'dispatch: loop {
        match pc {
            0x82F334E8 => {
    //   block [0x82F334E8..0x82F334F8)
	// 82F334E8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F334EC: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82F334F0: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F334F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F334F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F334F8 size=216
    let mut pc: u32 = 0x82F334F8;
    'dispatch: loop {
        match pc {
            0x82F334F8 => {
    //   block [0x82F334F8..0x82F335D0)
	// 82F334F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F334FC: 48274C6D  bl 0x831a8168
	ctx.lr = 0x82F33500;
	sub_831A8130(ctx, base);
	// 82F33500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33504: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33508: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F3350C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F33510: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F33514: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F33518: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3351C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33520: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F33524: 40980020  bge cr6, 0x82f33544
	if !ctx.cr[6].lt {
	pc = 0x82F33544; continue 'dispatch;
	}
	// 82F33528: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3352C: 3909F3D8  addi r8, r9, -0xc28
	ctx.r[8].s64 = ctx.r[9].s64 + -3112;
	// 82F33530: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F33534: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F33538: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 82F3353C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F33540: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F33544: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F33548: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3354C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F33550: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F33554: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F33558: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F3355C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33560: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F33564: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F33568: 4E800421  bctrl
	ctx.lr = 0x82F3356C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3356C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F33570: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F33574: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F33578: 891E0000  lbz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3357C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82F33580: 419A0014  beq cr6, 0x82f33594
	if ctx.cr[6].eq {
	pc = 0x82F33594; continue 'dispatch;
	}
	// 82F33584: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F33588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F3358C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F33590: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82F33594: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F33598: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3359C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F335A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F335A4: 40980020  bge cr6, 0x82f335c4
	if !ctx.cr[6].lt {
	pc = 0x82F335C4; continue 'dispatch;
	}
	// 82F335A8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F335AC: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F335B0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F335B4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F335B8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F335BC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F335C0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F335C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F335C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F335CC: 48274BEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F335D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F335D0 size=196
    let mut pc: u32 = 0x82F335D0;
    'dispatch: loop {
        match pc {
            0x82F335D0 => {
    //   block [0x82F335D0..0x82F33694)
	// 82F335D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F335D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F335D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F335DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F335E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F335E4: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F335E8: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F335EC: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F335F0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F335F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F335F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F335FC: 40980020  bge cr6, 0x82f3361c
	if !ctx.cr[6].lt {
	pc = 0x82F3361C; continue 'dispatch;
	}
	// 82F33600: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F33604: 3909F3D8  addi r8, r9, -0xc28
	ctx.r[8].s64 = ctx.r[9].s64 + -3112;
	// 82F33608: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3360C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F33610: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F33614: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F33618: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F3361C: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F33620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F33624: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F33628: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82F3362C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F33630: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F33634: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F33638: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F3363C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33640: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F33644: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F33648: 4E800421  bctrl
	ctx.lr = 0x82F3364C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3364C: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F33650: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33654: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F33658: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F3365C: 40980020  bge cr6, 0x82f3367c
	if !ctx.cr[6].lt {
	pc = 0x82F3367C; continue 'dispatch;
	}
	// 82F33660: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F33664: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F33668: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3366C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F33670: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F33674: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F33678: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3367C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F33680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33688: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3368C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F33690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33698 size=152
    let mut pc: u32 = 0x82F33698;
    'dispatch: loop {
        match pc {
            0x82F33698 => {
    //   block [0x82F33698..0x82F33730)
	// 82F33698: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82F3369C: 8145000C  lwz r10, 0xc(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F336A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F336A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F336A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F336AC: 419A0024  beq cr6, 0x82f336d0
	if ctx.cr[6].eq {
	pc = 0x82F336D0; continue 'dispatch;
	}
	// 82F336B0: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 82F336B4: 90A90000  stw r5, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82F336B8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82F336BC: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F336C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F336C4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82F336C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F336CC: 409AFFE8  bne cr6, 0x82f336b4
	if !ctx.cr[6].eq {
	pc = 0x82F336B4; continue 'dispatch;
	}
	// 82F336D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F336D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F336D8: 40990040  ble cr6, 0x82f33718
	if !ctx.cr[6].gt {
	pc = 0x82F33718; continue 'dispatch;
	}
	// 82F336DC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F336E0: 38E1FFD0  addi r7, r1, -0x30
	ctx.r[7].s64 = ctx.r[1].s64 + -48;
	// 82F336E4: 38C4FFFF  addi r6, r4, -1
	ctx.r[6].s64 = ctx.r[4].s64 + -1;
	// 82F336E8: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82F336EC: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82F336F0: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F336F4: 40980024  bge cr6, 0x82f33718
	if !ctx.cr[6].lt {
	pc = 0x82F33718; continue 'dispatch;
	}
	// 82F336F8: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82F336FC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F33700: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82F33704: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33708: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3370C: 90A90000  stw r5, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82F33710: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82F33714: 4181FFDC  bgt 0x82f336f0
	if ctx.cr[0].gt {
	pc = 0x82F336F0; continue 'dispatch;
	}
	// 82F33718: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F3371C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F33720: 38680001  addi r3, r8, 1
	ctx.r[3].s64 = ctx.r[8].s64 + 1;
	// 82F33724: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82F33728: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82F3372C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33730 size=72
    let mut pc: u32 = 0x82F33730;
    'dispatch: loop {
        match pc {
            0x82F33730 => {
    //   block [0x82F33730..0x82F33778)
	// 82F33730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3373C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F33740: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F33744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F33748: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3374C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F33750: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F33754: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F33758: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3375C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F33760: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F33764: 48004245  bl 0x82f379a8
	ctx.lr = 0x82F33768;
	sub_82F379A8(ctx, base);
	// 82F33768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3376C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F33778 size=76
    let mut pc: u32 = 0x82F33778;
    'dispatch: loop {
        match pc {
            0x82F33778 => {
    //   block [0x82F33778..0x82F337C4)
	// 82F33778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3377C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33784: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F33788: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3378C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F33790: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F33794: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F33798: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3379C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F337A0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F337A4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F337A8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F337AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F337B0: 48003FA9  bl 0x82f37758
	ctx.lr = 0x82F337B4;
	sub_82F37758(ctx, base);
	// 82F337B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F337B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F337BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F337C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F337C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F337C8 size=224
    let mut pc: u32 = 0x82F337C8;
    'dispatch: loop {
        match pc {
            0x82F337C8 => {
    //   block [0x82F337C8..0x82F338A8)
	// 82F337C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F337CC: 482749A1  bl 0x831a816c
	ctx.lr = 0x82F337D0;
	sub_831A8130(ctx, base);
	// 82F337D0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F337D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F337D8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F337DC: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F337E0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F337E4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F337E8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F337EC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F337F0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F337F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F337F8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F337FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F33800: 4200FFF0  bdnz 0x82f337f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F337F0; continue 'dispatch;
	}
	// 82F33804: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F33808: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F3380C: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F33810: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F33814: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F33818: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F3381C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F33820: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F338A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F338A8 size=148
    let mut pc: u32 = 0x82F338A8;
    'dispatch: loop {
        match pc {
            0x82F338A8 => {
    //   block [0x82F338A8..0x82F3393C)
	// 82F338A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F338AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F338B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F338B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F338B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F338BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F338C0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F338C4: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 82F338C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F338CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F338D0: 419A0054  beq cr6, 0x82f33924
	if ctx.cr[6].eq {
	pc = 0x82F33924; continue 'dispatch;
	}
	// 82F338D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F338D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F338DC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F338E0: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F338E4: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F338E8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F338EC: 4198001C  blt cr6, 0x82f33908
	if ctx.cr[6].lt {
	pc = 0x82F33908; continue 'dispatch;
	}
	// 82F338F0: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F338F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F338F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F338FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F33900: 4BF6C761  bl 0x82ea0060
	ctx.lr = 0x82F33904;
	sub_82EA0060(ctx, base);
	// 82F33904: 48000020  b 0x82f33924
	pc = 0x82F33924; continue 'dispatch;
	// 82F33908: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F3390C: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82F33910: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F33914: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F33918: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82F3391C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F33920: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82F33924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F33928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3392C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F33938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33940 size=148
    let mut pc: u32 = 0x82F33940;
    'dispatch: loop {
        match pc {
            0x82F33940 => {
    //   block [0x82F33940..0x82F339D4)
	// 82F33940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33948: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3394C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F33954: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F33958: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F3395C: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 82F33960: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F33964: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F33968: 419A0054  beq cr6, 0x82f339bc
	if ctx.cr[6].eq {
	pc = 0x82F339BC; continue 'dispatch;
	}
	// 82F3396C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33970: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F33974: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F33978: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F3397C: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F33980: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F33984: 4198001C  blt cr6, 0x82f339a0
	if ctx.cr[6].lt {
	pc = 0x82F339A0; continue 'dispatch;
	}
	// 82F33988: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F3398C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F33990: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F33994: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F33998: 4BF6C6C9  bl 0x82ea0060
	ctx.lr = 0x82F3399C;
	sub_82EA0060(ctx, base);
	// 82F3399C: 48000020  b 0x82f339bc
	pc = 0x82F339BC; continue 'dispatch;
	// 82F339A0: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F339A4: 394B0058  addi r10, r11, 0x58
	ctx.r[10].s64 = ctx.r[11].s64 + 88;
	// 82F339A8: 814B0058  lwz r10, 0x58(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F339AC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F339B0: 912B005C  stw r9, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82F339B4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F339B8: 93EB0058  stw r31, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82F339BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F339C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F339C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F339C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F339CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F339D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F339D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F339D8 size=148
    let mut pc: u32 = 0x82F339D8;
    'dispatch: loop {
        match pc {
            0x82F339D8 => {
    //   block [0x82F339D8..0x82F33A6C)
	// 82F339D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F339DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F339E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F339E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F339E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F339EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F339F0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F339F4: 392BC3C0  addi r9, r11, -0x3c40
	ctx.r[9].s64 = ctx.r[11].s64 + -15424;
	// 82F339F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F339FC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F33A00: 419A0054  beq cr6, 0x82f33a54
	if ctx.cr[6].eq {
	pc = 0x82F33A54; continue 'dispatch;
	}
	// 82F33A04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33A08: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F33A0C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F33A10: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F33A14: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F33A18: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F33A1C: 4198001C  blt cr6, 0x82f33a38
	if ctx.cr[6].lt {
	pc = 0x82F33A38; continue 'dispatch;
	}
	// 82F33A20: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F33A24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F33A28: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82F33A2C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F33A30: 4BF6C631  bl 0x82ea0060
	ctx.lr = 0x82F33A34;
	sub_82EA0060(ctx, base);
	// 82F33A34: 48000020  b 0x82f33a54
	pc = 0x82F33A54; continue 'dispatch;
	// 82F33A38: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F33A3C: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82F33A40: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F33A44: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F33A48: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82F33A4C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F33A50: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82F33A54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F33A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F33A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33A64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F33A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33A70 size=256
    let mut pc: u32 = 0x82F33A70;
    'dispatch: loop {
        match pc {
            0x82F33A70 => {
    //   block [0x82F33A70..0x82F33B70)
	// 82F33A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33A74: 482746F9  bl 0x831a816c
	ctx.lr = 0x82F33A78;
	sub_831A8130(ctx, base);
	// 82F33A78: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33A7C: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F33A80: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F33A84: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F33A88: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F33A8C: 38CA3730  addi r6, r10, 0x3730
	ctx.r[6].s64 = ctx.r[10].s64 + 14128;
	// 82F33A90: 38A93778  addi r5, r9, 0x3778
	ctx.r[5].s64 = ctx.r[9].s64 + 14200;
	// 82F33A94: 388837C8  addi r4, r8, 0x37c8
	ctx.r[4].s64 = ctx.r[8].s64 + 14280;
	// 82F33A98: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F33A9C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F33AA0: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F33AA4: 38EB4578  addi r7, r11, 0x4578
	ctx.r[7].s64 = ctx.r[11].s64 + 17784;
	// 82F33AA8: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F33AAC: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F33AB0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F33AB4: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F33AB8: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82F33ABC: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F33AC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F33AC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F33AC8: 4BFF1979  bl 0x82f25440
	ctx.lr = 0x82F33ACC;
	sub_82F25440(ctx, base);
	// 82F33ACC: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F33AD0: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F33AD4: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F33AD8: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F33ADC: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F33AE0: 39033DE0  addi r8, r3, 0x3de0
	ctx.r[8].s64 = ctx.r[3].s64 + 15840;
	// 82F33AE4: 38CA7758  addi r6, r10, 0x7758
	ctx.r[6].s64 = ctx.r[10].s64 + 30552;
	// 82F33AE8: 38A94FF8  addi r5, r9, 0x4ff8
	ctx.r[5].s64 = ctx.r[9].s64 + 20472;
	// 82F33AEC: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F33AF0: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 82F33AF4: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F33AF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F33AFC: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F33B00: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F33B04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F33B08: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82F33B0C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82F33B10: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F33B14: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F33B18: 4BFF1929  bl 0x82f25440
	ctx.lr = 0x82F33B1C;
	sub_82F25440(ctx, base);
	// 82F33B1C: 3C8082F3  lis r4, -0x7d0d
	ctx.r[4].s64 = -2098003968;
	// 82F33B20: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F33B24: 9BC100A0  stb r30, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u8 ) };
	// 82F33B28: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F33B2C: 9BE100A1  stb r31, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[31].u8 ) };
	// 82F33B30: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F33B34: 392451A8  addi r9, r4, 0x51a8
	ctx.r[9].s64 = ctx.r[4].s64 + 20904;
	// 82F33B38: 390379A8  addi r8, r3, 0x79a8
	ctx.r[8].s64 = ctx.r[3].s64 + 31144;
	// 82F33B3C: 38AA4FF8  addi r5, r10, 0x4ff8
	ctx.r[5].s64 = ctx.r[10].s64 + 20472;
	// 82F33B40: 91210090  stw r9, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 82F33B44: 38EB7758  addi r7, r11, 0x7758
	ctx.r[7].s64 = ctx.r[11].s64 + 30552;
	// 82F33B48: 91010094  stw r8, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[8].u32 ) };
	// 82F33B4C: 90A1009C  stw r5, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[5].u32 ) };
	// 82F33B50: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82F33B54: 90E10098  stw r7, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82F33B58: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82F33B5C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F33B60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F33B64: 4BFF18DD  bl 0x82f25440
	ctx.lr = 0x82F33B68;
	sub_82F25440(ctx, base);
	// 82F33B68: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F33B6C: 48274650  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33B70 size=20
    let mut pc: u32 = 0x82F33B70;
    'dispatch: loop {
        match pc {
            0x82F33B70 => {
    //   block [0x82F33B70..0x82F33B84)
	// 82F33B70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F33B74: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F33B78: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F33B7C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F33B80: 48003E28  b 0x82f379a8
	sub_82F379A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33B88 size=20
    let mut pc: u32 = 0x82F33B88;
    'dispatch: loop {
        match pc {
            0x82F33B88 => {
    //   block [0x82F33B88..0x82F33B9C)
	// 82F33B88: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F33B8C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F33B90: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F33B94: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F33B98: 48003BC0  b 0x82f37758
	sub_82F37758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33BA0 size=24
    let mut pc: u32 = 0x82F33BA0;
    'dispatch: loop {
        match pc {
            0x82F33BA0 => {
    //   block [0x82F33BA0..0x82F33BB8)
	// 82F33BA0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F33BA4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F33BA8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F33BAC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F33BB0: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82F33BB4: 48003964  b 0x82f37518
	sub_82F37518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33BB8 size=84
    let mut pc: u32 = 0x82F33BB8;
    'dispatch: loop {
        match pc {
            0x82F33BB8 => {
    //   block [0x82F33BB8..0x82F33C0C)
	// 82F33BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33BC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F33BC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33BC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F33BCC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82F33BD0: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F33BD4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F33BD8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F33BDC: 480493FD  bl 0x82f7cfd8
	ctx.lr = 0x82F33BE0;
	sub_82F7CFD8(ctx, base);
	// 82F33BE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33BE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F33BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F33BEC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33BF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F33BF4: 4E800421  bctrl
	ctx.lr = 0x82F33BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F33BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F33BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F33C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33C10 size=8
    let mut pc: u32 = 0x82F33C10;
    'dispatch: loop {
        match pc {
            0x82F33C10 => {
    //   block [0x82F33C10..0x82F33C18)
	// 82F33C10: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F33C14: 48049024  b 0x82f7cc38
	sub_82F7CC38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33C18 size=8
    let mut pc: u32 = 0x82F33C18;
    'dispatch: loop {
        match pc {
            0x82F33C18 => {
    //   block [0x82F33C18..0x82F33C20)
	// 82F33C18: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F33C1C: 4804916C  b 0x82f7cd88
	sub_82F7CD88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33C20 size=132
    let mut pc: u32 = 0x82F33C20;
    'dispatch: loop {
        match pc {
            0x82F33C20 => {
    //   block [0x82F33C20..0x82F33CA4)
	// 82F33C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33C24: 48274541  bl 0x831a8164
	ctx.lr = 0x82F33C28;
	sub_831A8130(ctx, base);
	// 82F33C28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F33C30: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F33C34: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82F33C38: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82F33C3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F33C40: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F33C44: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33CA8 size=172
    let mut pc: u32 = 0x82F33CA8;
    'dispatch: loop {
        match pc {
            0x82F33CA8 => {
    //   block [0x82F33CA8..0x82F33D54)
	// 82F33CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33CB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F33CB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F33CB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33CBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F33CC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F33CC4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F33CC8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F33CCC: 388AF414  addi r4, r10, -0xbec
	ctx.r[4].s64 = ctx.r[10].s64 + -3052;
	// 82F33CD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33CD4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F33CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F33CDC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33CE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F33CE4: 4E800421  bctrl
	ctx.lr = 0x82F33CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F33CE8: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82F33CEC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F33CF0: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F33CF4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F33CF8: 409A0034  bne cr6, 0x82f33d2c
	if !ctx.cr[6].eq {
	pc = 0x82F33D2C; continue 'dispatch;
	}
	// 82F33CFC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33D00: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F33D04: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33D08: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F33D0C: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33D10: 3889F408  addi r4, r9, -0xbf8
	ctx.r[4].s64 = ctx.r[9].s64 + -3064;
	// 82F33D14: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F33D18: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F33D1C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F33D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F33D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F33D28: 4E800421  bctrl
	ctx.lr = 0x82F33D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F33D2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F33D30: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F33D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F33D38: 4803BE89  bl 0x82f6fbc0
	ctx.lr = 0x82F33D3C;
	sub_82F6FBC0(ctx, base);
	// 82F33D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F33D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F33D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F33D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F33D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F33D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33D58 size=136
    let mut pc: u32 = 0x82F33D58;
    'dispatch: loop {
        match pc {
            0x82F33D58 => {
    //   block [0x82F33D58..0x82F33DE0)
	// 82F33D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F33D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F33D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F33D6C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F33D70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F33D74: 392BF424  addi r9, r11, -0xbdc
	ctx.r[9].s64 = ctx.r[11].s64 + -3036;
	// 82F33D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F33D7C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F33D80: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F33D84: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F33D88: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82F33D8C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F33D90: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F33D94: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82F33D98: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F33D9C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82F33DA0: 911F0038  stw r8, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[8].u32 ) };
	// 82F33DA4: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33DA8: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F33DE0 size=88
    let mut pc: u32 = 0x82F33DE0;
    'dispatch: loop {
        match pc {
            0x82F33DE0 => {
    //   block [0x82F33DE0..0x82F33E38)
	// 82F33DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33DE4: 48274385  bl 0x831a8168
	ctx.lr = 0x82F33DE8;
	sub_831A8130(ctx, base);
	// 82F33DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F33DEC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F33DF0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F33DF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F33DF8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F33DFC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F33E00: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F33E04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F33E08: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F33E0C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F33E10: 4BF6C921  bl 0x82ea0730
	ctx.lr = 0x82F33E14;
	sub_82EA0730(ctx, base);
	// 82F33E14: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F33E18: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F33E1C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F33E20: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F33E24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F33E28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F33E2C: 4BFFFF2D  bl 0x82f33d58
	ctx.lr = 0x82F33E30;
	sub_82F33D58(ctx, base);
	// 82F33E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F33E34: 48274384  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F33E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F33E38 size=1552
    let mut pc: u32 = 0x82F33E38;
    'dispatch: loop {
        match pc {
            0x82F33E38 => {
    //   block [0x82F33E38..0x82F34448)
	// 82F33E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F33E3C: 48274305  bl 0x831a8140
	ctx.lr = 0x82F33E40;
	sub_831A8130(ctx, base);
	// 82F33E40: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82F33E44: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82F33E48: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34448 size=300
    let mut pc: u32 = 0x82F34448;
    'dispatch: loop {
        match pc {
            0x82F34448 => {
    //   block [0x82F34448..0x82F34574)
	// 82F34448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3444C: 48273D1D  bl 0x831a8168
	ctx.lr = 0x82F34450;
	sub_831A8130(ctx, base);
	// 82F34450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34454: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F34458: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3445C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F34460: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F34464: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F34468: 419A00BC  beq cr6, 0x82f34524
	if ctx.cr[6].eq {
	pc = 0x82F34524; continue 'dispatch;
	}
	// 82F3446C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F34470: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F34474: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F34478: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3447C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34480: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F34484: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F34488: 4E800421  bctrl
	ctx.lr = 0x82F3448C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3448C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82F34490: 41980058  blt cr6, 0x82f344e8
	if ctx.cr[6].lt {
	pc = 0x82F344E8; continue 'dispatch;
	}
	// 82F34494: 419A0018  beq cr6, 0x82f344ac
	if ctx.cr[6].eq {
	pc = 0x82F344AC; continue 'dispatch;
	}
	// 82F34498: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82F3449C: 4198004C  blt cr6, 0x82f344e8
	if ctx.cr[6].lt {
	pc = 0x82F344E8; continue 'dispatch;
	}
	// 82F344A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F344A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F344A8: 48273D10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F344AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F344B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F344B4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F344B8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F344BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F344C0: 4BF6C271  bl 0x82ea0730
	ctx.lr = 0x82F344C4;
	sub_82EA0730(ctx, base);
	// 82F344C4: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F344C8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F344CC: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F344D0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F344D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F344D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F344DC: 48012CBD  bl 0x82f47198
	ctx.lr = 0x82F344E0;
	sub_82F47198(ctx, base);
	// 82F344E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F344E4: 48273CD4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F344E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F344EC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F344F0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F344F4: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F344F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F344FC: 4BF6C235  bl 0x82ea0730
	ctx.lr = 0x82F34500;
	sub_82EA0730(ctx, base);
	// 82F34500: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F34504: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F34508: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3450C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F34510: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F34514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F34518: 4BFFF841  bl 0x82f33d58
	ctx.lr = 0x82F3451C;
	sub_82F33D58(ctx, base);
	// 82F3451C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F34520: 48273C98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F34524: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34528: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3452C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F34530: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F34534: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F34538: 4BF6C1F9  bl 0x82ea0730
	ctx.lr = 0x82F3453C;
	sub_82EA0730(ctx, base);
	// 82F3453C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F34540: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F34544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F34548: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3454C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F34550: B13C0004  sth r9, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F34554: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F34558: 4BFFF801  bl 0x82f33d58
	ctx.lr = 0x82F3455C;
	sub_82F33D58(ctx, base);
	// 82F3455C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F34560: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F34564: 38E8F484  addi r7, r8, -0xb7c
	ctx.r[7].s64 = ctx.r[8].s64 + -2940;
	// 82F34568: 90FC0000  stw r7, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3456C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F34570: 48273C48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34578 size=108
    let mut pc: u32 = 0x82F34578;
    'dispatch: loop {
        match pc {
            0x82F34578 => {
    //   block [0x82F34578..0x82F345E4)
	// 82F34578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3457C: 48273BE9  bl 0x831a8164
	ctx.lr = 0x82F34580;
	sub_831A8130(ctx, base);
	// 82F34580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34584: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34588: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3458C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F34590: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F34594: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F34598: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F3459C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F345A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F345A4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F345A8: 4BF6C189  bl 0x82ea0730
	ctx.lr = 0x82F345AC;
	sub_82EA0730(ctx, base);
	// 82F345AC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F345B0: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F345B4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F345B8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F345BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F345C0: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F345C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F345C8: 4BFFF791  bl 0x82f33d58
	ctx.lr = 0x82F345CC;
	sub_82F33D58(ctx, base);
	// 82F345CC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F345D0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F345D4: 38E8F484  addi r7, r8, -0xb7c
	ctx.r[7].s64 = ctx.r[8].s64 + -2940;
	// 82F345D8: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F345DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F345E0: 48273BD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F345E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F345E8 size=204
    let mut pc: u32 = 0x82F345E8;
    'dispatch: loop {
        match pc {
            0x82F345E8 => {
    //   block [0x82F345E8..0x82F346B4)
	// 82F345E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F345EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F345F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F345F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F345F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F345FC: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F34600: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F34604: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F34608: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F3460C: 38CA3730  addi r6, r10, 0x3730
	ctx.r[6].s64 = ctx.r[10].s64 + 14128;
	// 82F34610: 38A93778  addi r5, r9, 0x3778
	ctx.r[5].s64 = ctx.r[9].s64 + 14200;
	// 82F34614: 388849A8  addi r4, r8, 0x49a8
	ctx.r[4].s64 = ctx.r[8].s64 + 18856;
	// 82F34618: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3461C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F34620: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F34624: 38EB4578  addi r7, r11, 0x4578
	ctx.r[7].s64 = ctx.r[11].s64 + 17784;
	// 82F34628: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3462C: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F34630: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F34634: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F34638: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82F3463C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F34640: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F34644: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F34648: 4BFF0DF9  bl 0x82f25440
	ctx.lr = 0x82F3464C;
	sub_82F25440(ctx, base);
	// 82F3464C: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F34650: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F34654: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F34658: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F3465C: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F34660: 39033DE0  addi r8, r3, 0x3de0
	ctx.r[8].s64 = ctx.r[3].s64 + 15840;
	// 82F34664: 38CA7758  addi r6, r10, 0x7758
	ctx.r[6].s64 = ctx.r[10].s64 + 30552;
	// 82F34668: 38A97518  addi r5, r9, 0x7518
	ctx.r[5].s64 = ctx.r[9].s64 + 29976;
	// 82F3466C: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F34670: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 82F34674: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F34678: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F3467C: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F34680: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F34684: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82F34688: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F3468C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F34690: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F34694: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F34698: 4BFF0DA9  bl 0x82f25440
	ctx.lr = 0x82F3469C;
	sub_82F25440(ctx, base);
	// 82F3469C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F346A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F346A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F346A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F346AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F346B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F346B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F346B8 size=204
    let mut pc: u32 = 0x82F346B8;
    'dispatch: loop {
        match pc {
            0x82F346B8 => {
    //   block [0x82F346B8..0x82F34784)
	// 82F346B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F346BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F346C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F346C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F346C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F346CC: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F346D0: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F346D4: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F346D8: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F346DC: 38CA3730  addi r6, r10, 0x3730
	ctx.r[6].s64 = ctx.r[10].s64 + 14128;
	// 82F346E0: 38A93778  addi r5, r9, 0x3778
	ctx.r[5].s64 = ctx.r[9].s64 + 14200;
	// 82F346E4: 388849A8  addi r4, r8, 0x49a8
	ctx.r[4].s64 = ctx.r[8].s64 + 18856;
	// 82F346E8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F346EC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F346F0: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F346F4: 38EB4578  addi r7, r11, 0x4578
	ctx.r[7].s64 = ctx.r[11].s64 + 17784;
	// 82F346F8: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F346FC: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F34700: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F34704: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F34708: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82F3470C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F34710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F34714: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F34718: 4BFF0D29  bl 0x82f25440
	ctx.lr = 0x82F3471C;
	sub_82F25440(ctx, base);
	// 82F3471C: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F34720: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F34724: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F34728: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F3472C: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F34730: 39033DE0  addi r8, r3, 0x3de0
	ctx.r[8].s64 = ctx.r[3].s64 + 15840;
	// 82F34734: 38CA7758  addi r6, r10, 0x7758
	ctx.r[6].s64 = ctx.r[10].s64 + 30552;
	// 82F34738: 38A97518  addi r5, r9, 0x7518
	ctx.r[5].s64 = ctx.r[9].s64 + 29976;
	// 82F3473C: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F34740: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 82F34744: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F34748: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F3474C: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F34750: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F34754: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82F34758: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F3475C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82F34760: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F34764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F34768: 4BFF0CD9  bl 0x82f25440
	ctx.lr = 0x82F3476C;
	sub_82F25440(ctx, base);
	// 82F3476C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F34770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34778: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3477C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F34780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34788 size=336
    let mut pc: u32 = 0x82F34788;
    'dispatch: loop {
        match pc {
            0x82F34788 => {
    //   block [0x82F34788..0x82F348D8)
	// 82F34788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3478C: 482739D9  bl 0x831a8164
	ctx.lr = 0x82F34790;
	sub_831A8130(ctx, base);
	// 82F34790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34794: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F34798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3479C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F347A0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F347A4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F347A8: 419A00E0  beq cr6, 0x82f34888
	if ctx.cr[6].eq {
	pc = 0x82F34888; continue 'dispatch;
	}
	// 82F347AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F347B0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F347B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F347B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F347BC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F347C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F347C4: 4E800421  bctrl
	ctx.lr = 0x82F347C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F347C8: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82F347CC: 4198006C  blt cr6, 0x82f34838
	if ctx.cr[6].lt {
	pc = 0x82F34838; continue 'dispatch;
	}
	// 82F347D0: 419A0018  beq cr6, 0x82f347e8
	if ctx.cr[6].eq {
	pc = 0x82F347E8; continue 'dispatch;
	}
	// 82F347D4: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82F347D8: 41980060  blt cr6, 0x82f34838
	if ctx.cr[6].lt {
	pc = 0x82F34838; continue 'dispatch;
	}
	// 82F347DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F347E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F347E4: 482739D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F347E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F347EC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F347F0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F347F4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F347F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F347FC: 4BF6BF35  bl 0x82ea0730
	ctx.lr = 0x82F34800;
	sub_82EA0730(ctx, base);
	// 82F34800: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F34804: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F34808: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3480C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F34810: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F34814: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F34818: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3481C: 4801297D  bl 0x82f47198
	ctx.lr = 0x82F34820;
	sub_82F47198(ctx, base);
	// 82F34820: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F34824: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F34828: 38E8F4C0  addi r7, r8, -0xb40
	ctx.r[7].s64 = ctx.r[8].s64 + -2880;
	// 82F3482C: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F34830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F34834: 48273980  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F34838: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3483C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F34840: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F34844: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F34848: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3484C: 4BF6BEE5  bl 0x82ea0730
	ctx.lr = 0x82F34850;
	sub_82EA0730(ctx, base);
	// 82F34850: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F34854: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F34858: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3485C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F34860: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F34864: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F34868: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3486C: 4BFFF4ED  bl 0x82f33d58
	ctx.lr = 0x82F34870;
	sub_82F33D58(ctx, base);
	// 82F34870: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F34874: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F34878: 38E8F484  addi r7, r8, -0xb7c
	ctx.r[7].s64 = ctx.r[8].s64 + -2940;
	// 82F3487C: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F34880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F34884: 48273930  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F34888: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3488C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F34890: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F34894: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F34898: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3489C: 4BF6BE95  bl 0x82ea0730
	ctx.lr = 0x82F348A0;
	sub_82EA0730(ctx, base);
	// 82F348A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F348A4: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F348A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F348AC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F348B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F348B4: B13C0004  sth r9, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F348B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F348BC: 4BFFF49D  bl 0x82f33d58
	ctx.lr = 0x82F348C0;
	sub_82F33D58(ctx, base);
	// 82F348C0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F348C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F348C8: 38E8F484  addi r7, r8, -0xb7c
	ctx.r[7].s64 = ctx.r[8].s64 + -2940;
	// 82F348CC: 90FC0000  stw r7, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F348D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F348D4: 482738E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F348D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F348D8 size=204
    let mut pc: u32 = 0x82F348D8;
    'dispatch: loop {
        match pc {
            0x82F348D8 => {
    //   block [0x82F348D8..0x82F349A4)
	// 82F348D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F348DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F348E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F348E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F348E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F348EC: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F348F0: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F348F4: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F348F8: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F348FC: 38CA3730  addi r6, r10, 0x3730
	ctx.r[6].s64 = ctx.r[10].s64 + 14128;
	// 82F34900: 38A93778  addi r5, r9, 0x3778
	ctx.r[5].s64 = ctx.r[9].s64 + 14200;
	// 82F34904: 388849A8  addi r4, r8, 0x49a8
	ctx.r[4].s64 = ctx.r[8].s64 + 18856;
	// 82F34908: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3490C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F34910: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F34914: 38EB4788  addi r7, r11, 0x4788
	ctx.r[7].s64 = ctx.r[11].s64 + 18312;
	// 82F34918: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3491C: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F34920: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82F34924: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F34928: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82F3492C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F34930: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F34934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F34938: 4BFF0B09  bl 0x82f25440
	ctx.lr = 0x82F3493C;
	sub_82F25440(ctx, base);
	// 82F3493C: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F34940: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F34944: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F34948: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F3494C: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F34950: 39034448  addi r8, r3, 0x4448
	ctx.r[8].s64 = ctx.r[3].s64 + 17480;
	// 82F34954: 38CA7758  addi r6, r10, 0x7758
	ctx.r[6].s64 = ctx.r[10].s64 + 30552;
	// 82F34958: 38A97518  addi r5, r9, 0x7518
	ctx.r[5].s64 = ctx.r[9].s64 + 29976;
	// 82F3495C: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F34960: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 82F34964: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F34968: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F3496C: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F34970: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F34974: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82F34978: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F3497C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82F34980: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F34984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F34988: 4BFF0AB9  bl 0x82f25440
	ctx.lr = 0x82F3498C;
	sub_82F25440(ctx, base);
	// 82F3498C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F34990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3499C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F349A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F349A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F349A8 size=224
    let mut pc: u32 = 0x82F349A8;
    'dispatch: loop {
        match pc {
            0x82F349A8 => {
    //   block [0x82F349A8..0x82F34A88)
	// 82F349A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F349AC: 482737C1  bl 0x831a816c
	ctx.lr = 0x82F349B0;
	sub_831A8130(ctx, base);
	// 82F349B0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F349B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F349B8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F349BC: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F349C0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F349C4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F349C8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F349CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F349D0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F349D4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F349D8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F349DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F349E0: 4200FFF0  bdnz 0x82f349d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F349D0; continue 'dispatch;
	}
	// 82F349E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F349E8: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F349EC: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F349F0: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F349F4: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F349F8: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F349FC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F34A00: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F34A88 size=116
    let mut pc: u32 = 0x82F34A88;
    'dispatch: loop {
        match pc {
            0x82F34A88 => {
    //   block [0x82F34A88..0x82F34AFC)
	// 82F34A88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F34A8C: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82F34A90: 392BC5A0  addi r9, r11, -0x3a60
	ctx.r[9].s64 = ctx.r[11].s64 + -14944;
	// 82F34A94: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34AFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F34AFC size=20
    let mut pc: u32 = 0x82F34AFC;
    'dispatch: loop {
        match pc {
            0x82F34AFC => {
    //   block [0x82F34AFC..0x82F34B10)
	// 82F34AFC: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82F34B00: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F34B04: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82F34B08: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82F34B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34B10 size=68
    let mut pc: u32 = 0x82F34B10;
    'dispatch: loop {
        match pc {
            0x82F34B10 => {
    //   block [0x82F34B10..0x82F34B54)
	// 82F34B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34B1C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F34B20: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F34B24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F34B28: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F34B2C: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F34B30: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F34B34: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F34B38: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F34B3C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F34B40: 48002E69  bl 0x82f379a8
	ctx.lr = 0x82F34B44;
	sub_82F379A8(ctx, base);
	// 82F34B44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F34B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F34B58 size=72
    let mut pc: u32 = 0x82F34B58;
    'dispatch: loop {
        match pc {
            0x82F34B58 => {
    //   block [0x82F34B58..0x82F34BA0)
	// 82F34B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34B64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F34B68: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F34B6C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F34B70: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F34B74: 392AF3E8  addi r9, r10, -0xc18
	ctx.r[9].s64 = ctx.r[10].s64 + -3096;
	// 82F34B78: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F34B7C: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F34B80: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F34B84: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F34B88: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F34B8C: 48002BCD  bl 0x82f37758
	ctx.lr = 0x82F34B90;
	sub_82F37758(ctx, base);
	// 82F34B90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F34B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F34BA0 size=16
    let mut pc: u32 = 0x82F34BA0;
    'dispatch: loop {
        match pc {
            0x82F34BA0 => {
    //   block [0x82F34BA0..0x82F34BB0)
	// 82F34BA0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F34BA4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F34BA8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F34BAC: 4BFFF074  b 0x82f33c20
	sub_82F33C20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F34BB0 size=176
    let mut pc: u32 = 0x82F34BB0;
    'dispatch: loop {
        match pc {
            0x82F34BB0 => {
    //   block [0x82F34BB0..0x82F34C60)
	// 82F34BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F34BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F34BC0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F34BC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34BC8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F34BCC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F34BD0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F34BD4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F34BD8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34BDC: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F34BE0: 4BFFF259  bl 0x82f33e38
	ctx.lr = 0x82F34BE4;
	sub_82F33E38(ctx, base);
	// 82F34BE4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34BE8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F34BEC: 40980044  bge cr6, 0x82f34c30
	if !ctx.cr[6].lt {
	pc = 0x82F34C30; continue 'dispatch;
	}
	// 82F34BF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F34BF4: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F34BF8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34C60 size=164
    let mut pc: u32 = 0x82F34C60;
    'dispatch: loop {
        match pc {
            0x82F34C60 => {
    //   block [0x82F34C60..0x82F34D04)
	// 82F34C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34C68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F34C6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F34C70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34C74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F34C78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F34C7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F34C80: 394BF424  addi r10, r11, -0xbdc
	ctx.r[10].s64 = ctx.r[11].s64 + -3036;
	// 82F34C84: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F34C88: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F34C8C: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F34C90: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F34C94: 409A0020  bne cr6, 0x82f34cb4
	if !ctx.cr[6].eq {
	pc = 0x82F34CB4; continue 'dispatch;
	}
	// 82F34C98: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34C9C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F34CA0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F34CA4: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F34CA8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F34CAC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F34CB0: 4BF6BB01  bl 0x82ea07b0
	ctx.lr = 0x82F34CB4;
	sub_82EA07B0(ctx, base);
	// 82F34CB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F34CB8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F34CBC: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F34CC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F34CC4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F34CC8: 419A0020  beq cr6, 0x82f34ce8
	if ctx.cr[6].eq {
	pc = 0x82F34CE8; continue 'dispatch;
	}
	// 82F34CCC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34CD0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F34CD4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F34CD8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F34CDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F34CE0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F34CE4: 4BF6BACD  bl 0x82ea07b0
	ctx.lr = 0x82F34CE8;
	sub_82EA07B0(ctx, base);
	// 82F34CE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F34CEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F34CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34CF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F34CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F34D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F34D08 size=16
    let mut pc: u32 = 0x82F34D08;
    'dispatch: loop {
        match pc {
            0x82F34D08 => {
    //   block [0x82F34D08..0x82F34D18)
	// 82F34D08: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F34D0C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F34D10: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F34D14: 4801220C  b 0x82f46f20
	sub_82F46F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34D18 size=72
    let mut pc: u32 = 0x82F34D18;
    'dispatch: loop {
        match pc {
            0x82F34D18 => {
    //   block [0x82F34D18..0x82F34D60)
	// 82F34D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34D24: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F34D28: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F34D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F34D30: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F34D34: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F34D38: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F34D3C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F34D40: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F34D44: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F34D48: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F34D4C: 48011F05  bl 0x82f46c50
	ctx.lr = 0x82F34D50;
	sub_82F46C50(ctx, base);
	// 82F34D50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F34D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F34D60 size=76
    let mut pc: u32 = 0x82F34D60;
    'dispatch: loop {
        match pc {
            0x82F34D60 => {
    //   block [0x82F34D60..0x82F34DAC)
	// 82F34D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34D6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F34D70: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F34D74: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F34D78: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F34D7C: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F34D80: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F34D84: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F34D88: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F34D8C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F34D90: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F34D94: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F34D98: 48011929  bl 0x82f466c0
	ctx.lr = 0x82F34D9C;
	sub_82F466C0(ctx, base);
	// 82F34D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F34DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F34DB0 size=176
    let mut pc: u32 = 0x82F34DB0;
    'dispatch: loop {
        match pc {
            0x82F34DB0 => {
    //   block [0x82F34DB0..0x82F34E60)
	// 82F34DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34DB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F34DBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F34DC0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F34DC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34DC8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F34DCC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F34DD0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F34DD4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F34DD8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34DDC: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F34DE0: 480117C1  bl 0x82f465a0
	ctx.lr = 0x82F34DE4;
	sub_82F465A0(ctx, base);
	// 82F34DE4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34DE8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F34DEC: 40980044  bge cr6, 0x82f34e30
	if !ctx.cr[6].lt {
	pc = 0x82F34E30; continue 'dispatch;
	}
	// 82F34DF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F34DF4: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F34DF8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34E60 size=224
    let mut pc: u32 = 0x82F34E60;
    'dispatch: loop {
        match pc {
            0x82F34E60 => {
    //   block [0x82F34E60..0x82F34F40)
	// 82F34E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34E64: 48273305  bl 0x831a8168
	ctx.lr = 0x82F34E68;
	sub_831A8130(ctx, base);
	// 82F34E68: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34E6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F34E70: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F34E74: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F34E78: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F34E7C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F34E80: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F34E84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F34E88: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F34E8C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F34E90: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F34E94: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F34E98: 4200FFF0  bdnz 0x82f34e88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F34E88; continue 'dispatch;
	}
	// 82F34E9C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F34EA0: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F34EA4: E9060058  ld r8, 0x58(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F34EA8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F34EAC: 38CAC5B0  addi r6, r10, -0x3a50
	ctx.r[6].s64 = ctx.r[10].s64 + -14928;
	// 82F34EB0: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F34EB4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F34EB8: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34F40 size=120
    let mut pc: u32 = 0x82F34F40;
    'dispatch: loop {
        match pc {
            0x82F34F40 => {
    //   block [0x82F34F40..0x82F34FB8)
	// 82F34F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34F48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F34F4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34F50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F34F54: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F34F58: 394BF4FC  addi r10, r11, -0xb04
	ctx.r[10].s64 = ctx.r[11].s64 + -2820;
	// 82F34F5C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F34F60: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F34F64: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F34F68: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F34F6C: 409A002C  bne cr6, 0x82f34f98
	if !ctx.cr[6].eq {
	pc = 0x82F34F98; continue 'dispatch;
	}
	// 82F34F70: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F34F74: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F34F78: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F34F7C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F34F80: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82F34F84: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F34F88: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F34F8C: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F34F90: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F34F94: 4BF6B81D  bl 0x82ea07b0
	ctx.lr = 0x82F34F98;
	sub_82EA07B0(ctx, base);
	// 82F34F98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F34F9C: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F34FA0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F34FA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F34FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F34FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F34FB8 size=60
    let mut pc: u32 = 0x82F34FB8;
    'dispatch: loop {
        match pc {
            0x82F34FB8 => {
    //   block [0x82F34FB8..0x82F34FF4)
	// 82F34FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F34FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F34FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F34FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F34FCC: 480007DD  bl 0x82f357a8
	ctx.lr = 0x82F34FD0;
	sub_82F357A8(ctx, base);
	// 82F34FD0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F34FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F34FD8: 394BF538  addi r10, r11, -0xac8
	ctx.r[10].s64 = ctx.r[11].s64 + -2760;
	// 82F34FDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F34FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F34FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F34FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F34FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F34FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F34FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F34FF8 size=432
    let mut pc: u32 = 0x82F34FF8;
    'dispatch: loop {
        match pc {
            0x82F34FF8 => {
    //   block [0x82F34FF8..0x82F351A8)
	// 82F34FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F34FFC: 48273161  bl 0x831a815c
	ctx.lr = 0x82F35000;
	sub_831A8130(ctx, base);
	// 82F35000: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35004: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35008: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82F3500C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F35010: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F35014: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F35018: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F3501C: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F35020: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F35024: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35028: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3502C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F35030: 40980020  bge cr6, 0x82f35050
	if !ctx.cr[6].lt {
	pc = 0x82F35050; continue 'dispatch;
	}
	// 82F35034: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F35038: 3909F570  addi r8, r9, -0xa90
	ctx.r[8].s64 = ctx.r[9].s64 + -2704;
	// 82F3503C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F35040: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F35044: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F35048: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3504C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F35050: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82F35054: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35058: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3505C: 4BF6C005  bl 0x82ea1060
	ctx.lr = 0x82F35060;
	sub_82EA1060(ctx, base);
	// 82F35060: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35064: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F35068: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82F3506C: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82F35070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35074: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F35078: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3507C: 4E800421  bctrl
	ctx.lr = 0x82F35080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35080: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82F35084: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F35088: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3508C: 38A6FFA0  addi r5, r6, -0x60
	ctx.r[5].s64 = ctx.r[6].s64 + -96;
	// 82F35090: C1BF0004  lfs f13, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F35094: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F35098: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82F3509C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82F350A0: C0099450  lfs f0, -0x6bb0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F350A4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F351A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F351A8 size=176
    let mut pc: u32 = 0x82F351A8;
    'dispatch: loop {
        match pc {
            0x82F351A8 => {
    //   block [0x82F351A8..0x82F35258)
	// 82F351A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F351AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F351B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F351B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F351B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F351BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F351C0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F351C4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F351C8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F351CC: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F351D0: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F351D4: 810A0010  lwz r8, 0x10(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F351D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F351DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F351E0: 80E90024  lwz r7, 0x24(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F351E4: 80C80024  lwz r6, 0x24(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F351E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F351EC: 7F073000  cmpw cr6, r7, r6
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F351F0: 40980028  bge cr6, 0x82f35218
	if !ctx.cr[6].lt {
	pc = 0x82F35218; continue 'dispatch;
	}
	// 82F351F4: 4BF6B53D  bl 0x82ea0730
	ctx.lr = 0x82F351F8;
	sub_82EA0730(ctx, base);
	// 82F351F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F351FC: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F35200: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F35204: B13E0004  sth r9, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F35208: 480005A1  bl 0x82f357a8
	ctx.lr = 0x82F3520C;
	sub_82F357A8(ctx, base);
	// 82F3520C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F35210: 38E8F538  addi r7, r8, -0xac8
	ctx.r[7].s64 = ctx.r[8].s64 + -2760;
	// 82F35214: 48000024  b 0x82f35238
	pc = 0x82F35238; continue 'dispatch;
	// 82F35218: 4BF6B519  bl 0x82ea0730
	ctx.lr = 0x82F3521C;
	sub_82EA0730(ctx, base);
	// 82F3521C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F35220: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F35224: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F35228: B13E0004  sth r9, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3522C: 4800057D  bl 0x82f357a8
	ctx.lr = 0x82F35230;
	sub_82F357A8(ctx, base);
	// 82F35230: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F35234: 38E8F57C  addi r7, r8, -0xa84
	ctx.r[7].s64 = ctx.r[8].s64 + -2692;
	// 82F35238: 90FE0000  stw r7, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3523C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F35240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F35244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F35248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3524C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F35250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F35254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35258 size=256
    let mut pc: u32 = 0x82F35258;
    'dispatch: loop {
        match pc {
            0x82F35258 => {
    //   block [0x82F35258..0x82F35358)
	// 82F35258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3525C: 48272F11  bl 0x831a816c
	ctx.lr = 0x82F35260;
	sub_831A8130(ctx, base);
	// 82F35260: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35264: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F35268: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F3526C: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F35270: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F35274: 38CA3730  addi r6, r10, 0x3730
	ctx.r[6].s64 = ctx.r[10].s64 + 14128;
	// 82F35278: 38A93778  addi r5, r9, 0x3778
	ctx.r[5].s64 = ctx.r[9].s64 + 14200;
	// 82F3527C: 388837C8  addi r4, r8, 0x37c8
	ctx.r[4].s64 = ctx.r[8].s64 + 14280;
	// 82F35280: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F35284: 38EB5BD8  addi r7, r11, 0x5bd8
	ctx.r[7].s64 = ctx.r[11].s64 + 23512;
	// 82F35288: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3528C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82F35290: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F35294: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F35298: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3529C: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82F352A0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F352A4: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F352A8: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82F352AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F352B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F352B4: 4BFF018D  bl 0x82f25440
	ctx.lr = 0x82F352B8;
	sub_82F25440(ctx, base);
	// 82F352B8: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F352BC: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F352C0: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F352C4: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F352C8: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F352CC: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F352D0: 39035808  addi r8, r3, 0x5808
	ctx.r[8].s64 = ctx.r[3].s64 + 22536;
	// 82F352D4: 38CA7758  addi r6, r10, 0x7758
	ctx.r[6].s64 = ctx.r[10].s64 + 30552;
	// 82F352D8: 38A94FF8  addi r5, r9, 0x4ff8
	ctx.r[5].s64 = ctx.r[9].s64 + 20472;
	// 82F352DC: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F352E0: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 82F352E4: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F352E8: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F352EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F352F0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F352F4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82F352F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F352FC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F35300: 4BFF0141  bl 0x82f25440
	ctx.lr = 0x82F35304;
	sub_82F25440(ctx, base);
	// 82F35304: 3C8082F3  lis r4, -0x7d0d
	ctx.r[4].s64 = -2098003968;
	// 82F35308: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F3530C: 9BE100A0  stb r31, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u8 ) };
	// 82F35310: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F35314: 9BC100A1  stb r30, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[30].u8 ) };
	// 82F35318: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F3531C: 392451A8  addi r9, r4, 0x51a8
	ctx.r[9].s64 = ctx.r[4].s64 + 20904;
	// 82F35320: 390379A8  addi r8, r3, 0x79a8
	ctx.r[8].s64 = ctx.r[3].s64 + 31144;
	// 82F35324: 38AA4FF8  addi r5, r10, 0x4ff8
	ctx.r[5].s64 = ctx.r[10].s64 + 20472;
	// 82F35328: 91210090  stw r9, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 82F3532C: 38EB7758  addi r7, r11, 0x7758
	ctx.r[7].s64 = ctx.r[11].s64 + 30552;
	// 82F35330: 91010094  stw r8, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[8].u32 ) };
	// 82F35334: 90A1009C  stw r5, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[5].u32 ) };
	// 82F35338: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82F3533C: 90E10098  stw r7, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82F35340: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82F35344: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F35348: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F3534C: 4BFF00F5  bl 0x82f25440
	ctx.lr = 0x82F35350;
	sub_82F25440(ctx, base);
	// 82F35350: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F35354: 48272E68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F35358 size=16
    let mut pc: u32 = 0x82F35358;
    'dispatch: loop {
        match pc {
            0x82F35358 => {
    //   block [0x82F35358..0x82F35368)
	// 82F35358: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3535C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F35360: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F35364: 4800059C  b 0x82f35900
	sub_82F35900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35368 size=72
    let mut pc: u32 = 0x82F35368;
    'dispatch: loop {
        match pc {
            0x82F35368 => {
    //   block [0x82F35368..0x82F353B0)
	// 82F35368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3536C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F35370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35374: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F35378: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3537C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F35380: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F35384: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F35388: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3538C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F35390: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F35394: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F35398: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3539C: 4800299D  bl 0x82f37d38
	ctx.lr = 0x82F353A0;
	sub_82F37D38(ctx, base);
	// 82F353A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F353A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F353A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F353AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F353B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F353B0 size=76
    let mut pc: u32 = 0x82F353B0;
    'dispatch: loop {
        match pc {
            0x82F353B0 => {
    //   block [0x82F353B0..0x82F353FC)
	// 82F353B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F353B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F353B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F353BC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F353C0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F353C4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F353C8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F353CC: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F353D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F353D4: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F353D8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F353DC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F353E0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F353E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F353E8: 48002939  bl 0x82f37d20
	ctx.lr = 0x82F353EC;
	sub_82F37D20(ctx, base);
	// 82F353EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F353F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F353F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F353F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F35400 size=176
    let mut pc: u32 = 0x82F35400;
    'dispatch: loop {
        match pc {
            0x82F35400 => {
    //   block [0x82F35400..0x82F354B0)
	// 82F35400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F35408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3540C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F35410: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F35414: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35418: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3541C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F35420: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F35424: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F35428: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3542C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F35430: 48001339  bl 0x82f36768
	ctx.lr = 0x82F35434;
	sub_82F36768(ctx, base);
	// 82F35434: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35438: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F3543C: 40980044  bge cr6, 0x82f35480
	if !ctx.cr[6].lt {
	pc = 0x82F35480; continue 'dispatch;
	}
	// 82F35440: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F35444: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F35448: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F354B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F354B0 size=224
    let mut pc: u32 = 0x82F354B0;
    'dispatch: loop {
        match pc {
            0x82F354B0 => {
    //   block [0x82F354B0..0x82F35590)
	// 82F354B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F354B4: 48272CB5  bl 0x831a8168
	ctx.lr = 0x82F354B8;
	sub_831A8130(ctx, base);
	// 82F354B8: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F354BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F354C0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F354C4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F354C8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F354CC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F354D0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F354D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F354D8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F354DC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F354E0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F354E4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F354E8: 4200FFF0  bdnz 0x82f354d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F354D8; continue 'dispatch;
	}
	// 82F354EC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F354F0: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F354F4: E9060058  ld r8, 0x58(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F354F8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F354FC: 38CAC5B0  addi r6, r10, -0x3a50
	ctx.r[6].s64 = ctx.r[10].s64 + -14928;
	// 82F35500: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F35504: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F35508: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35590 size=100
    let mut pc: u32 = 0x82F35590;
    'dispatch: loop {
        match pc {
            0x82F35590 => {
    //   block [0x82F35590..0x82F355F4)
	// 82F35590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F35598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3559C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F355A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F355A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F355A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F355AC: 4BFFF995  bl 0x82f34f40
	ctx.lr = 0x82F355B0;
	sub_82F34F40(ctx, base);
	// 82F355B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F355B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F355B8: 419A0020  beq cr6, 0x82f355d8
	if ctx.cr[6].eq {
	pc = 0x82F355D8; continue 'dispatch;
	}
	// 82F355BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F355C0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F355C4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F355C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F355CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F355D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F355D4: 4BF6B1DD  bl 0x82ea07b0
	ctx.lr = 0x82F355D8;
	sub_82EA07B0(ctx, base);
	// 82F355D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F355DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F355E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F355E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F355E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F355EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F355F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F355F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F355F8 size=16
    let mut pc: u32 = 0x82F355F8;
    'dispatch: loop {
        match pc {
            0x82F355F8 => {
    //   block [0x82F355F8..0x82F35608)
	// 82F355F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82F355FC: 896B09E0  lbz r11, 0x9e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2528 as u32) ) } as u64;
	// 82F35600: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F35604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F35608 size=12
    let mut pc: u32 = 0x82F35608;
    'dispatch: loop {
        match pc {
            0x82F35608 => {
    //   block [0x82F35608..0x82F35614)
	// 82F35608: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82F3560C: 986B09E0  stb r3, 0x9e0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2528 as u32), ctx.r[3].u8 ) };
	// 82F35610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35618 size=140
    let mut pc: u32 = 0x82F35618;
    'dispatch: loop {
        match pc {
            0x82F35618 => {
    //   block [0x82F35618..0x82F356A4)
	// 82F35618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3561C: 48272B4D  bl 0x831a8168
	ctx.lr = 0x82F35620;
	sub_831A8130(ctx, base);
	// 82F35620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F35628: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3562C: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82F35630: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F35634: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35638: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F3563C: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82F35640: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F35644: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F35648: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F3564C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F35650: 419A0034  beq cr6, 0x82f35684
	if ctx.cr[6].eq {
	pc = 0x82F35684; continue 'dispatch;
	}
	// 82F35654: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F3565C: 419A001C  beq cr6, 0x82f35678
	if ctx.cr[6].eq {
	pc = 0x82F35678; continue 'dispatch;
	}
	// 82F35660: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F35664: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F35668: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3566C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F35670: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35674: 4E800421  bctrl
	ctx.lr = 0x82F35678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35678: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82F3567C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F35680: 409AFFD4  bne cr6, 0x82f35654
	if !ctx.cr[6].eq {
	pc = 0x82F35654; continue 'dispatch;
	}
	// 82F35684: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35688: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3568C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F35690: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35694: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35698: 4E800421  bctrl
	ctx.lr = 0x82F3569C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3569C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F356A0: 48272B18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F356A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F356A8 size=112
    let mut pc: u32 = 0x82F356A8;
    'dispatch: loop {
        match pc {
            0x82F356A8 => {
    //   block [0x82F356A8..0x82F35718)
	// 82F356A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F356AC: 48272AC1  bl 0x831a816c
	ctx.lr = 0x82F356B0;
	sub_831A8130(ctx, base);
	// 82F356B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F356B4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F356B8: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82F356BC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F356C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F356C4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F356C8: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82F356CC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F356D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F356D4: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F356D8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F356DC: 419A0034  beq cr6, 0x82f35710
	if ctx.cr[6].eq {
	pc = 0x82F35710; continue 'dispatch;
	}
	// 82F356E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F356E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F356E8: 419A001C  beq cr6, 0x82f35704
	if ctx.cr[6].eq {
	pc = 0x82F35704; continue 'dispatch;
	}
	// 82F356EC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F356F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F356F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F356F8: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F356FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35700: 4E800421  bctrl
	ctx.lr = 0x82F35704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35704: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82F35708: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F3570C: 409AFFD4  bne cr6, 0x82f356e0
	if !ctx.cr[6].eq {
	pc = 0x82F356E0; continue 'dispatch;
	}
	// 82F35710: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F35714: 48272AA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35718 size=144
    let mut pc: u32 = 0x82F35718;
    'dispatch: loop {
        match pc {
            0x82F35718 => {
    //   block [0x82F35718..0x82F357A8)
	// 82F35718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3571C: 48272A51  bl 0x831a816c
	ctx.lr = 0x82F35720;
	sub_831A8130(ctx, base);
	// 82F35720: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82F35724: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F35728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3572C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F35730: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82F35734: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35738: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F3573C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F35740: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F35744: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82F35748: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F3574C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F35750: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F35754: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F35758: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F3575C: 419A003C  beq cr6, 0x82f35798
	if ctx.cr[6].eq {
	pc = 0x82F35798; continue 'dispatch;
	}
	// 82F35760: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35764: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F35768: 419A0024  beq cr6, 0x82f3578c
	if ctx.cr[6].eq {
	pc = 0x82F3578C; continue 'dispatch;
	}
	// 82F3576C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F35770: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82F35774: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F35778: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F3577C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35780: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F35784: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35788: 4E800421  bctrl
	ctx.lr = 0x82F3578C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3578C: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82F35790: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F35794: 409AFFCC  bne cr6, 0x82f35760
	if !ctx.cr[6].eq {
	pc = 0x82F35760; continue 'dispatch;
	}
	// 82F35798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3579C: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82F357A0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82F357A4: 48272A18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F357A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F357A8 size=96
    let mut pc: u32 = 0x82F357A8;
    'dispatch: loop {
        match pc {
            0x82F357A8 => {
    //   block [0x82F357A8..0x82F35808)
	// 82F357A8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F357AC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82F357B0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F357B4: 390AF4FC  addi r8, r10, -0xb04
	ctx.r[8].s64 = ctx.r[10].s64 + -2820;
	// 82F357B8: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82F357BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F357C0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F357C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F357C8: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82F357CC: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82F357D0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F357D4: C009BA78  lfs f0, -0x4588(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F357D8: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82F357DC: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F357E0: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82F357E4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F35808 size=164
    let mut pc: u32 = 0x82F35808;
    'dispatch: loop {
        match pc {
            0x82F35808 => {
    //   block [0x82F35808..0x82F358AC)
	// 82F35808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3580C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F35810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F35814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35818: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3581C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F35820: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F35824: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F35828: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F3582C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F35830: 4BF6AF01  bl 0x82ea0730
	ctx.lr = 0x82F35834;
	sub_82EA0730(ctx, base);
	// 82F35834: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F35838: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F3583C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F35840: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82F35844: 38E9F4FC  addi r7, r9, -0xb04
	ctx.r[7].s64 = ctx.r[9].s64 + -2820;
	// 82F35848: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82F3584C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F35850: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F35854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F35858: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82F3585C: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 82F35860: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82F35864: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F35868: C008BA78  lfs f0, -0x4588(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3586C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82F35870: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F35874: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82F35878: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F358B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F358B0 size=80
    let mut pc: u32 = 0x82F358B0;
    'dispatch: loop {
        match pc {
            0x82F358B0 => {
    //   block [0x82F358B0..0x82F35900)
	// 82F358B0: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F358B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F358B8: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F358BC: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F358C0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F358C4: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F358C8: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82F358CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F358D0: 409A0008  bne cr6, 0x82f358d8
	if !ctx.cr[6].eq {
	pc = 0x82F358D8; continue 'dispatch;
	}
	// 82F358D4: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	// 82F358D8: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F358DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F358E0: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F358E4: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F358E8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F358EC: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F358F0: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F358F4: 810909A0  lwz r8, 0x9a0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F358F8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F358FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35900 size=480
    let mut pc: u32 = 0x82F35900;
    'dispatch: loop {
        match pc {
            0x82F35900 => {
    //   block [0x82F35900..0x82F35AE0)
	// 82F35900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35904: 48272851  bl 0x831a8154
	ctx.lr = 0x82F35908;
	sub_831A8130(ctx, base);
	// 82F35908: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3590C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F35910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F35914: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F35918: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3591C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F35920: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35924: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35928: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3592C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35930: 4E800421  bctrl
	ctx.lr = 0x82F35934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35934: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F35938: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F3593C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82F35940: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F35944: 40990194  ble cr6, 0x82f35ad8
	if !ctx.cr[6].gt {
	pc = 0x82F35AD8; continue 'dispatch;
	}
	// 82F35948: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F3594C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35950: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F35954: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35958: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F3595C: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F35960: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F35964: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F35968: 4E800421  bctrl
	ctx.lr = 0x82F3596C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3596C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35970: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35974: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F35978: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F3597C: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82F35980: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35984: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82F35988: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F3598C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F35990: 90A10068  stw r5, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[5].u32 ) };
	// 82F35994: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F35998: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3599C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F359A0: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82F359A4: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82F359A8: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F359AC: 836A0000  lwz r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F359B0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F359B4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F359B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F359BC: 4E800421  bctrl
	ctx.lr = 0x82F359C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F359C0: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F359C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F359C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F359CC: 7F7E5A14  add r27, r30, r11
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F359D0: 419A00B4  beq cr6, 0x82f35a84
	if ctx.cr[6].eq {
	pc = 0x82F35A84; continue 'dispatch;
	}
	// 82F359D4: 4BFF082D  bl 0x82f26200
	ctx.lr = 0x82F359D8;
	sub_82F26200(ctx, base);
	// 82F359D8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F359DC: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82F359E0: 409A0074  bne cr6, 0x82f35a54
	if !ctx.cr[6].eq {
	pc = 0x82F35A54; continue 'dispatch;
	}
	// 82F359E4: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F359E8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F359EC: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F359F0: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F359F4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F359F8: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F359FC: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35A00: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82F35A04: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35A08: 409A0008  bne cr6, 0x82f35a10
	if !ctx.cr[6].eq {
	pc = 0x82F35A10; continue 'dispatch;
	}
	// 82F35A0C: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	// 82F35A10: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F35A14: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F35A18: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F35A1C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F35A20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F35A24: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F35A28: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F35A2C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F35A30: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F35A34: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F35A38: 810909A0  lwz r8, 0x9a0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F35A3C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F35A40: 4E800421  bctrl
	ctx.lr = 0x82F35A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35A44: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35A48: 7CFE5A14  add r7, r30, r11
	ctx.r[7].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F35A4C: 90670008  stw r3, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82F35A50: 48000074  b 0x82f35ac4
	pc = 0x82F35AC4; continue 'dispatch;
	// 82F35A54: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35A58: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F35A5C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F35A60: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F35A64: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F35A68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F35A6C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35A70: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35A74: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F35A78: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F35A7C: 4E800421  bctrl
	ctx.lr = 0x82F35A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35A80: 48000044  b 0x82f35ac4
	pc = 0x82F35AC4; continue 'dispatch;
	// 82F35A84: 4BFF077D  bl 0x82f26200
	ctx.lr = 0x82F35A88;
	sub_82F26200(ctx, base);
	// 82F35A88: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35A8C: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82F35A90: 419A0034  beq cr6, 0x82f35ac4
	if ctx.cr[6].eq {
	pc = 0x82F35AC4; continue 'dispatch;
	}
	// 82F35A94: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35A98: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82F35A9C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F35AA0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35AA4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35AA8: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F35AAC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F35AB0: 4E800421  bctrl
	ctx.lr = 0x82F35AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35AB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35AB8: 7F7E5A14  add r27, r30, r11
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F35ABC: 4BFF0745  bl 0x82f26200
	ctx.lr = 0x82F35AC0;
	sub_82F26200(ctx, base);
	// 82F35AC0: 907B0008  stw r3, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82F35AC4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F35AC8: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82F35ACC: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82F35AD0: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F35AD4: 4198FE78  blt cr6, 0x82f3594c
	if ctx.cr[6].lt {
	pc = 0x82F3594C; continue 'dispatch;
	}
	// 82F35AD8: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82F35ADC: 482726C8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F35AE0 size=244
    let mut pc: u32 = 0x82F35AE0;
    'dispatch: loop {
        match pc {
            0x82F35AE0 => {
    //   block [0x82F35AE0..0x82F35BD4)
	// 82F35AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35AE4: 48272685  bl 0x831a8168
	ctx.lr = 0x82F35AE8;
	sub_831A8130(ctx, base);
	// 82F35AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35AEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F35AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F35AF4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F35AF8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F35AFC: 388AF5C0  addi r4, r10, -0xa40
	ctx.r[4].s64 = ctx.r[10].s64 + -2624;
	// 82F35B00: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35B04: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F35B08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F35B0C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F35B10: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F35B14: 4E800421  bctrl
	ctx.lr = 0x82F35B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35B18: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F35B1C: 55480000  rlwinm r8, r10, 0, 0, 0
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82F35B20: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F35B24: 409A0048  bne cr6, 0x82f35b6c
	if !ctx.cr[6].eq {
	pc = 0x82F35B6C; continue 'dispatch;
	}
	// 82F35B28: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F35B2C: 554900BE  clrlwi r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82F35B30: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35B34: 554A087C  rlwinm r10, r10, 1, 1, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82F35B38: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F35B3C: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35B40: 7CA95214  add r5, r9, r10
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F35B44: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F35B48: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F35B4C: 81470008  lwz r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35B50: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F35B54: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F35B58: 388BF5B4  addi r4, r11, -0xa4c
	ctx.r[4].s64 = ctx.r[11].s64 + -2636;
	// 82F35B5C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F35B60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F35B64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35B68: 4E800421  bctrl
	ctx.lr = 0x82F35B6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35B6C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F35B70: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35B74: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F35B78: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82F35B7C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F35B80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F35B84: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F35B88: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F35B8C: 419A0040  beq cr6, 0x82f35bcc
	if ctx.cr[6].eq {
	pc = 0x82F35BCC; continue 'dispatch;
	}
	// 82F35B90: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F35B94: 3B8B487C  addi r28, r11, 0x487c
	ctx.r[28].s64 = ctx.r[11].s64 + 18556;
	// 82F35B98: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35B9C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F35BA0: 419A0020  beq cr6, 0x82f35bc0
	if ctx.cr[6].eq {
	pc = 0x82F35BC0; continue 'dispatch;
	}
	// 82F35BA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35BA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F35BAC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F35BB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F35BB4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F35BB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F35BBC: 4E800421  bctrl
	ctx.lr = 0x82F35BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35BC0: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82F35BC4: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F35BC8: 409AFFD0  bne cr6, 0x82f35b98
	if !ctx.cr[6].eq {
	pc = 0x82F35B98; continue 'dispatch;
	}
	// 82F35BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F35BD0: 482725E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F35BD8 size=176
    let mut pc: u32 = 0x82F35BD8;
    'dispatch: loop {
        match pc {
            0x82F35BD8 => {
    //   block [0x82F35BD8..0x82F35C88)
	// 82F35BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F35BE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F35BE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35BE8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35BEC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F35BF0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F35BF4: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F35BF8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F35BFC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F35C00: 4BF6AB31  bl 0x82ea0730
	ctx.lr = 0x82F35C04;
	sub_82EA0730(ctx, base);
	// 82F35C04: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F35C08: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F35C0C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F35C10: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82F35C14: 38E9F4FC  addi r7, r9, -0xb04
	ctx.r[7].s64 = ctx.r[9].s64 + -2820;
	// 82F35C18: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82F35C1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F35C20: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F35C24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F35C28: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82F35C2C: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 82F35C30: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82F35C34: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F35C38: C008BA78  lfs f0, -0x4588(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F35C3C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82F35C40: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F35C44: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82F35C48: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F35C88 size=324
    let mut pc: u32 = 0x82F35C88;
    'dispatch: loop {
        match pc {
            0x82F35C88 => {
    //   block [0x82F35C88..0x82F35DCC)
	// 82F35C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F35C90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F35C94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35C98: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35C9C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F35CA0: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35CA4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F35CA8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F35CAC: C00B00A0  lfs f0, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F35CB0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35CB4: C1AA00A0  lfs f13, 0xa0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F35CB8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F35CBC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F35CC0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F35CC4: 40980080  bge cr6, 0x82f35d44
	if !ctx.cr[6].lt {
	pc = 0x82F35D44; continue 'dispatch;
	}
	// 82F35CC8: 4BF6AA69  bl 0x82ea0730
	ctx.lr = 0x82F35CCC;
	sub_82EA0730(ctx, base);
	// 82F35CCC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F35CD0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F35CD4: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F35CD8: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82F35CDC: 38E9F4FC  addi r7, r9, -0xb04
	ctx.r[7].s64 = ctx.r[9].s64 + -2820;
	// 82F35CE0: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82F35CE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F35CE8: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F35CEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F35CF0: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82F35CF4: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 82F35CF8: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82F35CFC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F35D00: C008BA78  lfs f0, -0x4588(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F35D04: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F35D08: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82F35D0C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82F35D10: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F35DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F35DD0 size=2452
    let mut pc: u32 = 0x82F35DD0;
    'dispatch: loop {
        match pc {
            0x82F35DD0 => {
    //   block [0x82F35DD0..0x82F36764)
	// 82F35DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F35DD4: 48272369  bl 0x831a813c
	ctx.lr = 0x82F35DD8;
	sub_831A8130(ctx, base);
	// 82F35DD8: 9421FA80  stwu r1, -0x580(r1)
	ea = ctx.r[1].u32.wrapping_add(-1408 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F35DDC: 7C922378  mr r18, r4
	ctx.r[18].u64 = ctx.r[4].u64;
	// 82F35DE0: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 82F35DE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F35DE8: 386104C0  addi r3, r1, 0x4c0
	ctx.r[3].s64 = ctx.r[1].s64 + 1216;
	// 82F35DEC: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82F35DF0: 80B20008  lwz r5, 8(r18)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35DF4: 7CF33B78  mr r19, r7
	ctx.r[19].u64 = ctx.r[7].u64;
	// 82F35DF8: 80910008  lwz r4, 8(r17)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F35DFC: 4BF6B265  bl 0x82ea1060
	ctx.lr = 0x82F35E00;
	sub_82EA1060(ctx, base);
	// 82F35E00: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35E04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F35E08: C1B40004  lfs f13, 4(r20)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F35E0C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F35E10: 388104C0  addi r4, r1, 0x4c0
	ctx.r[4].s64 = ctx.r[1].s64 + 1216;
	// 82F35E14: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F35E18: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F35E1C: EC2D683A  fmadds f1, f13, f0, f13
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82F35E20: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F35E24: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F35E28: 4E800421  bctrl
	ctx.lr = 0x82F35E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F35E2C: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 82F35E30: 896809E0  lbz r11, 0x9e0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(2528 as u32) ) } as u64;
	// 82F35E34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F35E38: 419A0064  beq cr6, 0x82f35e9c
	if ctx.cr[6].eq {
	pc = 0x82F35E9C; continue 'dispatch;
	}
	// 82F35E3C: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82F35E40: 117F038C  vspltisw v11, -1
	for i in 0..4 {
		ctx.v[11].u32[i] = 4294967295;
	}
	// 82F35E44: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F35E48: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82F35E4C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F35E50: 3CE08338  lis r7, -0x7cc8
	ctx.r[7].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F36768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F36768 size=3500
    let mut pc: u32 = 0x82F36768;
    'dispatch: loop {
        match pc {
            0x82F36768 => {
    //   block [0x82F36768..0x82F37514)
	// 82F36768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3676C: 482719C5  bl 0x831a8130
	ctx.lr = 0x82F36770;
	sub_831A8130(ctx, base);
	// 82F36770: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82F36774: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82F36778: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F37518 size=576
    let mut pc: u32 = 0x82F37518;
    'dispatch: loop {
        match pc {
            0x82F37518 => {
    //   block [0x82F37518..0x82F37758)
	// 82F37518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3751C: 48270C31  bl 0x831a814c
	ctx.lr = 0x82F37520;
	sub_831A8130(ctx, base);
	// 82F37520: 9421FB00  stwu r1, -0x500(r1)
	ea = ctx.r[1].u32.wrapping_add(-1280 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37524: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37528: 3AC00018  li r22, 0x18
	ctx.r[22].s64 = 24;
	// 82F3752C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F37530: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F37534: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F37538: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F3753C: 7D75B02E  lwzx r11, r21, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F37540: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F37544: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37548: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3754C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F37550: 4098002C  bge cr6, 0x82f3757c
	if !ctx.cr[6].lt {
	pc = 0x82F3757C; continue 'dispatch;
	}
	// 82F37554: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F37558: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3755C: 38E9F618  addi r7, r9, -0x9e8
	ctx.r[7].s64 = ctx.r[9].s64 + -2536;
	// 82F37560: 38C8F468  addi r6, r8, -0xb98
	ctx.r[6].s64 = ctx.r[8].s64 + -2968;
	// 82F37564: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F37568: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F3756C: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F37570: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82F37574: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37578: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F3757C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F37580: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F37584: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F37588: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F3758C: 480007C5  bl 0x82f37d50
	ctx.lr = 0x82F37590;
	sub_82F37D50(ctx, base);
	// 82F37590: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82F37594: 3941009C  addi r10, r1, 0x9c
	ctx.r[10].s64 = ctx.r[1].s64 + 156;
	// 82F37598: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3759C: 61680080  ori r8, r11, 0x80
	ctx.r[8].u64 = ctx.r[11].u64 | 128;
	// 82F375A0: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82F375A4: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82F375A8: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F375AC: 91010098  stw r8, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[8].u32 ) };
	// 82F375B0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F375B4: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F375B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F375BC: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F375C0: 80C7002C  lwz r6, 0x2c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F375C4: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F375C8: 4E800421  bctrl
	ctx.lr = 0x82F375CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F375CC: 7D75B02E  lwzx r11, r21, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F375D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F375D4: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F375D8: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82F375DC: 40980020  bge cr6, 0x82f375fc
	if !ctx.cr[6].lt {
	pc = 0x82F375FC; continue 'dispatch;
	}
	// 82F375E0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F375E4: 3909F608  addi r8, r9, -0x9f8
	ctx.r[8].s64 = ctx.r[9].s64 + -2552;
	// 82F375E8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F375EC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F375F0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F375F4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F375F8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F375FC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37600: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37604: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37608: 81210094  lwz r9, 0x94(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82F3760C: 83E10090  lwz r31, 0x90(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F37610: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37614: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F37618: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82F3761C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82F37620: 7EEBFA14  add r23, r11, r31
	ctx.r[23].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F37624: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37628: 80E80014  lwz r7, 0x14(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3762C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F37630: 4E800421  bctrl
	ctx.lr = 0x82F37634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37634: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F37638: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82F3763C: 419A00BC  beq cr6, 0x82f376f8
	if ctx.cr[6].eq {
	pc = 0x82F376F8; continue 'dispatch;
	}
	// 82F37640: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37644: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82F37648: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3764C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F37650: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F37654: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F37658: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F3765C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37660: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37664: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37668: 4E800421  bctrl
	ctx.lr = 0x82F3766C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3766C: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37670: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F37674: 419A0078  beq cr6, 0x82f376ec
	if ctx.cr[6].eq {
	pc = 0x82F376EC; continue 'dispatch;
	}
	// 82F37678: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3767C: 38A102A0  addi r5, r1, 0x2a0
	ctx.r[5].s64 = ctx.r[1].s64 + 672;
	// 82F37680: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37688: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3768C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37690: 4E800421  bctrl
	ctx.lr = 0x82F37694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37694: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F3769C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F376A0: 393B000D  addi r9, r27, 0xd
	ctx.r[9].s64 = ctx.r[27].s64 + 13;
	// 82F376A4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F376A8: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F376AC: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F376B0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82F376B4: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82F376B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F376BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F376C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F376C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F376C8: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F376CC: 7D6850AE  lbzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F376D0: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F376D4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F376D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F376DC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F376E0: 812A09AC  lwz r9, 0x9ac(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82F376E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F376E8: 4E800421  bctrl
	ctx.lr = 0x82F376EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F376EC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F376F0: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82F376F4: 409AFF4C  bne cr6, 0x82f37640
	if !ctx.cr[6].eq {
	pc = 0x82F37640; continue 'dispatch;
	}
	// 82F376F8: 7D55B02E  lwzx r10, r21, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F376FC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37700: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37704: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F37708: 40980020  bge cr6, 0x82f37728
	if !ctx.cr[6].lt {
	pc = 0x82F37728; continue 'dispatch;
	}
	// 82F3770C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F37710: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F37714: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F37718: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3771C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F37720: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F37724: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37728: 81610098  lwz r11, 0x98(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82F3772C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F37730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F37734: 409A001C  bne cr6, 0x82f37750
	if !ctx.cr[6].eq {
	pc = 0x82F37750; continue 'dispatch;
	}
	// 82F37738: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3773C: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F37740: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F37744: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F37748: 7C75502E  lwzx r3, r21, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F3774C: 4BF69065  bl 0x82ea07b0
	ctx.lr = 0x82F37750;
	sub_82EA07B0(ctx, base);
	// 82F37750: 38210500  addi r1, r1, 0x500
	ctx.r[1].s64 = ctx.r[1].s64 + 1280;
	// 82F37754: 48270A48  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F37758 size=592
    let mut pc: u32 = 0x82F37758;
    'dispatch: loop {
        match pc {
            0x82F37758 => {
    //   block [0x82F37758..0x82F379A8)
	// 82F37758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3775C: 482709F5  bl 0x831a8150
	ctx.lr = 0x82F37760;
	sub_831A8130(ctx, base);
	// 82F37760: 9421FAC0  stwu r1, -0x540(r1)
	ea = ctx.r[1].u32.wrapping_add(-1344 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37764: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37768: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F3776C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F37770: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F37774: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F37778: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F3777C: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F37780: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37784: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37788: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3778C: 4098002C  bge cr6, 0x82f377b8
	if !ctx.cr[6].lt {
	pc = 0x82F377B8; continue 'dispatch;
	}
	// 82F37790: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F37794: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F37798: 38E9F618  addi r7, r9, -0x9e8
	ctx.r[7].s64 = ctx.r[9].s64 + -2536;
	// 82F3779C: 38C8F468  addi r6, r8, -0xb98
	ctx.r[6].s64 = ctx.r[8].s64 + -2968;
	// 82F377A0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F377A4: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F377A8: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F377AC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F377B0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F377B4: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F377B8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82F377BC: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F377C0: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F377C4: 4BF6989D  bl 0x82ea1060
	ctx.lr = 0x82F377C8;
	sub_82EA1060(ctx, base);
	// 82F377C8: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F377CC: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F377D0: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F377D4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F377D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F377DC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F377E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F377E4: 4E800421  bctrl
	ctx.lr = 0x82F377E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F377E8: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82F377EC: 390100DC  addi r8, r1, 0xdc
	ctx.r[8].s64 = ctx.r[1].s64 + 220;
	// 82F377F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F377F4: 61260080  ori r6, r9, 0x80
	ctx.r[6].u64 = ctx.r[9].u64 | 128;
	// 82F377F8: 910100D0  stw r8, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[8].u32 ) };
	// 82F377FC: 90E100D4  stw r7, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[7].u32 ) };
	// 82F37800: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 82F37804: 90C100D8  stw r6, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u32 ) };
	// 82F37808: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3780C: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37810: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37814: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37818: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F3781C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37820: 4E800421  bctrl
	ctx.lr = 0x82F37824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37824: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F37828: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3782C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37830: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F37834: 40980020  bge cr6, 0x82f37854
	if !ctx.cr[6].lt {
	pc = 0x82F37854; continue 'dispatch;
	}
	// 82F37838: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3783C: 3909F608  addi r8, r9, -0x9f8
	ctx.r[8].s64 = ctx.r[9].s64 + -2552;
	// 82F37840: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F37844: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F37848: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3784C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F37850: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37854: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37858: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F3785C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37860: 812100D4  lwz r9, 0xd4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82F37864: 83E100D0  lwz r31, 0xd0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82F37868: 834B000C  lwz r26, 0xc(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3786C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F37870: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82F37874: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82F37878: 7F0BFA14  add r24, r11, r31
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F3787C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37880: 80E80014  lwz r7, 0x14(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F37884: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F37888: 4E800421  bctrl
	ctx.lr = 0x82F3788C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3788C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F37890: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82F37894: 419A00B4  beq cr6, 0x82f37948
	if ctx.cr[6].eq {
	pc = 0x82F37948; continue 'dispatch;
	}
	// 82F37898: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3789C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82F378A0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378A4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82F378A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F378AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F378B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F378B4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F378BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F378C0: 4E800421  bctrl
	ctx.lr = 0x82F378C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F378C4: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F378CC: 419A0070  beq cr6, 0x82f3793c
	if ctx.cr[6].eq {
	pc = 0x82F3793C; continue 'dispatch;
	}
	// 82F378D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378D4: 38A102E0  addi r5, r1, 0x2e0
	ctx.r[5].s64 = ctx.r[1].s64 + 736;
	// 82F378D8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F378E0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F378E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F378E8: 4E800421  bctrl
	ctx.lr = 0x82F378EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F378EC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378F0: 38FA000D  addi r7, r26, 0xd
	ctx.r[7].s64 = ctx.r[26].s64 + 13;
	// 82F378F4: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F378F8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F378FC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82F37900: 54E92834  slwi r9, r7, 5
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F37904: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F37908: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82F3790C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37910: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F37914: 7D6450AE  lbzx r11, r4, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F37918: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F3791C: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F37920: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F37924: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F37928: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F3792C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F37930: 814B09A8  lwz r10, 0x9a8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82F37934: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37938: 4E800421  bctrl
	ctx.lr = 0x82F3793C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3793C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F37940: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82F37944: 409AFF54  bne cr6, 0x82f37898
	if !ctx.cr[6].eq {
	pc = 0x82F37898; continue 'dispatch;
	}
	// 82F37948: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F3794C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37950: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37954: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F37958: 40980020  bge cr6, 0x82f37978
	if !ctx.cr[6].lt {
	pc = 0x82F37978; continue 'dispatch;
	}
	// 82F3795C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F37960: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F37964: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F37968: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3796C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F37970: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F37974: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37978: 816100D8  lwz r11, 0xd8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F3797C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F37980: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F37984: 409A001C  bne cr6, 0x82f379a0
	if !ctx.cr[6].eq {
	pc = 0x82F379A0; continue 'dispatch;
	}
	// 82F37988: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3798C: 808100D0  lwz r4, 0xd0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82F37990: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F37994: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F37998: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F3799C: 4BF68E15  bl 0x82ea07b0
	ctx.lr = 0x82F379A0;
	sub_82EA07B0(ctx, base);
	// 82F379A0: 38210540  addi r1, r1, 0x540
	ctx.r[1].s64 = ctx.r[1].s64 + 1344;
	// 82F379A4: 482707FC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F379A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F379A8 size=604
    let mut pc: u32 = 0x82F379A8;
    'dispatch: loop {
        match pc {
            0x82F379A8 => {
    //   block [0x82F379A8..0x82F37C04)
	// 82F379A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F379AC: 482707A5  bl 0x831a8150
	ctx.lr = 0x82F379B0;
	sub_831A8130(ctx, base);
	// 82F379B0: 9421FAC0  stwu r1, -0x540(r1)
	ea = ctx.r[1].u32.wrapping_add(-1344 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F379B4: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F379B8: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F379BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F379C0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F379C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F379C8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F379CC: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F379D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F379D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F379D8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F379DC: 4098002C  bge cr6, 0x82f37a08
	if !ctx.cr[6].lt {
	pc = 0x82F37A08; continue 'dispatch;
	}
	// 82F379E0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F379E4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F379E8: 38E9F618  addi r7, r9, -0x9e8
	ctx.r[7].s64 = ctx.r[9].s64 + -2536;
	// 82F379EC: 38C8F468  addi r6, r8, -0xb98
	ctx.r[6].s64 = ctx.r[8].s64 + -2968;
	// 82F379F0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F379F4: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F379F8: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F379FC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F37A00: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37A04: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F37A08: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82F37A0C: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37A10: 809A0008  lwz r4, 8(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37A14: 4BF6964D  bl 0x82ea1060
	ctx.lr = 0x82F37A18;
	sub_82EA1060(ctx, base);
	// 82F37A18: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37A1C: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F37A20: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F37A24: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F37A28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37A2C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F37A30: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37A34: 4E800421  bctrl
	ctx.lr = 0x82F37A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37A38: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82F37A3C: 390100DC  addi r8, r1, 0xdc
	ctx.r[8].s64 = ctx.r[1].s64 + 220;
	// 82F37A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F37A44: 61260080  ori r6, r9, 0x80
	ctx.r[6].u64 = ctx.r[9].u64 | 128;
	// 82F37A48: 910100D0  stw r8, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[8].u32 ) };
	// 82F37A4C: 90E100D4  stw r7, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[7].u32 ) };
	// 82F37A50: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 82F37A54: 90C100D8  stw r6, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u32 ) };
	// 82F37A58: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F37A5C: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37A60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37A64: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37A68: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F37A6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37A70: 4E800421  bctrl
	ctx.lr = 0x82F37A74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37A74: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F37A78: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37A7C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37A80: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F37A84: 40980020  bge cr6, 0x82f37aa4
	if !ctx.cr[6].lt {
	pc = 0x82F37AA4; continue 'dispatch;
	}
	// 82F37A88: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F37A8C: 3909F608  addi r8, r9, -0x9f8
	ctx.r[8].s64 = ctx.r[9].s64 + -2552;
	// 82F37A90: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F37A94: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F37A98: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F37A9C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F37AA0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37AA4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37AA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37AAC: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37AB0: 812100D4  lwz r9, 0xd4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82F37AB4: 83E100D0  lwz r31, 0xd0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82F37AB8: 832B000C  lwz r25, 0xc(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37ABC: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F37AC0: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82F37AC4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82F37AC8: 7F0BFA14  add r24, r11, r31
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F37ACC: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37AD0: 80E80014  lwz r7, 0x14(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F37AD4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F37AD8: 4E800421  bctrl
	ctx.lr = 0x82F37ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37ADC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F37AE0: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82F37AE4: 419A00C0  beq cr6, 0x82f37ba4
	if ctx.cr[6].eq {
	pc = 0x82F37BA4; continue 'dispatch;
	}
	// 82F37AE8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37AEC: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82F37AF0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37AF4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F37AF8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F37AFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F37B00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F37B04: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37B08: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37B0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37B10: 4E800421  bctrl
	ctx.lr = 0x82F37B14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37B14: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37B18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F37B1C: 419A007C  beq cr6, 0x82f37b98
	if ctx.cr[6].eq {
	pc = 0x82F37B98; continue 'dispatch;
	}
	// 82F37B20: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37B24: 38A102E0  addi r5, r1, 0x2e0
	ctx.r[5].s64 = ctx.r[1].s64 + 736;
	// 82F37B28: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37B2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37B30: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F37B34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37B38: 4E800421  bctrl
	ctx.lr = 0x82F37B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37B3C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37B40: 38F9000D  addi r7, r25, 0xd
	ctx.r[7].s64 = ctx.r[25].s64 + 13;
	// 82F37B44: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F37B48: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37B4C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F37B50: 54E92834  slwi r9, r7, 5
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F37B54: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F37B58: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82F37B5C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37B60: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F37B64: 7D6450AE  lbzx r11, r4, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F37B68: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F37B6C: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F37B70: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F37B74: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F37B78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F37B7C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F37B80: 814B09A4  lwz r10, 0x9a4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82F37B84: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37B88: 4E800421  bctrl
	ctx.lr = 0x82F37B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37B8C: 893B0004  lbz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37B90: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F37B94: 409A0010  bne cr6, 0x82f37ba4
	if !ctx.cr[6].eq {
	pc = 0x82F37BA4; continue 'dispatch;
	}
	// 82F37B98: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F37B9C: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82F37BA0: 409AFF48  bne cr6, 0x82f37ae8
	if !ctx.cr[6].eq {
	pc = 0x82F37AE8; continue 'dispatch;
	}
	// 82F37BA4: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F37BA8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37BAC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37BB0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F37BB4: 40980020  bge cr6, 0x82f37bd4
	if !ctx.cr[6].lt {
	pc = 0x82F37BD4; continue 'dispatch;
	}
	// 82F37BB8: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F37BBC: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F37BC0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F37BC4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F37BC8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F37BCC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F37BD0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F37BD4: 816100D8  lwz r11, 0xd8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F37BD8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F37BDC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F37BE0: 409A001C  bne cr6, 0x82f37bfc
	if !ctx.cr[6].eq {
	pc = 0x82F37BFC; continue 'dispatch;
	}
	// 82F37BE4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F37BE8: 808100D0  lwz r4, 0xd0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82F37BEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F37BF0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F37BF4: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F37BF8: 4BF68BB9  bl 0x82ea07b0
	ctx.lr = 0x82F37BFC;
	sub_82EA07B0(ctx, base);
	// 82F37BFC: 38210540  addi r1, r1, 0x540
	ctx.r[1].s64 = ctx.r[1].s64 + 1344;
	// 82F37C00: 482705A0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F37C08 size=256
    let mut pc: u32 = 0x82F37C08;
    'dispatch: loop {
        match pc {
            0x82F37C08 => {
    //   block [0x82F37C08..0x82F37D08)
	// 82F37C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F37C0C: 48270561  bl 0x831a816c
	ctx.lr = 0x82F37C10;
	sub_831A8130(ctx, base);
	// 82F37C10: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37C14: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F37C18: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F37C1C: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F37C20: 3D0082F3  lis r8, -0x7d0d
	ctx.r[8].s64 = -2098003968;
	// 82F37C24: 38CA3730  addi r6, r10, 0x3730
	ctx.r[6].s64 = ctx.r[10].s64 + 14128;
	// 82F37C28: 38A93778  addi r5, r9, 0x3778
	ctx.r[5].s64 = ctx.r[9].s64 + 14200;
	// 82F37C2C: 388849A8  addi r4, r8, 0x49a8
	ctx.r[4].s64 = ctx.r[8].s64 + 18856;
	// 82F37C30: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F37C34: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F37C38: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F37C3C: 38EB5BD8  addi r7, r11, 0x5bd8
	ctx.r[7].s64 = ctx.r[11].s64 + 23512;
	// 82F37C40: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F37C44: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F37C48: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F37C4C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F37C50: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82F37C54: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F37C58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F37C5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F37C60: 4BFED7E1  bl 0x82f25440
	ctx.lr = 0x82F37C64;
	sub_82F25440(ctx, base);
	// 82F37C64: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F37C68: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F37C6C: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F37C70: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F37C74: 3D2082F3  lis r9, -0x7d0d
	ctx.r[9].s64 = -2098003968;
	// 82F37C78: 39035808  addi r8, r3, 0x5808
	ctx.r[8].s64 = ctx.r[3].s64 + 22536;
	// 82F37C7C: 38CA7758  addi r6, r10, 0x7758
	ctx.r[6].s64 = ctx.r[10].s64 + 30552;
	// 82F37C80: 38A97518  addi r5, r9, 0x7518
	ctx.r[5].s64 = ctx.r[9].s64 + 29976;
	// 82F37C84: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F37C88: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 82F37C8C: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F37C90: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F37C94: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F37C98: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F37C9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37CA0: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82F37CA4: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82F37CA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F37CAC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F37CB0: 4BFED791  bl 0x82f25440
	ctx.lr = 0x82F37CB4;
	sub_82F25440(ctx, base);
	// 82F37CB4: 3C8082F3  lis r4, -0x7d0d
	ctx.r[4].s64 = -2098003968;
	// 82F37CB8: 3C6082F3  lis r3, -0x7d0d
	ctx.r[3].s64 = -2098003968;
	// 82F37CBC: 9BC100A0  stb r30, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u8 ) };
	// 82F37CC0: 3D6082F3  lis r11, -0x7d0d
	ctx.r[11].s64 = -2098003968;
	// 82F37CC4: 9BE100A1  stb r31, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[31].u8 ) };
	// 82F37CC8: 3D4082F3  lis r10, -0x7d0d
	ctx.r[10].s64 = -2098003968;
	// 82F37CCC: 39245C88  addi r9, r4, 0x5c88
	ctx.r[9].s64 = ctx.r[4].s64 + 23688;
	// 82F37CD0: 390379A8  addi r8, r3, 0x79a8
	ctx.r[8].s64 = ctx.r[3].s64 + 31144;
	// 82F37CD4: 38AA7518  addi r5, r10, 0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + 29976;
	// 82F37CD8: 91210090  stw r9, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 82F37CDC: 38EB7758  addi r7, r11, 0x7758
	ctx.r[7].s64 = ctx.r[11].s64 + 30552;
	// 82F37CE0: 91010094  stw r8, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[8].u32 ) };
	// 82F37CE4: 90A1009C  stw r5, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[5].u32 ) };
	// 82F37CE8: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82F37CEC: 90E10098  stw r7, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82F37CF0: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82F37CF4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F37CF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F37CFC: 4BFED745  bl 0x82f25440
	ctx.lr = 0x82F37D00;
	sub_82F25440(ctx, base);
	// 82F37D00: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F37D04: 482704B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F37D08 size=24
    let mut pc: u32 = 0x82F37D08;
    'dispatch: loop {
        match pc {
            0x82F37D08 => {
    //   block [0x82F37D08..0x82F37D20)
	// 82F37D08: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F37D0C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F37D10: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F37D14: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F37D18: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82F37D1C: 4BFFF7FC  b 0x82f37518
	sub_82F37518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F37D20 size=20
    let mut pc: u32 = 0x82F37D20;
    'dispatch: loop {
        match pc {
            0x82F37D20 => {
    //   block [0x82F37D20..0x82F37D34)
	// 82F37D20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F37D24: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F37D28: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F37D2C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F37D30: 4BFFFA28  b 0x82f37758
	sub_82F37758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F37D38 size=20
    let mut pc: u32 = 0x82F37D38;
    'dispatch: loop {
        match pc {
            0x82F37D38 => {
    //   block [0x82F37D38..0x82F37D4C)
	// 82F37D38: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F37D3C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F37D40: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F37D44: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F37D48: 4BFFFC60  b 0x82f379a8
	sub_82F379A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F37D50 size=200
    let mut pc: u32 = 0x82F37D50;
    'dispatch: loop {
        match pc {
            0x82F37D50 => {
    //   block [0x82F37D50..0x82F37E18)
	// 82F37D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F37D54: 48270415  bl 0x831a8168
	ctx.lr = 0x82F37D58;
	sub_831A8130(ctx, base);
	// 82F37D58: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37D5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F37D60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F37D64: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F37D68: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F37D6C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F37D70: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37D74: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37D78: 4BF692E9  bl 0x82ea1060
	ctx.lr = 0x82F37D7C;
	sub_82EA1060(ctx, base);
	// 82F37D7C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37D80: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F37D84: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F37D88: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F37D8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37D90: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F37D94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37D98: 4E800421  bctrl
	ctx.lr = 0x82F37D9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37D9C: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37DA0: 395E0050  addi r10, r30, 0x50
	ctx.r[10].s64 = ctx.r[30].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F37E18 size=212
    let mut pc: u32 = 0x82F37E18;
    'dispatch: loop {
        match pc {
            0x82F37E18 => {
    //   block [0x82F37E18..0x82F37EEC)
	// 82F37E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F37E1C: 48270351  bl 0x831a816c
	ctx.lr = 0x82F37E20;
	sub_831A8130(ctx, base);
	// 82F37E20: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37E24: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F37E28: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F37E2C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F37E30: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F37E34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F37E38: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F37E3C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F37E40: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F37E44: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F37E48: 4200FFF0  bdnz 0x82f37e38
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F37E38; continue 'dispatch;
	}
	// 82F37E4C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F37E50: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F37E54: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F37E58: E8C60058  ld r6, 0x58(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F37E5C: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F37E60: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 82F37E64: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F37E68: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F37EF0 size=144
    let mut pc: u32 = 0x82F37EF0;
    'dispatch: loop {
        match pc {
            0x82F37EF0 => {
    //   block [0x82F37EF0..0x82F37F80)
	// 82F37EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F37EF4: 48270275  bl 0x831a8168
	ctx.lr = 0x82F37EF8;
	sub_831A8130(ctx, base);
	// 82F37EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37EFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F37F00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F37F04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37F08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F37F0C: 419A0054  beq cr6, 0x82f37f60
	if ctx.cr[6].eq {
	pc = 0x82F37F60; continue 'dispatch;
	}
	// 82F37F10: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F37F14: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F37F18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F37F1C: 40990044  ble cr6, 0x82f37f60
	if !ctx.cr[6].gt {
	pc = 0x82F37F60; continue 'dispatch;
	}
	// 82F37F20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F37F24: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F37F28: 7C8BF22E  lhzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F37F2C: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82F37F30: 419A001C  beq cr6, 0x82f37f4c
	if ctx.cr[6].eq {
	pc = 0x82F37F4C; continue 'dispatch;
	}
	// 82F37F34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37F38: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F37F3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37F40: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F37F44: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37F48: 4E800421  bctrl
	ctx.lr = 0x82F37F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37F4C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F37F50: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F37F54: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82F37F58: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F37F5C: 4198FFC8  blt cr6, 0x82f37f24
	if ctx.cr[6].lt {
	pc = 0x82F37F24; continue 'dispatch;
	}
	// 82F37F60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37F64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F37F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F37F6C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37F70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37F74: 4E800421  bctrl
	ctx.lr = 0x82F37F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F37F7C: 4827023C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F37F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F37F80 size=216
    let mut pc: u32 = 0x82F37F80;
    'dispatch: loop {
        match pc {
            0x82F37F80 => {
    //   block [0x82F37F80..0x82F38058)
	// 82F37F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F37F84: 482701E5  bl 0x831a8168
	ctx.lr = 0x82F37F88;
	sub_831A8130(ctx, base);
	// 82F37F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F37F8C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F37F90: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F37F94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F37F98: 392BF634  addi r9, r11, -0x9cc
	ctx.r[9].s64 = ctx.r[11].s64 + -2508;
	// 82F37F9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F37FA0: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F37FA4: 90FC0008  stw r7, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F37FA8: B15C0006  sth r10, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F37FAC: 3BFC000C  addi r31, r28, 0xc
	ctx.r[31].s64 = ctx.r[28].s64 + 12;
	// 82F37FB0: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F37FB4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F37FB8: 917C000C  stw r11, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F37FBC: 917C0010  stw r11, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F37FC0: 911C0014  stw r8, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F37FC4: 419A0088  beq cr6, 0x82f3804c
	if ctx.cr[6].eq {
	pc = 0x82F3804C; continue 'dispatch;
	}
	// 82F37FC8: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37FCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F37FD0: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F37FD4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F37FD8: 4E800421  bctrl
	ctx.lr = 0x82F37FDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F37FDC: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F37FE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F37FE4: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F37FE8: 40990060  ble cr6, 0x82f38048
	if !ctx.cr[6].gt {
	pc = 0x82F38048; continue 'dispatch;
	}
	// 82F37FEC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F37FF0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F37FF4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F37FF8: 40980024  bge cr6, 0x82f3801c
	if !ctx.cr[6].lt {
	pc = 0x82F3801C; continue 'dispatch;
	}
	// 82F37FFC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F38000: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F38004: 41980008  blt cr6, 0x82f3800c
	if ctx.cr[6].lt {
	pc = 0x82F3800C; continue 'dispatch;
	}
	// 82F38008: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82F3800C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F38010: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F38014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F38018: 4BF6E7E1  bl 0x82ea67f8
	ctx.lr = 0x82F3801C;
	sub_82EA67F8(ctx, base);
	// 82F3801C: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F38020: 40980028  bge cr6, 0x82f38048
	if !ctx.cr[6].lt {
	pc = 0x82F38048; continue 'dispatch;
	}
	// 82F38024: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82F38028: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F3802C: 7D7DF050  subf r11, r29, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82F38030: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82F38034: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38038: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F3803C: 7D2A432E  sthx r9, r10, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u16) };
	// 82F38040: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82F38044: 4082FFF0  bne 0x82f38034
	if !ctx.cr[0].eq {
	pc = 0x82F38034; continue 'dispatch;
	}
	// 82F38048: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82F3804C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F38050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F38054: 48270164  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38058 size=88
    let mut pc: u32 = 0x82F38058;
    'dispatch: loop {
        match pc {
            0x82F38058 => {
    //   block [0x82F38058..0x82F380B0)
	// 82F38058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3805C: 4827010D  bl 0x831a8168
	ctx.lr = 0x82F38060;
	sub_831A8130(ctx, base);
	// 82F38060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38064: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38068: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3806C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F38070: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F38074: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F38078: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82F3807C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F38080: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F38084: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F38088: 4BF686A9  bl 0x82ea0730
	ctx.lr = 0x82F3808C;
	sub_82EA0730(ctx, base);
	// 82F3808C: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82F38090: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F38094: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F38098: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F3809C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F380A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F380A4: 4BFFFEDD  bl 0x82f37f80
	ctx.lr = 0x82F380A8;
	sub_82F37F80(ctx, base);
	// 82F380A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F380AC: 4827010C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F380B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F380B0 size=760
    let mut pc: u32 = 0x82F380B0;
    'dispatch: loop {
        match pc {
            0x82F380B0 => {
    //   block [0x82F380B0..0x82F383A8)
	// 82F380B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F380B4: 48270091  bl 0x831a8144
	ctx.lr = 0x82F380B8;
	sub_831A8130(ctx, base);
	// 82F380B8: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F380BC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F380C0: 3A800018  li r20, 0x18
	ctx.r[20].s64 = 24;
	// 82F380C4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82F380C8: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82F380CC: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82F380D0: 7CD33378  mr r19, r6
	ctx.r[19].u64 = ctx.r[6].u64;
	// 82F380D4: 7D5CA02E  lwzx r10, r28, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82F380D8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F380DC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F380E0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F380E4: 4098002C  bge cr6, 0x82f38110
	if !ctx.cr[6].lt {
	pc = 0x82F38110; continue 'dispatch;
	}
	// 82F380E8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F380EC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F380F0: 38E9F6A4  addi r7, r9, -0x95c
	ctx.r[7].s64 = ctx.r[9].s64 + -2396;
	// 82F380F4: 38C8F694  addi r6, r8, -0x96c
	ctx.r[6].s64 = ctx.r[8].s64 + -2412;
	// 82F380F8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F380FC: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F38100: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F38104: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F38108: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3810C: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F38110: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F38114: 83F70000  lwz r31, 0(r23)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38118: 83360000  lwz r25, 0(r22)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3811C: 80B70008  lwz r5, 8(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F38120: 80960008  lwz r4, 8(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F38124: 4BF68F3D  bl 0x82ea1060
	ctx.lr = 0x82F38128;
	sub_82EA1060(ctx, base);
	// 82F38128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3812C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F38130: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F38134: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F38138: 4E800421  bctrl
	ctx.lr = 0x82F3813C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3813C: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 82F38140: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F38144: 393D0001  addi r9, r29, 1
	ctx.r[9].s64 = ctx.r[29].s64 + 1;
	// 82F38148: 7C7CC02E  lwzx r3, r28, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F3814C: 553E2036  slwi r30, r9, 4
	ctx.r[30].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82F38150: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F38154: 8103002C  lwz r8, 0x2c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F38158: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F3815C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F38160: 41990010  bgt cr6, 0x82f38170
	if ctx.cr[6].gt {
	pc = 0x82F38170; continue 'dispatch;
	}
	// 82F38164: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F38168: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82F3816C: 4800001C  b 0x82f38188
	pc = 0x82F38188; continue 'dispatch;
	// 82F38170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38174: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F38178: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3817C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F38180: 4E800421  bctrl
	ctx.lr = 0x82F38184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F38184: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F38188: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3818C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F38190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F38194: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F38198: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3819C: 4E800421  bctrl
	ctx.lr = 0x82F381A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F381A0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82F381A4: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F381A8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82F381AC: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82F381B0: 3B5DFFFF  addi r26, r29, -1
	ctx.r[26].s64 = ctx.r[29].s64 + -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F383A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F383A8 size=1036
    let mut pc: u32 = 0x82F383A8;
    'dispatch: loop {
        match pc {
            0x82F383A8 => {
    //   block [0x82F383A8..0x82F387B4)
	// 82F383A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F383AC: 4826FD95  bl 0x831a8140
	ctx.lr = 0x82F383B0;
	sub_831A8130(ctx, base);
	// 82F383B0: DBE1FF80  stfd f31, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82F383B4: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F383B8: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F383BC: 3A600018  li r19, 0x18
	ctx.r[19].s64 = 24;
	// 82F383C0: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82F383C4: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82F383C8: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82F383CC: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82F383D0: 7D59982E  lwzx r10, r25, r19
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82F383D4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F383D8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F383DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F383E0: 4098002C  bge cr6, 0x82f3840c
	if !ctx.cr[6].lt {
	pc = 0x82F3840C; continue 'dispatch;
	}
	// 82F383E4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F383E8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F383EC: 38E9F6A4  addi r7, r9, -0x95c
	ctx.r[7].s64 = ctx.r[9].s64 + -2396;
	// 82F383F0: 38C8F6E8  addi r6, r8, -0x918
	ctx.r[6].s64 = ctx.r[8].s64 + -2328;
	// 82F383F4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F383F8: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F383FC: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F38400: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F38404: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F38408: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F3840C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82F38410: 83F50000  lwz r31, 0(r21)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38414: 83770000  lwz r27, 0(r23)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38418: 80B50008  lwz r5, 8(r21)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3841C: 80970008  lwz r4, 8(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F38420: 4BF68C41  bl 0x82ea1060
	ctx.lr = 0x82F38424;
	sub_82EA1060(ctx, base);
	// 82F38424: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3842C: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F38430: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F38434: 4E800421  bctrl
	ctx.lr = 0x82F38438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F38438: 3A400014  li r18, 0x14
	ctx.r[18].s64 = 20;
	// 82F3843C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F38440: 393C0001  addi r9, r28, 1
	ctx.r[9].s64 = ctx.r[28].s64 + 1;
	// 82F38444: 7C79902E  lwzx r3, r25, r18
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82F38448: 553D2036  slwi r29, r9, 4
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82F3844C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F38450: 8103002C  lwz r8, 0x2c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F38454: 7D4BEA14  add r10, r11, r29
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82F38458: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F3845C: 41990010  bgt cr6, 0x82f3846c
	if ctx.cr[6].gt {
	pc = 0x82F3846C; continue 'dispatch;
	}
	// 82F38460: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F38464: 7D765B78  mr r22, r11
	ctx.r[22].u64 = ctx.r[11].u64;
	// 82F38468: 4800001C  b 0x82f38484
	pc = 0x82F38484; continue 'dispatch;
	// 82F3846C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38470: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F38474: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F38478: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3847C: 4E800421  bctrl
	ctx.lr = 0x82F38480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F38480: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82F38484: 7D59982E  lwzx r10, r25, r19
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82F38488: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3848C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F38490: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F38494: 40980020  bge cr6, 0x82f384b4
	if !ctx.cr[6].lt {
	pc = 0x82F384B4; continue 'dispatch;
	}
	// 82F38498: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3849C: 3909F6D8  addi r8, r9, -0x928
	ctx.r[8].s64 = ctx.r[9].s64 + -2344;
	// 82F384A0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F384A4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F384A8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F384AC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F384B0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F384B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F384B8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82F384BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F384C0: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F384C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F384C8: 4E800421  bctrl
	ctx.lr = 0x82F384CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F384CC: 7D79982E  lwzx r11, r25, r19
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82F384D0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F384D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F384D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F384DC: 40980020  bge cr6, 0x82f384fc
	if !ctx.cr[6].lt {
	pc = 0x82F384FC; continue 'dispatch;
	}
	// 82F384E0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F384E4: 3909F6CC  addi r8, r9, -0x934
	ctx.r[8].s64 = ctx.r[9].s64 + -2356;
	// 82F384E8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F384EC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F384F0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F384F4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F384F8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F384FC: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82F38500: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F38504: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F38508: 38E100D0  addi r7, r1, 0xd0
	ctx.r[7].s64 = ctx.r[1].s64 + 208;
	// 82F3850C: 3BDCFFFF  addi r30, r28, -1
	ctx.r[30].s64 = ctx.r[28].s64 + -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F387B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F387B8 size=844
    let mut pc: u32 = 0x82F387B8;
    'dispatch: loop {
        match pc {
            0x82F387B8 => {
    //   block [0x82F387B8..0x82F38B04)
	// 82F387B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F387BC: 4826F985  bl 0x831a8140
	ctx.lr = 0x82F387C0;
	sub_831A8130(ctx, base);
	// 82F387C0: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F38B08 size=24
    let mut pc: u32 = 0x82F38B08;
    'dispatch: loop {
        match pc {
            0x82F38B08 => {
    //   block [0x82F38B08..0x82F38B20)
	// 82F38B08: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F38B0C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F38B10: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F38B14: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F38B18: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82F38B1C: 4BFFFC9C  b 0x82f387b8
	sub_82F387B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F38B20 size=20
    let mut pc: u32 = 0x82F38B20;
    'dispatch: loop {
        match pc {
            0x82F38B20 => {
    //   block [0x82F38B20..0x82F38B34)
	// 82F38B20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F38B24: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F38B28: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F38B2C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F38B30: 4BFFF580  b 0x82f380b0
	sub_82F380B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F38B38 size=20
    let mut pc: u32 = 0x82F38B38;
    'dispatch: loop {
        match pc {
            0x82F38B38 => {
    //   block [0x82F38B38..0x82F38B4C)
	// 82F38B38: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F38B3C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F38B40: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F38B44: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F38B48: 4BFFF860  b 0x82f383a8
	sub_82F383A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38B50 size=108
    let mut pc: u32 = 0x82F38B50;
    'dispatch: loop {
        match pc {
            0x82F38B50 => {
    //   block [0x82F38B50..0x82F38BBC)
	// 82F38B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F38B54: 4826F611  bl 0x831a8164
	ctx.lr = 0x82F38B58;
	sub_831A8130(ctx, base);
	// 82F38B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38B5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38B60: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F38B64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F38B68: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F38B6C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F38B70: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82F38B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F38B78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F38B7C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F38B80: 4BF67BB1  bl 0x82ea0730
	ctx.lr = 0x82F38B84;
	sub_82EA0730(ctx, base);
	// 82F38B84: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F38B88: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82F38B8C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F38B90: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F38B94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F38B98: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F38B9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F38BA0: 4BFFF3E1  bl 0x82f37f80
	ctx.lr = 0x82F38BA4;
	sub_82F37F80(ctx, base);
	// 82F38BA4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F38BA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F38BAC: 38E8F714  addi r7, r8, -0x8ec
	ctx.r[7].s64 = ctx.r[8].s64 + -2284;
	// 82F38BB0: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F38BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F38BB8: 4826F5FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38BC0 size=204
    let mut pc: u32 = 0x82F38BC0;
    'dispatch: loop {
        match pc {
            0x82F38BC0 => {
    //   block [0x82F38BC0..0x82F38C8C)
	// 82F38BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F38BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F38BC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F38BCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F38BD0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38BD4: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F38BD8: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F38BDC: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F38BE0: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F38BE4: 38CA9220  addi r6, r10, -0x6de0
	ctx.r[6].s64 = ctx.r[10].s64 + -28128;
	// 82F38BE8: 38A99268  addi r5, r9, -0x6d98
	ctx.r[5].s64 = ctx.r[9].s64 + -28056;
	// 82F38BEC: 388892B8  addi r4, r8, -0x6d48
	ctx.r[4].s64 = ctx.r[8].s64 + -27976;
	// 82F38BF0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F38BF4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F38BF8: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F38BFC: 38EB8B50  addi r7, r11, -0x74b0
	ctx.r[7].s64 = ctx.r[11].s64 + -29872;
	// 82F38C00: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F38C04: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F38C08: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82F38C0C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F38C10: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82F38C14: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F38C18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F38C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F38C20: 4BFEC821  bl 0x82f25440
	ctx.lr = 0x82F38C24;
	sub_82F25440(ctx, base);
	// 82F38C24: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F38C28: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F38C2C: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F38C30: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F38C34: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F38C38: 39038058  addi r8, r3, -0x7fa8
	ctx.r[8].s64 = ctx.r[3].s64 + -32680;
	// 82F38C3C: 38CA83A8  addi r6, r10, -0x7c58
	ctx.r[6].s64 = ctx.r[10].s64 + -31832;
	// 82F38C40: 38A987B8  addi r5, r9, -0x7848
	ctx.r[5].s64 = ctx.r[9].s64 + -30792;
	// 82F38C44: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F38C48: 38EB80B0  addi r7, r11, -0x7f50
	ctx.r[7].s64 = ctx.r[11].s64 + -32592;
	// 82F38C4C: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F38C50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F38C54: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F38C58: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F38C5C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82F38C60: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F38C64: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82F38C68: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F38C6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F38C70: 4BFEC7D1  bl 0x82f25440
	ctx.lr = 0x82F38C74;
	sub_82F25440(ctx, base);
	// 82F38C74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F38C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F38C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F38C80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F38C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F38C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38C90 size=72
    let mut pc: u32 = 0x82F38C90;
    'dispatch: loop {
        match pc {
            0x82F38C90 => {
    //   block [0x82F38C90..0x82F38CD8)
	// 82F38C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F38C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F38C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F38C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F38CA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F38CA8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F38CAC: 392BBDA0  addi r9, r11, -0x4260
	ctx.r[9].s64 = ctx.r[11].s64 + -16992;
	// 82F38CB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F38CB4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F38CB8: 419A000C  beq cr6, 0x82f38cc4
	if ctx.cr[6].eq {
	pc = 0x82F38CC4; continue 'dispatch;
	}
	// 82F38CBC: 4B3875AD  bl 0x822c0268
	ctx.lr = 0x82F38CC0;
	sub_822C0268(ctx, base);
	// 82F38CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F38CC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F38CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F38CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F38CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F38CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38CD8 size=244
    let mut pc: u32 = 0x82F38CD8;
    'dispatch: loop {
        match pc {
            0x82F38CD8 => {
    //   block [0x82F38CD8..0x82F38DCC)
	// 82F38CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F38CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F38CE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F38CE4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38CE8: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82F38CEC: 80E40008  lwz r7, 8(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F38CF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F38CF4: 90810084  stw r4, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 82F38CF8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38DD0 size=164
    let mut pc: u32 = 0x82F38DD0;
    'dispatch: loop {
        match pc {
            0x82F38DD0 => {
    //   block [0x82F38DD0..0x82F38E74)
	// 82F38DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F38DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F38DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F38DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F38DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F38DE8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F38DEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F38DF0: 394BF634  addi r10, r11, -0x9cc
	ctx.r[10].s64 = ctx.r[11].s64 + -2508;
	// 82F38DF4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F38DF8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F38DFC: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F38E00: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F38E04: 409A0020  bne cr6, 0x82f38e24
	if !ctx.cr[6].eq {
	pc = 0x82F38E24; continue 'dispatch;
	}
	// 82F38E08: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38E0C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F38E10: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F38E14: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F38E18: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F38E1C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F38E20: 4BF67991  bl 0x82ea07b0
	ctx.lr = 0x82F38E24;
	sub_82EA07B0(ctx, base);
	// 82F38E24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F38E28: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F38E2C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F38E30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F38E34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F38E38: 419A0020  beq cr6, 0x82f38e58
	if ctx.cr[6].eq {
	pc = 0x82F38E58; continue 'dispatch;
	}
	// 82F38E3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38E40: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F38E44: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F38E48: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F38E4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F38E50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F38E54: 4BF6795D  bl 0x82ea07b0
	ctx.lr = 0x82F38E58;
	sub_82EA07B0(ctx, base);
	// 82F38E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F38E5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F38E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F38E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F38E68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F38E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F38E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F38E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F38E78 size=932
    let mut pc: u32 = 0x82F38E78;
    'dispatch: loop {
        match pc {
            0x82F38E78 => {
    //   block [0x82F38E78..0x82F3921C)
	// 82F38E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F38E7C: 4826F2B5  bl 0x831a8130
	ctx.lr = 0x82F38E80;
	sub_831A8130(ctx, base);
	// 82F38E80: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82F38E84: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F38E88: 820D0000  lwz r16, 0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38E8C: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82F38E90: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82F38E94: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82F38E98: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F38E9C: 7CCF3378  mr r15, r6
	ctx.r[15].u64 = ctx.r[6].u64;
	// 82F38EA0: 7D70582E  lwzx r11, r16, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F38EA4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F38EA8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F38EAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F38EB0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F38EB4: 40980020  bge cr6, 0x82f38ed4
	if !ctx.cr[6].lt {
	pc = 0x82F38ED4; continue 'dispatch;
	}
	// 82F38EB8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F38EBC: 3909F66C  addi r8, r9, -0x994
	ctx.r[8].s64 = ctx.r[9].s64 + -2452;
	// 82F38EC0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F38EC4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F38EC8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F38ECC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F38ED0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F38ED4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F38ED8: 83F10000  lwz r31, 0(r17)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38EDC: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38EE0: 80B10008  lwz r5, 8(r17)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F38EE4: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F38EE8: 4BF68179  bl 0x82ea1060
	ctx.lr = 0x82F38EEC;
	sub_82EA1060(ctx, base);
	// 82F38EEC: 39C00014  li r14, 0x14
	ctx.r[14].s64 = 20;
	// 82F38EF0: 7C70702E  lwzx r3, r16, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F38EF4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F38EF8: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F38EFC: 83D60010  lwz r30, 0x10(r22)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F38F00: 391E0001  addi r8, r30, 1
	ctx.r[8].s64 = ctx.r[30].s64 + 1;
	// 82F38F04: 551C2036  slwi r28, r8, 4
	ctx.r[28].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82F38F08: 7D4BE214  add r10, r11, r28
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82F38F0C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F38F10: 41990010  bgt cr6, 0x82f38f20
	if ctx.cr[6].gt {
	pc = 0x82F38F20; continue 'dispatch;
	}
	// 82F38F14: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F38F18: 7D725B78  mr r18, r11
	ctx.r[18].u64 = ctx.r[11].u64;
	// 82F38F1C: 4800001C  b 0x82f38f38
	pc = 0x82F38F38; continue 'dispatch;
	// 82F38F20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38F24: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F38F28: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F38F2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F38F30: 4E800421  bctrl
	ctx.lr = 0x82F38F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F38F34: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82F38F38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F38F3C: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82F38F40: 83B6000C  lwz r29, 0xc(r22)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F38F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F38F48: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F38F4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F38F50: 4E800421  bctrl
	ctx.lr = 0x82F38F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F38F54: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82F38F58: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F38F5C: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82F38F60: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82F38F64: 3BFEFFFF  addi r31, r30, -1
	ctx.r[31].s64 = ctx.r[30].s64 + -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39220 size=72
    let mut pc: u32 = 0x82F39220;
    'dispatch: loop {
        match pc {
            0x82F39220 => {
    //   block [0x82F39220..0x82F39268)
	// 82F39220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3922C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F39230: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F39234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F39238: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3923C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F39240: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F39244: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F39248: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3924C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F39250: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F39254: 4BFFEE5D  bl 0x82f380b0
	ctx.lr = 0x82F39258;
	sub_82F380B0(ctx, base);
	// 82F39258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3925C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F39260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F39264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39268 size=76
    let mut pc: u32 = 0x82F39268;
    'dispatch: loop {
        match pc {
            0x82F39268 => {
    //   block [0x82F39268..0x82F392B4)
	// 82F39268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39274: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F39278: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3927C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F39280: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F39284: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F39288: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3928C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F39290: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F39294: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F39298: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3929C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F392A0: 4BFFF109  bl 0x82f383a8
	ctx.lr = 0x82F392A4;
	sub_82F383A8(ctx, base);
	// 82F392A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F392A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F392AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F392B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F392B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F392B8 size=224
    let mut pc: u32 = 0x82F392B8;
    'dispatch: loop {
        match pc {
            0x82F392B8 => {
    //   block [0x82F392B8..0x82F39398)
	// 82F392B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F392BC: 4826EEB1  bl 0x831a816c
	ctx.lr = 0x82F392C0;
	sub_831A8130(ctx, base);
	// 82F392C0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F392C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F392C8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F392CC: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F392D0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F392D4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F392D8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F392DC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F392E0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F392E4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F392E8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F392EC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F392F0: 4200FFF0  bdnz 0x82f392e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F392E0; continue 'dispatch;
	}
	// 82F392F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F392F8: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F392FC: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F39300: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F39304: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F39308: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F3930C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F39310: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39398 size=176
    let mut pc: u32 = 0x82F39398;
    'dispatch: loop {
        match pc {
            0x82F39398 => {
    //   block [0x82F39398..0x82F39448)
	// 82F39398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F393A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F393A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F393A8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F393AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F393B0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F393B4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F393B8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F393BC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F393C0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F393C4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F393C8: 4BFFFAB1  bl 0x82f38e78
	ctx.lr = 0x82F393CC;
	sub_82F38E78(ctx, base);
	// 82F393CC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F393D0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F393D4: 40980044  bge cr6, 0x82f39418
	if !ctx.cr[6].lt {
	pc = 0x82F39418; continue 'dispatch;
	}
	// 82F393D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F393DC: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F393E0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39448 size=68
    let mut pc: u32 = 0x82F39448;
    'dispatch: loop {
        match pc {
            0x82F39448 => {
    //   block [0x82F39448..0x82F3948C)
	// 82F39448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39454: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F39458: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3945C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F39460: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F39464: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F39468: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3946C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F39470: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F39474: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F39478: 4BFFEC39  bl 0x82f380b0
	ctx.lr = 0x82F3947C;
	sub_82F380B0(ctx, base);
	// 82F3947C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F39480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F39484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F39488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39490 size=72
    let mut pc: u32 = 0x82F39490;
    'dispatch: loop {
        match pc {
            0x82F39490 => {
    //   block [0x82F39490..0x82F394D8)
	// 82F39490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3949C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F394A0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F394A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F394A8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F394AC: 392AF3E8  addi r9, r10, -0xc18
	ctx.r[9].s64 = ctx.r[10].s64 + -3096;
	// 82F394B0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F394B4: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F394B8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F394BC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F394C0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F394C4: 4BFFEEE5  bl 0x82f383a8
	ctx.lr = 0x82F394C8;
	sub_82F383A8(ctx, base);
	// 82F394C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F394CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F394D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F394D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F394D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F394D8 size=212
    let mut pc: u32 = 0x82F394D8;
    'dispatch: loop {
        match pc {
            0x82F394D8 => {
    //   block [0x82F394D8..0x82F395AC)
	// 82F394D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F394DC: 4826EC91  bl 0x831a816c
	ctx.lr = 0x82F394E0;
	sub_831A8130(ctx, base);
	// 82F394E0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F394E4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F394E8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F394EC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F394F0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F394F4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F394F8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F394FC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F39500: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F39504: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F39508: 4200FFF0  bdnz 0x82f394f8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F394F8; continue 'dispatch;
	}
	// 82F3950C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F39510: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F39514: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F39518: E8C60058  ld r6, 0x58(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F3951C: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F39520: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 82F39524: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F39528: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395B0 size=16
    let mut pc: u32 = 0x82F395B0;
    'dispatch: loop {
        match pc {
            0x82F395B0 => {
    //   block [0x82F395B0..0x82F395C0)
	// 82F395B0: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F395B4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F395B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F395BC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395C0 size=12
    let mut pc: u32 = 0x82F395C0;
    'dispatch: loop {
        match pc {
            0x82F395C0 => {
    //   block [0x82F395C0..0x82F395CC)
	// 82F395C0: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F395C4: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F395C8: 480365F8  b 0x82f6fbc0
	sub_82F6FBC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395CC size=4
    let mut pc: u32 = 0x82F395CC;
    'dispatch: loop {
        match pc {
            0x82F395CC => {
    //   block [0x82F395CC..0x82F395D0)
	// 82F395CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395D0 size=16
    let mut pc: u32 = 0x82F395D0;
    'dispatch: loop {
        match pc {
            0x82F395D0 => {
    //   block [0x82F395D0..0x82F395E0)
	// 82F395D0: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F395D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F395D8: 419A0008  beq cr6, 0x82f395e0
	if ctx.cr[6].eq {
		sub_82F395E0(ctx, base);
		return;
	}
	// 82F395DC: 48011174  b 0x82f4a750
	sub_82F4A750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395E0 size=8
    let mut pc: u32 = 0x82F395E0;
    'dispatch: loop {
        match pc {
            0x82F395E0 => {
    //   block [0x82F395E0..0x82F395E8)
	// 82F395E0: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F395E4: 48043654  b 0x82f7cc38
	sub_82F7CC38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395E8 size=16
    let mut pc: u32 = 0x82F395E8;
    'dispatch: loop {
        match pc {
            0x82F395E8 => {
    //   block [0x82F395E8..0x82F395F8)
	// 82F395E8: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F395EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F395F0: 419A0008  beq cr6, 0x82f395f8
	if ctx.cr[6].eq {
		sub_82F395F8(ctx, base);
		return;
	}
	// 82F395F4: 4801117C  b 0x82f4a770
	sub_82F4A770(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F395F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F395F8 size=8
    let mut pc: u32 = 0x82F395F8;
    'dispatch: loop {
        match pc {
            0x82F395F8 => {
    //   block [0x82F395F8..0x82F39600)
	// 82F395F8: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F395FC: 4804378C  b 0x82f7cd88
	sub_82F7CD88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39600 size=12
    let mut pc: u32 = 0x82F39600;
    'dispatch: loop {
        match pc {
            0x82F39600 => {
    //   block [0x82F39600..0x82F3960C)
	// 82F39600: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F39604: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F39608: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3960C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3960C size=8
    let mut pc: u32 = 0x82F3960C;
    'dispatch: loop {
        match pc {
            0x82F3960C => {
    //   block [0x82F3960C..0x82F39614)
	// 82F3960C: 48011D6C  b 0x82f4b378
	sub_82F4B378(ctx, base);
	return;
	// 82F39610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39618 size=12
    let mut pc: u32 = 0x82F39618;
    'dispatch: loop {
        match pc {
            0x82F39618 => {
    //   block [0x82F39618..0x82F39624)
	// 82F39618: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F3961C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F39620: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39624(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39624 size=8
    let mut pc: u32 = 0x82F39624;
    'dispatch: loop {
        match pc {
            0x82F39624 => {
    //   block [0x82F39624..0x82F3962C)
	// 82F39624: 48011D9C  b 0x82f4b3c0
	sub_82F4B3C0(ctx, base);
	return;
	// 82F39628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39630 size=12
    let mut pc: u32 = 0x82F39630;
    'dispatch: loop {
        match pc {
            0x82F39630 => {
    //   block [0x82F39630..0x82F3963C)
	// 82F39630: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F39634: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F39638: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3963C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3963C size=8
    let mut pc: u32 = 0x82F3963C;
    'dispatch: loop {
        match pc {
            0x82F3963C => {
    //   block [0x82F3963C..0x82F39644)
	// 82F3963C: 48011DCC  b 0x82f4b408
	sub_82F4B408(ctx, base);
	return;
	// 82F39640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39648 size=76
    let mut pc: u32 = 0x82F39648;
    'dispatch: loop {
        match pc {
            0x82F39648 => {
    //   block [0x82F39648..0x82F39694)
	// 82F39648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3964C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F39654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3965C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82F39660: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F39664: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F39668: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F3966C: 4804396D  bl 0x82f7cfd8
	ctx.lr = 0x82F39670;
	sub_82F7CFD8(ctx, base);
	// 82F39670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F39674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F39678: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82F3967C: 995F0084  stb r10, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u8 ) };
	// 82F39680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F39684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F39688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3968C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F39690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39698 size=112
    let mut pc: u32 = 0x82F39698;
    'dispatch: loop {
        match pc {
            0x82F39698 => {
    //   block [0x82F39698..0x82F39708)
	// 82F39698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3969C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F396A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F396A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F396A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F396AC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82F396B0: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F396B4: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F396B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F396BC: 419A0014  beq cr6, 0x82f396d0
	if ctx.cr[6].eq {
	pc = 0x82F396D0; continue 'dispatch;
	}
	// 82F396C0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F396C4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F396C8: 48048849  bl 0x82f81f10
	ctx.lr = 0x82F396CC;
	sub_82F81F10(ctx, base);
	// 82F396CC: 48000010  b 0x82f396dc
	pc = 0x82F396DC; continue 'dispatch;
	// 82F396D0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F396D4: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F396D8: 48043901  bl 0x82f7cfd8
	ctx.lr = 0x82F396DC;
	sub_82F7CFD8(ctx, base);
	// 82F396DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F396E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F396E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F396E8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F396EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F396F0: 4E800421  bctrl
	ctx.lr = 0x82F396F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F396F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F396F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F396FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F39700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F39704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39708 size=112
    let mut pc: u32 = 0x82F39708;
    'dispatch: loop {
        match pc {
            0x82F39708 => {
    //   block [0x82F39708..0x82F39778)
	// 82F39708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F39714: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3971C: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F39720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F39724: 409A0040  bne cr6, 0x82f39764
	if !ctx.cr[6].eq {
	pc = 0x82F39764; continue 'dispatch;
	}
	// 82F39728: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3972C: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39730: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82F39734: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F39738: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82F3973C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82F39740: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F39744: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39748: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3974C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F39750: 4E800421  bctrl
	ctx.lr = 0x82F39754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F39754: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82F39758: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3975C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F39760: 480448E9  bl 0x82f7e048
	ctx.lr = 0x82F39764;
	sub_82F7E048(ctx, base);
	// 82F39764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F39768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3976C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F39770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F39774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39778 size=144
    let mut pc: u32 = 0x82F39778;
    'dispatch: loop {
        match pc {
            0x82F39778 => {
    //   block [0x82F39778..0x82F39808)
	// 82F39778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3977C: 4826E9E9  bl 0x831a8164
	ctx.lr = 0x82F39780;
	sub_831A8130(ctx, base);
	// 82F39780: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39784: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F39788: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F3978C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F39790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F39794: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F39798: 48011CB9  bl 0x82f4b450
	ctx.lr = 0x82F3979C;
	sub_82F4B450(ctx, base);
	// 82F3979C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F397A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F397A4: 392BF7D4  addi r9, r11, -0x82c
	ctx.r[9].s64 = ctx.r[11].s64 + -2092;
	// 82F397A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F397AC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F397B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F397B4: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F397B8: 90FF0080  stw r7, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[7].u32 ) };
	// 82F397BC: 995F0084  stb r10, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u8 ) };
	// 82F397C0: 991F0085  stb r8, 0x85(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(133 as u32), ctx.r[8].u8 ) };
	// 82F397C4: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F397C8: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F397CC: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F397D0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F397D4: 4BF6788D  bl 0x82ea1060
	ctx.lr = 0x82F397D8;
	sub_82EA1060(ctx, base);
	// 82F397D8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82F397DC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F397E0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F397E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F397E8: 4804A789  bl 0x82f83f70
	ctx.lr = 0x82F397EC;
	sub_82F83F70(ctx, base);
	// 82F397EC: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82F397F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F397F4: C0069534  lfs f0, -0x6acc(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F397F8: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F397FC: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F39800: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F39804: 4826E9B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39808 size=592
    let mut pc: u32 = 0x82F39808;
    'dispatch: loop {
        match pc {
            0x82F39808 => {
    //   block [0x82F39808..0x82F39A58)
	// 82F39808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3980C: 4826E951  bl 0x831a815c
	ctx.lr = 0x82F39810;
	sub_831A8130(ctx, base);
	// 82F39810: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82F39814: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39818: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3981C: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 82F39820: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F39824: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F39828: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F3982C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F39830: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F39834: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39838: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3983C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F39840: 4098002C  bge cr6, 0x82f3986c
	if !ctx.cr[6].lt {
	pc = 0x82F3986C; continue 'dispatch;
	}
	// 82F39844: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F39848: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3984C: 38E9F828  addi r7, r9, -0x7d8
	ctx.r[7].s64 = ctx.r[9].s64 + -2008;
	// 82F39850: 38C8F81C  addi r6, r8, -0x7e4
	ctx.r[6].s64 = ctx.r[8].s64 + -2020;
	// 82F39854: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F39858: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F3985C: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F39860: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F39864: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39868: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F3986C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F39870: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F39874: 394BF78C  addi r10, r11, -0x874
	ctx.r[10].s64 = ctx.r[11].s64 + -2164;
	// 82F39878: 9B810054  stb r28, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u8 ) };
	// 82F3987C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F39880: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F39884: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39888: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F3988C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F39890: 480112E1  bl 0x82f4ab70
	ctx.lr = 0x82F39894;
	sub_82F4AB70(ctx, base);
	// 82F39894: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F39898: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F3989C: 419A0074  beq cr6, 0x82f39910
	if ctx.cr[6].eq {
	pc = 0x82F39910; continue 'dispatch;
	}
	// 82F398A0: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F398A4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F398A8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F398AC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F398B0: 40980020  bge cr6, 0x82f398d0
	if !ctx.cr[6].lt {
	pc = 0x82F398D0; continue 'dispatch;
	}
	// 82F398B4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F398B8: 3909F810  addi r8, r9, -0x7f0
	ctx.r[8].s64 = ctx.r[9].s64 + -2032;
	// 82F398BC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F398C0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F398C4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F398C8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F398CC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F398D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F398D4: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82F398D8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F398DC: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 82F398E0: 392AF3E8  addi r9, r10, -0xc18
	ctx.r[9].s64 = ctx.r[10].s64 + -3096;
	// 82F398E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F398E8: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F398EC: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F398F0: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82F398F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F398F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F398FC: 4800CEE5  bl 0x82f467e0
	ctx.lr = 0x82F39900;
	sub_82F467E0(ctx, base);
	// 82F39900: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F39904: 38E8BA80  addi r7, r8, -0x4580
	ctx.r[7].s64 = ctx.r[8].s64 + -17792;
	// 82F39908: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F3990C: 48000104  b 0x82f39a10
	pc = 0x82F39A10; continue 'dispatch;
	// 82F39910: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F39914: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 82F39918: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3991C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F39920: 392AC3D0  addi r9, r10, -0x3c30
	ctx.r[9].s64 = ctx.r[10].s64 + -15408;
	// 82F39924: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39928: C3EBBA78  lfs f31, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3992C: 91210080  stw r9, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82F39930: D3E100AC  stfs f31, 0xac(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82F39934: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F39938: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82F3993C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F39940: 480113D9  bl 0x82f4ad18
	ctx.lr = 0x82F39944;
	sub_82F4AD18(ctx, base);
	// 82F39944: 810100B0  lwz r8, 0xb0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82F39948: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3994C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82F39950: 3B8BBA80  addi r28, r11, -0x4580
	ctx.r[28].s64 = ctx.r[11].s64 + -17792;
	// 82F39954: 419A00B8  beq cr6, 0x82f39a0c
	if ctx.cr[6].eq {
	pc = 0x82F39A0C; continue 'dispatch;
	}
	// 82F39958: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3995C: C00100AC  lfs f0, 0xac(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F39960: C1AB0018  lfs f13, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F39964: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F39968: 40990048  ble cr6, 0x82f399b0
	if !ctx.cr[6].gt {
	pc = 0x82F399B0; continue 'dispatch;
	}
	// 82F3996C: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82F39970: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39974: 392100C0  addi r9, r1, 0xc0
	ctx.r[9].s64 = ctx.r[1].s64 + 192;
	// 82F39978: 93C100E0  stw r30, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[30].u32 ) };
	// 82F3997C: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82F39980: 38E100D0  addi r7, r1, 0xd0
	ctx.r[7].s64 = ctx.r[1].s64 + 208;
	// 82F39984: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39A58 size=20
    let mut pc: u32 = 0x82F39A58;
    'dispatch: loop {
        match pc {
            0x82F39A58 => {
    //   block [0x82F39A58..0x82F39A6C)
	// 82F39A58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F39A5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F39A60: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F39A64: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F39A68: 4BFFFDA0  b 0x82f39808
	sub_82F39808(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39A70 size=308
    let mut pc: u32 = 0x82F39A70;
    'dispatch: loop {
        match pc {
            0x82F39A70 => {
    //   block [0x82F39A70..0x82F39BA4)
	// 82F39A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39A74: 4826E6E5  bl 0x831a8158
	ctx.lr = 0x82F39A78;
	sub_831A8130(ctx, base);
	// 82F39A78: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39A7C: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39A80: 3B400018  li r26, 0x18
	ctx.r[26].s64 = 24;
	// 82F39A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F39A88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F39A8C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F39A90: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82F39A94: 7D5AD82E  lwzx r10, r26, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F39A98: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39A9C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F39AA0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F39AA4: 4098002C  bge cr6, 0x82f39ad0
	if !ctx.cr[6].lt {
	pc = 0x82F39AD0; continue 'dispatch;
	}
	// 82F39AA8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F39AAC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F39AB0: 38E9F828  addi r7, r9, -0x7d8
	ctx.r[7].s64 = ctx.r[9].s64 + -2008;
	// 82F39AB4: 38C8F81C  addi r6, r8, -0x7e4
	ctx.r[6].s64 = ctx.r[8].s64 + -2020;
	// 82F39AB8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F39ABC: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F39AC0: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F39AC4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F39AC8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39ACC: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F39AD0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F39AD4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82F39AD8: 394BF78C  addi r10, r11, -0x874
	ctx.r[10].s64 = ctx.r[11].s64 + -2164;
	// 82F39ADC: 9B210054  stb r25, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u8 ) };
	// 82F39AE0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F39AE4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F39AE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39AEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F39AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F39AF4: 4801107D  bl 0x82f4ab70
	ctx.lr = 0x82F39AF8;
	sub_82F4AB70(ctx, base);
	// 82F39AF8: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F39AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F39B00: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F39B04: 3B8BC3C0  addi r28, r11, -0x3c40
	ctx.r[28].s64 = ctx.r[11].s64 + -15424;
	// 82F39B08: 419A0060  beq cr6, 0x82f39b68
	if ctx.cr[6].eq {
	pc = 0x82F39B68; continue 'dispatch;
	}
	// 82F39B0C: 7D5AD82E  lwzx r10, r26, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F39B10: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39B14: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F39B18: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F39B1C: 40980020  bge cr6, 0x82f39b3c
	if !ctx.cr[6].lt {
	pc = 0x82F39B3C; continue 'dispatch;
	}
	// 82F39B20: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F39B24: 3909F810  addi r8, r9, -0x7f0
	ctx.r[8].s64 = ctx.r[9].s64 + -2032;
	// 82F39B28: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F39B2C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F39B30: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F39B34: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F39B38: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39B3C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F39B40: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82F39B44: 9B21005C  stb r25, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u8 ) };
	// 82F39B48: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82F39B4C: 394BF400  addi r10, r11, -0xc00
	ctx.r[10].s64 = ctx.r[11].s64 + -3072;
	// 82F39B50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39B54: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F39B58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F39B5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F39B60: 4800D219  bl 0x82f46d78
	ctx.lr = 0x82F39B64;
	sub_82F46D78(ctx, base);
	// 82F39B64: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82F39B68: 7D5AD82E  lwzx r10, r26, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F39B6C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82F39B70: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39B74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F39B78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F39B7C: 40980020  bge cr6, 0x82f39b9c
	if !ctx.cr[6].lt {
	pc = 0x82F39B9C; continue 'dispatch;
	}
	// 82F39B80: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F39B84: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F39B88: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F39B8C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F39B90: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F39B94: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F39B98: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39B9C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F39BA0: 4826E608  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39BA8 size=20
    let mut pc: u32 = 0x82F39BA8;
    'dispatch: loop {
        match pc {
            0x82F39BA8 => {
    //   block [0x82F39BA8..0x82F39BBC)
	// 82F39BA8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F39BAC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F39BB0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F39BB4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F39BB8: 4BFFFEB8  b 0x82f39a70
	sub_82F39A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39BC0 size=312
    let mut pc: u32 = 0x82F39BC0;
    'dispatch: loop {
        match pc {
            0x82F39BC0 => {
    //   block [0x82F39BC0..0x82F39CF8)
	// 82F39BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39BC4: 4826E599  bl 0x831a815c
	ctx.lr = 0x82F39BC8;
	sub_831A8130(ctx, base);
	// 82F39BC8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39BCC: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39BD0: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F39BD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F39BD8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F39BDC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F39BE0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F39BE4: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F39BE8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F39BEC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39BF0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F39BF4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F39BF8: 4098002C  bge cr6, 0x82f39c24
	if !ctx.cr[6].lt {
	pc = 0x82F39C24; continue 'dispatch;
	}
	// 82F39BFC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F39C00: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F39C04: 38E9F83C  addi r7, r9, -0x7c4
	ctx.r[7].s64 = ctx.r[9].s64 + -1988;
	// 82F39C08: 38C8F81C  addi r6, r8, -0x7e4
	ctx.r[6].s64 = ctx.r[8].s64 + -2020;
	// 82F39C0C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F39C10: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F39C14: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F39C18: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82F39C1C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39C20: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F39C24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F39C28: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F39C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F39C30: 390AF798  addi r8, r10, -0x868
	ctx.r[8].s64 = ctx.r[10].s64 + -2152;
	// 82F39C34: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 82F39C38: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F39C3C: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F39C40: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F39C44: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82F39C48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F39C4C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F39C50: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F39C54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F39C58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F39C5C: 48011F2D  bl 0x82f4bb88
	ctx.lr = 0x82F39C60;
	sub_82F4BB88(ctx, base);
	// 82F39C60: 88E10058  lbz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F39C64: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F39C68: 419A004C  beq cr6, 0x82f39cb4
	if ctx.cr[6].eq {
	pc = 0x82F39CB4; continue 'dispatch;
	}
	// 82F39C6C: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F39C70: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39C74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F39C78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F39C7C: 40980020  bge cr6, 0x82f39c9c
	if !ctx.cr[6].lt {
	pc = 0x82F39C9C; continue 'dispatch;
	}
	// 82F39C80: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F39C84: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F39C88: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F39C8C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F39C90: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F39C94: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F39C98: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39C9C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F39CA0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F39CA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F39CA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F39CAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F39CB0: 48000DA9  bl 0x82f3aa58
	ctx.lr = 0x82F39CB4;
	sub_82F3AA58(ctx, base);
	// 82F39CB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F39CB8: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F39CBC: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 82F39CC0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F39CC4: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39CC8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F39CCC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F39CD0: 40980020  bge cr6, 0x82f39cf0
	if !ctx.cr[6].lt {
	pc = 0x82F39CF0; continue 'dispatch;
	}
	// 82F39CD4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F39CD8: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F39CDC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F39CE0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F39CE4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F39CE8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F39CEC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F39CF0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F39CF4: 4826E4B8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39CF8 size=24
    let mut pc: u32 = 0x82F39CF8;
    'dispatch: loop {
        match pc {
            0x82F39CF8 => {
    //   block [0x82F39CF8..0x82F39D10)
	// 82F39CF8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F39CFC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F39D00: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F39D04: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F39D08: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82F39D0C: 4BFFFEB4  b 0x82f39bc0
	sub_82F39BC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F39D10 size=348
    let mut pc: u32 = 0x82F39D10;
    'dispatch: loop {
        match pc {
            0x82F39D10 => {
    //   block [0x82F39D10..0x82F39E6C)
	// 82F39D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39D14: 4826E451  bl 0x831a8164
	ctx.lr = 0x82F39D18;
	sub_831A8130(ctx, base);
	// 82F39D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39D1C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F39D20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F39D24: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F39D28: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F39D2C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F39D30: 419A0100  beq cr6, 0x82f39e30
	if ctx.cr[6].eq {
	pc = 0x82F39E30; continue 'dispatch;
	}
	// 82F39D34: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39D38: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F39D3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39D40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F39D44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39D48: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F39D4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F39D50: 4E800421  bctrl
	ctx.lr = 0x82F39D54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F39D54: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82F39D58: 41980088  blt cr6, 0x82f39de0
	if ctx.cr[6].lt {
	pc = 0x82F39DE0; continue 'dispatch;
	}
	// 82F39D5C: 419A0064  beq cr6, 0x82f39dc0
	if ctx.cr[6].eq {
	pc = 0x82F39DC0; continue 'dispatch;
	}
	// 82F39D60: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82F39D64: 41980010  blt cr6, 0x82f39d74
	if ctx.cr[6].lt {
	pc = 0x82F39D74; continue 'dispatch;
	}
	// 82F39D68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F39D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F39D70: 4826E444  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F39D74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39D78: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F39D7C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F39D80: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82F39D84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F39D88: 4BF669A9  bl 0x82ea0730
	ctx.lr = 0x82F39D8C;
	sub_82EA0730(ctx, base);
	// 82F39D8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F39D90: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82F39D94: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F39D98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39D9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F39DA0: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F39DA4: 480116AD  bl 0x82f4b450
	ctx.lr = 0x82F39DA8;
	sub_82F4B450(ctx, base);
	// 82F39DA8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F39DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F39DB0: 38E8F750  addi r7, r8, -0x8b0
	ctx.r[7].s64 = ctx.r[8].s64 + -2224;
	// 82F39DB4: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F39DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F39DBC: 4826E3F8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F39DC0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39DC4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F39DC8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F39DCC: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F39DD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F39DD4: 4BF6695D  bl 0x82ea0730
	ctx.lr = 0x82F39DD8;
	sub_82EA0730(ctx, base);
	// 82F39DD8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F39DDC: 48000070  b 0x82f39e4c
	pc = 0x82F39E4C; continue 'dispatch;
	// 82F39DE0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39DE4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F39DE8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F39DEC: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82F39DF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F39DF4: 4BF6693D  bl 0x82ea0730
	ctx.lr = 0x82F39DF8;
	sub_82EA0730(ctx, base);
	// 82F39DF8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F39DFC: 39200090  li r9, 0x90
	ctx.r[9].s64 = 144;
	// 82F39E00: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F39E04: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F39E08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F39E0C: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F39E10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F39E14: 4BFFF965  bl 0x82f39778
	ctx.lr = 0x82F39E18;
	sub_82F39778(ctx, base);
	// 82F39E18: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F39E1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F39E20: 38E8F854  addi r7, r8, -0x7ac
	ctx.r[7].s64 = ctx.r[8].s64 + -1964;
	// 82F39E24: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F39E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F39E2C: 4826E388  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F39E30: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F39E34: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F39E38: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F39E3C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F39E40: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F39E44: 4BF668ED  bl 0x82ea0730
	ctx.lr = 0x82F39E48;
	sub_82EA0730(ctx, base);
	// 82F39E48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F39E4C: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F39E50: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F39E54: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F39E58: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F39E5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F39E60: 4800D339  bl 0x82f47198
	ctx.lr = 0x82F39E64;
	sub_82F47198(ctx, base);
	// 82F39E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F39E68: 4826E34C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F39E70 size=128
    let mut pc: u32 = 0x82F39E70;
    'dispatch: loop {
        match pc {
            0x82F39E70 => {
    //   block [0x82F39E70..0x82F39EF0)
	// 82F39E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F39E78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F39E7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F39E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F39E84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F39E88: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F39E8C: 3BFE0030  addi r31, r30, 0x30
	ctx.r[31].s64 = ctx.r[30].s64 + 48;
	// 82F39E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F39E94: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F39E98: 48048079  bl 0x82f81f10
	ctx.lr = 0x82F39E9C;
	sub_82F81F10(ctx, base);
	// 82F39E9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F39EA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F39EA4: 997E0084  stb r11, 0x84(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 82F39EA8: 419A0014  beq cr6, 0x82f39ebc
	if ctx.cr[6].eq {
	pc = 0x82F39EBC; continue 'dispatch;
	}
	// 82F39EAC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F39EB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F39EB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F39EB8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82F39EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F39EC0: 48043339  bl 0x82f7d1f8
	ctx.lr = 0x82F39EC4;
	sub_82F7D1F8(ctx, base);
	// 82F39EC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F39EC8: 39400019  li r10, 0x19
	ctx.r[10].s64 = 25;
	// 82F39ECC: B15E0086  sth r10, 0x86(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 82F39ED0: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F39ED4: D01E003C  stfs f0, 0x3c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82F39ED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F39EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F39EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F39EE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F39EE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F39EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F39EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F39EF0 size=1452
    let mut pc: u32 = 0x82F39EF0;
    'dispatch: loop {
        match pc {
            0x82F39EF0 => {
    //   block [0x82F39EF0..0x82F3A49C)
	// 82F39EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F39EF4: 4826E23D  bl 0x831a8130
	ctx.lr = 0x82F39EF8;
	sub_831A8130(ctx, base);
	// 82F39EF8: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82F39EFC: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82F39F00: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3A4A0 size=392
    let mut pc: u32 = 0x82F3A4A0;
    'dispatch: loop {
        match pc {
            0x82F3A4A0 => {
    //   block [0x82F3A4A0..0x82F3A628)
	// 82F3A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3A4A4: 4826DCC1  bl 0x831a8164
	ctx.lr = 0x82F3A4A8;
	sub_831A8130(ctx, base);
	// 82F3A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3A4AC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F3A4B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3A4B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F3A4B8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F3A4BC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F3A4C0: 419A0118  beq cr6, 0x82f3a5d8
	if ctx.cr[6].eq {
	pc = 0x82F3A5D8; continue 'dispatch;
	}
	// 82F3A4C4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A4C8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A4CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3A4D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A4D4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A4D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A4DC: 4E800421  bctrl
	ctx.lr = 0x82F3A4E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3A4E0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82F3A4E4: 419800B8  blt cr6, 0x82f3a59c
	if ctx.cr[6].lt {
	pc = 0x82F3A59C; continue 'dispatch;
	}
	// 82F3A4E8: 419A0064  beq cr6, 0x82f3a54c
	if ctx.cr[6].eq {
	pc = 0x82F3A54C; continue 'dispatch;
	}
	// 82F3A4EC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82F3A4F0: 41980010  blt cr6, 0x82f3a500
	if ctx.cr[6].lt {
	pc = 0x82F3A500; continue 'dispatch;
	}
	// 82F3A4F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F3A4F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A4FC: 4826DCB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A500: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A504: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A508: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A50C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82F3A510: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A514: 4BF6621D  bl 0x82ea0730
	ctx.lr = 0x82F3A518;
	sub_82EA0730(ctx, base);
	// 82F3A518: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3A51C: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82F3A520: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F3A524: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F3A528: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3A52C: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A530: 48010F21  bl 0x82f4b450
	ctx.lr = 0x82F3A534;
	sub_82F4B450(ctx, base);
	// 82F3A534: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3A538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3A53C: 38E8F750  addi r7, r8, -0x8b0
	ctx.r[7].s64 = ctx.r[8].s64 + -2224;
	// 82F3A540: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3A544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A548: 4826DC6C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A54C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A550: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A554: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A558: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F3A55C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A560: 4BF661D1  bl 0x82ea0730
	ctx.lr = 0x82F3A564;
	sub_82EA0730(ctx, base);
	// 82F3A564: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F3A568: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F3A56C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3A570: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A574: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3A578: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A57C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3A580: 4800CC19  bl 0x82f47198
	ctx.lr = 0x82F3A584;
	sub_82F47198(ctx, base);
	// 82F3A584: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3A588: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F3A58C: 38E8F4C0  addi r7, r8, -0xb40
	ctx.r[7].s64 = ctx.r[8].s64 + -2880;
	// 82F3A590: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3A594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A598: 4826DC1C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A59C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A5A0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A5A4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A5A8: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82F3A5AC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A5B0: 4BF66181  bl 0x82ea0730
	ctx.lr = 0x82F3A5B4;
	sub_82EA0730(ctx, base);
	// 82F3A5B4: 39200090  li r9, 0x90
	ctx.r[9].s64 = 144;
	// 82F3A5B8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3A5BC: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A5C0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A5C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F3A5C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3A5CC: 4BFFF1AD  bl 0x82f39778
	ctx.lr = 0x82F3A5D0;
	sub_82F39778(ctx, base);
	// 82F3A5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A5D4: 4826DBE0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A5D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A5DC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A5E0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A5E4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F3A5E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A5EC: 4BF66145  bl 0x82ea0730
	ctx.lr = 0x82F3A5F0;
	sub_82EA0730(ctx, base);
	// 82F3A5F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F3A5F4: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F3A5F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F3A5FC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A600: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3A604: B13C0004  sth r9, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A608: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3A60C: 4800CB8D  bl 0x82f47198
	ctx.lr = 0x82F3A610;
	sub_82F47198(ctx, base);
	// 82F3A610: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3A614: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F3A618: 38E8F4C0  addi r7, r8, -0xb40
	ctx.r[7].s64 = ctx.r[8].s64 + -2880;
	// 82F3A61C: 90FC0000  stw r7, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3A620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A624: 4826DB90  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3A628 size=540
    let mut pc: u32 = 0x82F3A628;
    'dispatch: loop {
        match pc {
            0x82F3A628 => {
    //   block [0x82F3A628..0x82F3A844)
	// 82F3A628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3A62C: 4826DB39  bl 0x831a8164
	ctx.lr = 0x82F3A630;
	sub_831A8130(ctx, base);
	// 82F3A630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3A634: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F3A638: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3A63C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F3A640: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F3A644: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F3A648: 419A01C0  beq cr6, 0x82f3a808
	if ctx.cr[6].eq {
	pc = 0x82F3A808; continue 'dispatch;
	}
	// 82F3A64C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A650: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A654: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F3A658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3A65C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A660: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A664: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A668: 4E800421  bctrl
	ctx.lr = 0x82F3A66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3A66C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82F3A670: 41980148  blt cr6, 0x82f3a7b8
	if ctx.cr[6].lt {
	pc = 0x82F3A7B8; continue 'dispatch;
	}
	// 82F3A674: 419A0124  beq cr6, 0x82f3a798
	if ctx.cr[6].eq {
	pc = 0x82F3A798; continue 'dispatch;
	}
	// 82F3A678: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82F3A67C: 40980038  bge cr6, 0x82f3a6b4
	if !ctx.cr[6].lt {
	pc = 0x82F3A6B4; continue 'dispatch;
	}
	// 82F3A680: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A684: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A688: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3A68C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3A690: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A694: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A698: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A69C: 4E800421  bctrl
	ctx.lr = 0x82F3A6A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3A6A0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82F3A6A4: 419800B8  blt cr6, 0x82f3a75c
	if ctx.cr[6].lt {
	pc = 0x82F3A75C; continue 'dispatch;
	}
	// 82F3A6A8: 419A0064  beq cr6, 0x82f3a70c
	if ctx.cr[6].eq {
	pc = 0x82F3A70C; continue 'dispatch;
	}
	// 82F3A6AC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82F3A6B0: 41980010  blt cr6, 0x82f3a6c0
	if ctx.cr[6].lt {
	pc = 0x82F3A6C0; continue 'dispatch;
	}
	// 82F3A6B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F3A6B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A6BC: 4826DAF8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A6C0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A6C4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A6C8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A6CC: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82F3A6D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A6D4: 4BF6605D  bl 0x82ea0730
	ctx.lr = 0x82F3A6D8;
	sub_82EA0730(ctx, base);
	// 82F3A6D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3A6DC: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82F3A6E0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F3A6E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F3A6E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3A6EC: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A6F0: 48010D61  bl 0x82f4b450
	ctx.lr = 0x82F3A6F4;
	sub_82F4B450(ctx, base);
	// 82F3A6F4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3A6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3A6FC: 38E8F750  addi r7, r8, -0x8b0
	ctx.r[7].s64 = ctx.r[8].s64 + -2224;
	// 82F3A700: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3A704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A708: 4826DAAC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A70C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A710: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A714: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A718: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F3A71C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A720: 4BF66011  bl 0x82ea0730
	ctx.lr = 0x82F3A724;
	sub_82EA0730(ctx, base);
	// 82F3A724: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F3A728: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F3A72C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3A730: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A734: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3A738: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A73C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3A740: 4800CA59  bl 0x82f47198
	ctx.lr = 0x82F3A744;
	sub_82F47198(ctx, base);
	// 82F3A744: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3A748: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F3A74C: 38E8F4C0  addi r7, r8, -0xb40
	ctx.r[7].s64 = ctx.r[8].s64 + -2880;
	// 82F3A750: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3A754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A758: 4826DA5C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A75C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A760: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A764: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A768: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82F3A76C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A770: 4BF65FC1  bl 0x82ea0730
	ctx.lr = 0x82F3A774;
	sub_82EA0730(ctx, base);
	// 82F3A774: 39200090  li r9, 0x90
	ctx.r[9].s64 = 144;
	// 82F3A778: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3A77C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A780: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A784: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F3A788: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3A78C: 4BFFEFED  bl 0x82f39778
	ctx.lr = 0x82F3A790;
	sub_82F39778(ctx, base);
	// 82F3A790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A794: 4826DA20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A798: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A79C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A7A0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A7A4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F3A7A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A7AC: 4BF65F85  bl 0x82ea0730
	ctx.lr = 0x82F3A7B0;
	sub_82EA0730(ctx, base);
	// 82F3A7B0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3A7B4: 48000070  b 0x82f3a824
	pc = 0x82F3A824; continue 'dispatch;
	// 82F3A7B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A7BC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A7C0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A7C4: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82F3A7C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A7CC: 4BF65F65  bl 0x82ea0730
	ctx.lr = 0x82F3A7D0;
	sub_82EA0730(ctx, base);
	// 82F3A7D0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F3A7D4: 39200090  li r9, 0x90
	ctx.r[9].s64 = 144;
	// 82F3A7D8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3A7DC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A7E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3A7E4: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A7E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3A7EC: 4BFFEF8D  bl 0x82f39778
	ctx.lr = 0x82F3A7F0;
	sub_82F39778(ctx, base);
	// 82F3A7F0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3A7F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F3A7F8: 38E8F854  addi r7, r8, -0x7ac
	ctx.r[7].s64 = ctx.r[8].s64 + -1964;
	// 82F3A7FC: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3A800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A804: 4826D9B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F3A808: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A80C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3A810: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3A814: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F3A818: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3A81C: 4BF65F15  bl 0x82ea0730
	ctx.lr = 0x82F3A820;
	sub_82EA0730(ctx, base);
	// 82F3A820: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F3A824: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F3A828: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F3A82C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3A830: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F3A834: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3A838: 4800C961  bl 0x82f47198
	ctx.lr = 0x82F3A83C;
	sub_82F47198(ctx, base);
	// 82F3A83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3A840: 4826D974  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3A848 size=256
    let mut pc: u32 = 0x82F3A848;
    'dispatch: loop {
        match pc {
            0x82F3A848 => {
    //   block [0x82F3A848..0x82F3A948)
	// 82F3A848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3A84C: 4826D921  bl 0x831a816c
	ctx.lr = 0x82F3A850;
	sub_831A8130(ctx, base);
	// 82F3A850: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3A854: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3A858: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3A85C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3A860: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3A864: 38CAB090  addi r6, r10, -0x4f70
	ctx.r[6].s64 = ctx.r[10].s64 + -20336;
	// 82F3A868: 38A9B0D8  addi r5, r9, -0x4f28
	ctx.r[5].s64 = ctx.r[9].s64 + -20264;
	// 82F3A86C: 3888B128  addi r4, r8, -0x4ed8
	ctx.r[4].s64 = ctx.r[8].s64 + -20184;
	// 82F3A870: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3A874: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F3A878: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3A87C: 38EB9D10  addi r7, r11, -0x62f0
	ctx.r[7].s64 = ctx.r[11].s64 + -25328;
	// 82F3A880: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3A884: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F3A888: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F3A88C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3A890: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82F3A894: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F3A898: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3A89C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F3A8A0: 4BFEABA1  bl 0x82f25440
	ctx.lr = 0x82F3A8A4;
	sub_82F25440(ctx, base);
	// 82F3A8A4: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F3A8A8: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3A8AC: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F3A8B0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3A8B4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3A8B8: 3903A4A0  addi r8, r3, -0x5b60
	ctx.r[8].s64 = ctx.r[3].s64 + -23392;
	// 82F3A8BC: 38CA9808  addi r6, r10, -0x67f8
	ctx.r[6].s64 = ctx.r[10].s64 + -26616;
	// 82F3A8C0: 38A99BC0  addi r5, r9, -0x6440
	ctx.r[5].s64 = ctx.r[9].s64 + -25664;
	// 82F3A8C4: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F3A8C8: 38EB9A70  addi r7, r11, -0x6590
	ctx.r[7].s64 = ctx.r[11].s64 + -26000;
	// 82F3A8CC: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F3A8D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F3A8D4: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F3A8D8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F3A8DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F3A8E0: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82F3A8E4: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82F3A8E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F3A8EC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3A8F0: 4BFEAB51  bl 0x82f25440
	ctx.lr = 0x82F3A8F4;
	sub_82F25440(ctx, base);
	// 82F3A8F4: 3C8082F4  lis r4, -0x7d0c
	ctx.r[4].s64 = -2097938432;
	// 82F3A8F8: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F3A8FC: 9BC100A0  stb r30, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u8 ) };
	// 82F3A900: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3A904: 9BE100A1  stb r31, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[31].u8 ) };
	// 82F3A908: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3A90C: 3924A628  addi r9, r4, -0x59d8
	ctx.r[9].s64 = ctx.r[4].s64 + -23000;
	// 82F3A910: 39039A70  addi r8, r3, -0x6590
	ctx.r[8].s64 = ctx.r[3].s64 + -26000;
	// 82F3A914: 38AA9BC0  addi r5, r10, -0x6440
	ctx.r[5].s64 = ctx.r[10].s64 + -25664;
	// 82F3A918: 91210090  stw r9, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 82F3A91C: 38EB9808  addi r7, r11, -0x67f8
	ctx.r[7].s64 = ctx.r[11].s64 + -26616;
	// 82F3A920: 91010094  stw r8, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[8].u32 ) };
	// 82F3A924: 90A1009C  stw r5, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[5].u32 ) };
	// 82F3A928: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82F3A92C: 90E10098  stw r7, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82F3A930: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82F3A934: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F3A938: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F3A93C: 4BFEAB05  bl 0x82f25440
	ctx.lr = 0x82F3A940;
	sub_82F25440(ctx, base);
	// 82F3A940: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F3A944: 4826D878  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3A948 size=20
    let mut pc: u32 = 0x82F3A948;
    'dispatch: loop {
        match pc {
            0x82F3A948 => {
    //   block [0x82F3A948..0x82F3A95C)
	// 82F3A948: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A94C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A950: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F3A954: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A958: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3A960 size=20
    let mut pc: u32 = 0x82F3A960;
    'dispatch: loop {
        match pc {
            0x82F3A960 => {
    //   block [0x82F3A960..0x82F3A974)
	// 82F3A960: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A964: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A968: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3A96C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A970: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3A978 size=20
    let mut pc: u32 = 0x82F3A978;
    'dispatch: loop {
        match pc {
            0x82F3A978 => {
    //   block [0x82F3A978..0x82F3A98C)
	// 82F3A978: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A97C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A980: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3A984: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A988: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3A990 size=20
    let mut pc: u32 = 0x82F3A990;
    'dispatch: loop {
        match pc {
            0x82F3A990 => {
    //   block [0x82F3A990..0x82F3A9A4)
	// 82F3A990: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A994: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A998: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F3A99C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A9A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3A9A8 size=20
    let mut pc: u32 = 0x82F3A9A8;
    'dispatch: loop {
        match pc {
            0x82F3A9A8 => {
    //   block [0x82F3A9A8..0x82F3A9BC)
	// 82F3A9A8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3A9AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3A9B0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F3A9B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3A9B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3A9C0 size=72
    let mut pc: u32 = 0x82F3A9C0;
    'dispatch: loop {
        match pc {
            0x82F3A9C0 => {
    //   block [0x82F3A9C0..0x82F3AA08)
	// 82F3A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3A9C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3A9CC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3A9D0: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3A9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3A9D8: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3A9DC: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3A9E0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3A9E4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3A9E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3A9EC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3A9F0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3A9F4: 4800C385  bl 0x82f46d78
	ctx.lr = 0x82F3A9F8;
	sub_82F46D78(ctx, base);
	// 82F3A9F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3A9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3AA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3AA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3AA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3AA08 size=76
    let mut pc: u32 = 0x82F3AA08;
    'dispatch: loop {
        match pc {
            0x82F3AA08 => {
    //   block [0x82F3AA08..0x82F3AA54)
	// 82F3AA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3AA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3AA10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3AA14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3AA18: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3AA1C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3AA20: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3AA24: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3AA28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3AA2C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3AA30: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3AA34: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3AA38: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3AA3C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3AA40: 4800BDA1  bl 0x82f467e0
	ctx.lr = 0x82F3AA44;
	sub_82F467E0(ctx, base);
	// 82F3AA44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3AA48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3AA4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3AA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3AA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3AA58 size=224
    let mut pc: u32 = 0x82F3AA58;
    'dispatch: loop {
        match pc {
            0x82F3AA58 => {
    //   block [0x82F3AA58..0x82F3AB38)
	// 82F3AA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3AA5C: 4826D711  bl 0x831a816c
	ctx.lr = 0x82F3AA60;
	sub_831A8130(ctx, base);
	// 82F3AA60: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3AA64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3AA68: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3AA6C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F3AA70: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F3AA74: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3AA78: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F3AA7C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3AA80: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3AA84: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F3AA88: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F3AA8C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F3AA90: 4200FFF0  bdnz 0x82f3aa80
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3AA80; continue 'dispatch;
	}
	// 82F3AA94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3AA98: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F3AA9C: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F3AAA0: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F3AAA4: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F3AAA8: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F3AAAC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3AAB0: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3AB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3AB38 size=148
    let mut pc: u32 = 0x82F3AB38;
    'dispatch: loop {
        match pc {
            0x82F3AB38 => {
    //   block [0x82F3AB38..0x82F3ABCC)
	// 82F3AB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3AB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3AB40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3AB44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3AB48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3AB4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3AB50: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F3AB54: 392BC3C0  addi r9, r11, -0x3c40
	ctx.r[9].s64 = ctx.r[11].s64 + -15424;
	// 82F3AB58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F3AB5C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3AB60: 419A0054  beq cr6, 0x82f3abb4
	if ctx.cr[6].eq {
	pc = 0x82F3ABB4; continue 'dispatch;
	}
	// 82F3AB64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AB68: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3AB6C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3AB70: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F3AB74: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F3AB78: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F3AB7C: 4198001C  blt cr6, 0x82f3ab98
	if ctx.cr[6].lt {
	pc = 0x82F3AB98; continue 'dispatch;
	}
	// 82F3AB80: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F3AB84: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F3AB88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3AB8C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3AB90: 4BF654D1  bl 0x82ea0060
	ctx.lr = 0x82F3AB94;
	sub_82EA0060(ctx, base);
	// 82F3AB94: 48000020  b 0x82f3abb4
	pc = 0x82F3ABB4; continue 'dispatch;
	// 82F3AB98: 812B0044  lwz r9, 0x44(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F3AB9C: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82F3ABA0: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F3ABA4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F3ABA8: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82F3ABAC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F3ABB0: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82F3ABB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3ABB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3ABBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3ABC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3ABC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3ABC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3ABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3ABD0 size=232
    let mut pc: u32 = 0x82F3ABD0;
    'dispatch: loop {
        match pc {
            0x82F3ABD0 => {
    //   block [0x82F3ABD0..0x82F3ACB8)
	// 82F3ABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3ABD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3ABD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3ABDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3ABE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3ABE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3ABE8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F3ABEC: C1BE1030  lfs f13, 0x1030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F3ABF0: C01F3030  lfs f0, 0x3030(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3ABF4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F3ABF8: 419A0088  beq cr6, 0x82f3ac80
	if ctx.cr[6].eq {
	pc = 0x82F3AC80; continue 'dispatch;
	}
	// 82F3ABFC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AC00: 38BF3050  addi r5, r31, 0x3050
	ctx.r[5].s64 = ctx.r[31].s64 + 12368;
	// 82F3AC04: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3AC08: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82F3AC0C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F3AC10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3AC14: 4E800421  bctrl
	ctx.lr = 0x82F3AC18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3AC18: 397E1010  addi r11, r30, 0x1010
	ctx.r[11].s64 = ctx.r[30].s64 + 4112;
	// 82F3AC1C: 395F3010  addi r10, r31, 0x3010
	ctx.r[10].s64 = ctx.r[31].s64 + 12304;
	// 82F3AC20: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82F3AC24: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 82F3AC28: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3ACB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3ACB8 size=376
    let mut pc: u32 = 0x82F3ACB8;
    'dispatch: loop {
        match pc {
            0x82F3ACB8 => {
    //   block [0x82F3ACB8..0x82F3AE30)
	// 82F3ACB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3ACBC: 4826D495  bl 0x831a8150
	ctx.lr = 0x82F3ACC0;
	sub_831A8130(ctx, base);
	// 82F3ACC0: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3ACC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3ACC8: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82F3ACCC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F3ACD0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F3ACD4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F3ACD8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3ACDC: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F3ACE0: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F3ACE4: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82F3ACE8: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82F3ACEC: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3ACF0: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 82F3ACF4: 409A006C  bne cr6, 0x82f3ad60
	if !ctx.cr[6].eq {
	pc = 0x82F3AD60; continue 'dispatch;
	}
	// 82F3ACF8: 891F0008  lbz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3ACFC: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 82F3AD00: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AD04: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82F3AD08: 40990024  ble cr6, 0x82f3ad2c
	if !ctx.cr[6].gt {
	pc = 0x82F3AD2C; continue 'dispatch;
	}
	// 82F3AD0C: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82F3AD10: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AD14: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F3AD18: 409A0108  bne cr6, 0x82f3ae20
	if !ctx.cr[6].eq {
	pc = 0x82F3AE20; continue 'dispatch;
	}
	// 82F3AD1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F3AD20: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82F3AD24: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F3AD28: 4198FFE8  blt cr6, 0x82f3ad10
	if ctx.cr[6].lt {
	pc = 0x82F3AD10; continue 'dispatch;
	}
	// 82F3AD2C: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3AD30: 38690014  addi r3, r9, 0x14
	ctx.r[3].s64 = ctx.r[9].s64 + 20;
	// 82F3AD34: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F3AD38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3AD3C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3AD40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3AD44: 4E800421  bctrl
	ctx.lr = 0x82F3AD48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3AD48: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3AD4C: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82F3AD50: 3B610060  addi r27, r1, 0x60
	ctx.r[27].s64 = ctx.r[1].s64 + 96;
	// 82F3AD54: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F3AD58: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82F3AD5C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F3AD60: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AD64: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3AD68: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 82F3AD6C: 409A0080  bne cr6, 0x82f3adec
	if !ctx.cr[6].eq {
	pc = 0x82F3ADEC; continue 'dispatch;
	}
	// 82F3AD70: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3AD74: 893F0009  lbz r9, 9(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82F3AD78: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F3AD7C: 5567083E  rotlwi r7, r11, 1
	ctx.r[7].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82F3AD80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F3AD84: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F3AD88: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F3AD8C: 7FC7F8AE  lbzx r30, r7, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F3AD90: 40980028  bge cr6, 0x82f3adb8
	if !ctx.cr[6].lt {
	pc = 0x82F3ADB8; continue 'dispatch;
	}
	// 82F3AD94: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F3AD98: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F3AD9C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3ADA0: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F3ADA4: 409A007C  bne cr6, 0x82f3ae20
	if !ctx.cr[6].eq {
	pc = 0x82F3AE20; continue 'dispatch;
	}
	// 82F3ADA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F3ADAC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82F3ADB0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F3ADB4: 4198FFE8  blt cr6, 0x82f3ad9c
	if ctx.cr[6].lt {
	pc = 0x82F3AD9C; continue 'dispatch;
	}
	// 82F3ADB8: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3ADBC: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 82F3ADC0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F3ADC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3ADC8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3ADCC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3ADD0: 4E800421  bctrl
	ctx.lr = 0x82F3ADD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3ADD4: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3ADD8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F3ADDC: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 82F3ADE0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F3ADE4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82F3ADE8: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82F3ADEC: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3ADF0: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82F3ADF4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82F3ADF8: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F3ADFC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82F3AE00: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F3AE04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AE08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F3AE0C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3AE10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3AE14: 4E800421  bctrl
	ctx.lr = 0x82F3AE18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3AE18: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82F3AE1C: 4826D384  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 82F3AE20: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F3AE24: 9ADA0010  stb r22, 0x10(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), ctx.r[22].u8 ) };
	// 82F3AE28: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82F3AE2C: 4826D374  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3AE30 size=412
    let mut pc: u32 = 0x82F3AE30;
    'dispatch: loop {
        match pc {
            0x82F3AE30 => {
    //   block [0x82F3AE30..0x82F3AFCC)
	// 82F3AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3AE34: 4826D31D  bl 0x831a8150
	ctx.lr = 0x82F3AE38;
	sub_831A8130(ctx, base);
	// 82F3AE38: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82F3AE3C: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3AE40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3AE44: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F3AE48: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82F3AE4C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F3AE50: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F3AE54: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F3AE58: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AE5C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F3AE60: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82F3AE64: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82F3AE68: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82F3AE6C: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3AE70: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 82F3AE74: 409A006C  bne cr6, 0x82f3aee0
	if !ctx.cr[6].eq {
	pc = 0x82F3AEE0; continue 'dispatch;
	}
	// 82F3AE78: 891F0008  lbz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3AE7C: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 82F3AE80: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AE84: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82F3AE88: 40990024  ble cr6, 0x82f3aeac
	if !ctx.cr[6].gt {
	pc = 0x82F3AEAC; continue 'dispatch;
	}
	// 82F3AE8C: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82F3AE90: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AE94: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F3AE98: 409A0120  bne cr6, 0x82f3afb8
	if !ctx.cr[6].eq {
	pc = 0x82F3AFB8; continue 'dispatch;
	}
	// 82F3AE9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F3AEA0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82F3AEA4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F3AEA8: 4198FFE8  blt cr6, 0x82f3ae90
	if ctx.cr[6].lt {
	pc = 0x82F3AE90; continue 'dispatch;
	}
	// 82F3AEAC: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3AEB0: 38690014  addi r3, r9, 0x14
	ctx.r[3].s64 = ctx.r[9].s64 + 20;
	// 82F3AEB4: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3AEB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F3AEBC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3AEC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3AEC4: 4E800421  bctrl
	ctx.lr = 0x82F3AEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3AEC8: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3AECC: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82F3AED0: 3B610070  addi r27, r1, 0x70
	ctx.r[27].s64 = ctx.r[1].s64 + 112;
	// 82F3AED4: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82F3AED8: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82F3AEDC: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82F3AEE0: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AEE4: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3AEE8: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 82F3AEEC: 409A0080  bne cr6, 0x82f3af6c
	if !ctx.cr[6].eq {
	pc = 0x82F3AF6C; continue 'dispatch;
	}
	// 82F3AEF0: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3AEF4: 893F0009  lbz r9, 9(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82F3AEF8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F3AEFC: 5567083E  rotlwi r7, r11, 1
	ctx.r[7].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82F3AF00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F3AF04: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F3AF08: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F3AF0C: 7FC7F8AE  lbzx r30, r7, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F3AF10: 40980028  bge cr6, 0x82f3af38
	if !ctx.cr[6].lt {
	pc = 0x82F3AF38; continue 'dispatch;
	}
	// 82F3AF14: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F3AF18: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F3AF1C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AF20: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F3AF24: 409A0094  bne cr6, 0x82f3afb8
	if !ctx.cr[6].eq {
	pc = 0x82F3AFB8; continue 'dispatch;
	}
	// 82F3AF28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F3AF2C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82F3AF30: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F3AF34: 4198FFE8  blt cr6, 0x82f3af1c
	if ctx.cr[6].lt {
	pc = 0x82F3AF1C; continue 'dispatch;
	}
	// 82F3AF38: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3AF3C: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 82F3AF40: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3AF44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3AF48: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3AF4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3AF50: 4E800421  bctrl
	ctx.lr = 0x82F3AF54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3AF54: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3AF58: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82F3AF5C: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 82F3AF60: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F3AF64: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82F3AF68: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F3AF6C: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3AF70: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82F3AF74: 8161033C  lwz r11, 0x33c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(828 as u32) ) } as u64;
	// 82F3AF78: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82F3AF7C: 81010334  lwz r8, 0x334(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(820 as u32) ) } as u64;
	// 82F3AF80: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F3AF84: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F3AF88: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82F3AF8C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3AF90: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F3AF94: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F3AF98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F3AF9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82F3AFA0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F3AFA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F3AFA8: 4E800421  bctrl
	ctx.lr = 0x82F3AFAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3AFAC: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 82F3AFB0: CBE1FFA0  lfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82F3AFB4: 4826D1EC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 82F3AFB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F3AFBC: 9ADA0010  stb r22, 0x10(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), ctx.r[22].u8 ) };
	// 82F3AFC0: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 82F3AFC4: CBE1FFA0  lfd f31, -0x60(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82F3AFC8: 4826D1D8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3AFD0 size=96
    let mut pc: u32 = 0x82F3AFD0;
    'dispatch: loop {
        match pc {
            0x82F3AFD0 => {
    //   block [0x82F3AFD0..0x82F3B030)
	// 82F3AFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3AFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3AFD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3AFDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3AFE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3AFE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3AFE8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F3AFEC: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F3AFF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F3AFF4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3AFF8: 419A0020  beq cr6, 0x82f3b018
	if ctx.cr[6].eq {
	pc = 0x82F3B018; continue 'dispatch;
	}
	// 82F3AFFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B000: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3B004: 38C00022  li r6, 0x22
	ctx.r[6].s64 = 34;
	// 82F3B008: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3B00C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F3B010: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3B014: 4BF6579D  bl 0x82ea07b0
	ctx.lr = 0x82F3B018;
	sub_82EA07B0(ctx, base);
	// 82F3B018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3B01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3B020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3B02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B030 size=96
    let mut pc: u32 = 0x82F3B030;
    'dispatch: loop {
        match pc {
            0x82F3B030 => {
    //   block [0x82F3B030..0x82F3B090)
	// 82F3B030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B03C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B044: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3B048: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F3B04C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F3B050: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F3B054: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3B058: 419A0020  beq cr6, 0x82f3b078
	if ctx.cr[6].eq {
	pc = 0x82F3B078; continue 'dispatch;
	}
	// 82F3B05C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B060: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3B064: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F3B068: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3B06C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F3B070: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3B074: 4BF6573D  bl 0x82ea07b0
	ctx.lr = 0x82F3B078;
	sub_82EA07B0(ctx, base);
	// 82F3B078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3B07C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3B080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3B08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B090 size=72
    let mut pc: u32 = 0x82F3B090;
    'dispatch: loop {
        match pc {
            0x82F3B090 => {
    //   block [0x82F3B090..0x82F3B0D8)
	// 82F3B090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B09C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3B0A0: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3B0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3B0A8: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3B0AC: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3B0B0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3B0B4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3B0B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3B0BC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B0C0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3B0C4: 4BFFE9AD  bl 0x82f39a70
	ctx.lr = 0x82F3B0C8;
	sub_82F39A70(ctx, base);
	// 82F3B0C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B0D8 size=76
    let mut pc: u32 = 0x82F3B0D8;
    'dispatch: loop {
        match pc {
            0x82F3B0D8 => {
    //   block [0x82F3B0D8..0x82F3B124)
	// 82F3B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B0E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B0E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3B0E8: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3B0EC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3B0F0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3B0F4: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3B0F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3B0FC: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3B100: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3B104: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3B108: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B10C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3B110: 4BFFE6F9  bl 0x82f39808
	ctx.lr = 0x82F3B114;
	sub_82F39808(ctx, base);
	// 82F3B114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B128 size=224
    let mut pc: u32 = 0x82F3B128;
    'dispatch: loop {
        match pc {
            0x82F3B128 => {
    //   block [0x82F3B128..0x82F3B208)
	// 82F3B128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B12C: 4826D041  bl 0x831a816c
	ctx.lr = 0x82F3B130;
	sub_831A8130(ctx, base);
	// 82F3B130: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B138: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3B13C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F3B140: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F3B144: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3B148: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F3B14C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3B150: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3B154: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F3B158: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F3B15C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F3B160: 4200FFF0  bdnz 0x82f3b150
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3B150; continue 'dispatch;
	}
	// 82F3B164: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3B168: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F3B16C: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F3B170: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F3B174: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F3B178: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F3B17C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3B180: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B208 size=112
    let mut pc: u32 = 0x82F3B208;
    'dispatch: loop {
        match pc {
            0x82F3B208 => {
    //   block [0x82F3B208..0x82F3B278)
	// 82F3B208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B214: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B21C: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F3B220: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F3B224: 409A0040  bne cr6, 0x82f3b264
	if !ctx.cr[6].eq {
	pc = 0x82F3B264; continue 'dispatch;
	}
	// 82F3B228: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3B22C: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B230: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82F3B234: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82F3B238: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82F3B23C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82F3B240: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F3B244: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B248: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3B24C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3B250: 4E800421  bctrl
	ctx.lr = 0x82F3B254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3B254: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82F3B258: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3B25C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F3B260: 48042DE9  bl 0x82f7e048
	ctx.lr = 0x82F3B264;
	sub_82F7E048(ctx, base);
	// 82F3B264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3B268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3B274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B278 size=68
    let mut pc: u32 = 0x82F3B278;
    'dispatch: loop {
        match pc {
            0x82F3B278 => {
    //   block [0x82F3B278..0x82F3B2BC)
	// 82F3B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B284: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3B288: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3B28C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F3B290: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F3B294: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F3B298: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3B29C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F3B2A0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3B2A4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B2A8: 4BFFE7C9  bl 0x82f39a70
	ctx.lr = 0x82F3B2AC;
	sub_82F39A70(ctx, base);
	// 82F3B2AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B2C0 size=72
    let mut pc: u32 = 0x82F3B2C0;
    'dispatch: loop {
        match pc {
            0x82F3B2C0 => {
    //   block [0x82F3B2C0..0x82F3B308)
	// 82F3B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B2CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3B2D0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3B2D4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3B2D8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3B2DC: 392AF3E8  addi r9, r10, -0xc18
	ctx.r[9].s64 = ctx.r[10].s64 + -3096;
	// 82F3B2E0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3B2E4: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3B2E8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F3B2EC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3B2F0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B2F4: 4BFFE515  bl 0x82f39808
	ctx.lr = 0x82F3B2F8;
	sub_82F39808(ctx, base);
	// 82F3B2F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B308 size=212
    let mut pc: u32 = 0x82F3B308;
    'dispatch: loop {
        match pc {
            0x82F3B308 => {
    //   block [0x82F3B308..0x82F3B3DC)
	// 82F3B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B30C: 4826CE61  bl 0x831a816c
	ctx.lr = 0x82F3B310;
	sub_831A8130(ctx, base);
	// 82F3B310: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B314: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3B318: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F3B31C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F3B320: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F3B324: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3B328: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3B32C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F3B330: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F3B334: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F3B338: 4200FFF0  bdnz 0x82f3b328
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3B328; continue 'dispatch;
	}
	// 82F3B33C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3B340: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F3B344: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F3B348: E8C60058  ld r6, 0x58(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F3B34C: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F3B350: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 82F3B354: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3B358: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B3E0 size=204
    let mut pc: u32 = 0x82F3B3E0;
    'dispatch: loop {
        match pc {
            0x82F3B3E0 => {
    //   block [0x82F3B3E0..0x82F3B4AC)
	// 82F3B3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B3E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B3EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B3F0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F3B3F4: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B3FC: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82F3B400: 39643010  addi r11, r4, 0x3010
	ctx.r[11].s64 = ctx.r[4].s64 + 12304;
	// 82F3B404: 395F1010  addi r10, r31, 0x1010
	ctx.r[10].s64 = ctx.r[31].s64 + 4112;
	// 82F3B408: C009BA78  lfs f0, -0x4588(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3B40C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82F3B410: D01F1030  stfs f0, 0x1030(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4144 as u32), tmp.u32 ) };
	// 82F3B414: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3B418: C00608A4  lfs f0, 0x8a4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3B41C: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 82F3B420: D01F1050  stfs f0, 0x1050(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4176 as u32), tmp.u32 ) };
	// 82F3B424: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
	// 82F3B428: D01F1054  stfs f0, 0x1054(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4180 as u32), tmp.u32 ) };
	// 82F3B42C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B4B0 size=176
    let mut pc: u32 = 0x82F3B4B0;
    'dispatch: loop {
        match pc {
            0x82F3B4B0 => {
    //   block [0x82F3B4B0..0x82F3B560)
	// 82F3B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B4B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3B4BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B4C0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F3B4C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B4C8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3B4CC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3B4D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3B4D4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3B4D8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B4DC: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3B4E0: 4BFFEA11  bl 0x82f39ef0
	ctx.lr = 0x82F3B4E4;
	sub_82F39EF0(ctx, base);
	// 82F3B4E4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B4E8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F3B4EC: 40980044  bge cr6, 0x82f3b530
	if !ctx.cr[6].lt {
	pc = 0x82F3B530; continue 'dispatch;
	}
	// 82F3B4F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3B4F4: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F3B4F8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B560 size=84
    let mut pc: u32 = 0x82F3B560;
    'dispatch: loop {
        match pc {
            0x82F3B560 => {
    //   block [0x82F3B560..0x82F3B5B4)
	// 82F3B560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B56C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B574: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82F3B578: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F3B57C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3B580: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3B584: 48041A55  bl 0x82f7cfd8
	ctx.lr = 0x82F3B588;
	sub_82F7CFD8(ctx, base);
	// 82F3B588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B58C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3B590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3B594: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B598: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3B59C: 4E800421  bctrl
	ctx.lr = 0x82F3B5A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3B5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3B5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B5AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3B5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3B5B8 size=8
    let mut pc: u32 = 0x82F3B5B8;
    'dispatch: loop {
        match pc {
            0x82F3B5B8 => {
    //   block [0x82F3B5B8..0x82F3B5C0)
	// 82F3B5B8: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82F3B5BC: 4804167C  b 0x82f7cc38
	sub_82F7CC38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3B5C0 size=8
    let mut pc: u32 = 0x82F3B5C0;
    'dispatch: loop {
        match pc {
            0x82F3B5C0 => {
    //   block [0x82F3B5C0..0x82F3B5C8)
	// 82F3B5C0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82F3B5C4: 480417C4  b 0x82f7cd88
	sub_82F7CD88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3B5C8 size=16
    let mut pc: u32 = 0x82F3B5C8;
    'dispatch: loop {
        match pc {
            0x82F3B5C8 => {
    //   block [0x82F3B5C8..0x82F3B5D8)
	// 82F3B5C8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3B5CC: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3B5D0: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82F3B5D4: 480345EC  b 0x82f6fbc0
	sub_82F6FBC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B5D8 size=100
    let mut pc: u32 = 0x82F3B5D8;
    'dispatch: loop {
        match pc {
            0x82F3B5D8 => {
    //   block [0x82F3B5D8..0x82F3B63C)
	// 82F3B5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B5E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B5E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B5E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B5EC: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B5F0: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82F3B5F4: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F3B5F8: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82F3B5FC: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F3B600: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3B604: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82F3B608: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B60C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3B610: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3B614: 4E800421  bctrl
	ctx.lr = 0x82F3B618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3B618: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82F3B61C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3B620: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F3B624: 48042A25  bl 0x82f7e048
	ctx.lr = 0x82F3B628;
	sub_82F7E048(ctx, base);
	// 82F3B628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3B62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3B638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B640 size=76
    let mut pc: u32 = 0x82F3B640;
    'dispatch: loop {
        match pc {
            0x82F3B640 => {
    //   block [0x82F3B640..0x82F3B68C)
	// 82F3B640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B64C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3B650: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3B654: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3B658: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3B65C: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3B660: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3B664: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3B668: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3B66C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3B670: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B674: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3B678: 4800B169  bl 0x82f467e0
	ctx.lr = 0x82F3B67C;
	sub_82F467E0(ctx, base);
	// 82F3B67C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B690 size=72
    let mut pc: u32 = 0x82F3B690;
    'dispatch: loop {
        match pc {
            0x82F3B690 => {
    //   block [0x82F3B690..0x82F3B6D8)
	// 82F3B690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B69C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3B6A0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3B6A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3B6A8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3B6AC: 392AF3E8  addi r9, r10, -0xc18
	ctx.r[9].s64 = ctx.r[10].s64 + -3096;
	// 82F3B6B0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3B6B4: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3B6B8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F3B6BC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3B6C0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B6C4: 4800B11D  bl 0x82f467e0
	ctx.lr = 0x82F3B6C8;
	sub_82F467E0(ctx, base);
	// 82F3B6C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3B6D8 size=4
    let mut pc: u32 = 0x82F3B6D8;
    'dispatch: loop {
        match pc {
            0x82F3B6D8 => {
    //   block [0x82F3B6D8..0x82F3B6DC)
	// 82F3B6D8: 4BFFF380  b 0x82f3aa58
	sub_82F3AA58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B6E0 size=72
    let mut pc: u32 = 0x82F3B6E0;
    'dispatch: loop {
        match pc {
            0x82F3B6E0 => {
    //   block [0x82F3B6E0..0x82F3B728)
	// 82F3B6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B6E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B6EC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3B6F0: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3B6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3B6F8: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3B6FC: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3B700: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3B704: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3B708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3B70C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B710: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3B714: 4800B665  bl 0x82f46d78
	ctx.lr = 0x82F3B718;
	sub_82F46D78(ctx, base);
	// 82F3B718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3B728 size=456
    let mut pc: u32 = 0x82F3B728;
    'dispatch: loop {
        match pc {
            0x82F3B728 => {
    //   block [0x82F3B728..0x82F3B8F0)
	// 82F3B728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B72C: 4826CA31  bl 0x831a815c
	ctx.lr = 0x82F3B730;
	sub_831A8130(ctx, base);
	// 82F3B730: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B734: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3B738: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82F3B73C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F3B740: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3B744: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82F3B748: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82F3B74C: 90C10068  stw r6, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 82F3B750: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F3B754: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3B758: C1A60058  lfs f13, 0x58(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F3B75C: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3B760: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B764: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3B768: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F3B76C: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B770: 39640040  addi r11, r4, 0x40
	ctx.r[11].s64 = ctx.r[4].s64 + 64;
	// 82F3B774: 39450040  addi r10, r5, 0x40
	ctx.r[10].s64 = ctx.r[5].s64 + 64;
	// 82F3B778: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82F3B77C: C185005C  lfs f12, 0x5c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(92 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3B8F0 size=24
    let mut pc: u32 = 0x82F3B8F0;
    'dispatch: loop {
        match pc {
            0x82F3B8F0 => {
    //   block [0x82F3B8F0..0x82F3B908)
	// 82F3B8F0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3B8F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F3B8F8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3B8FC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F3B900: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82F3B904: 4BFFF154  b 0x82f3aa58
	sub_82F3AA58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B908 size=68
    let mut pc: u32 = 0x82F3B908;
    'dispatch: loop {
        match pc {
            0x82F3B908 => {
    //   block [0x82F3B908..0x82F3B94C)
	// 82F3B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B914: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3B918: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3B91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F3B920: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F3B924: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F3B928: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3B92C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F3B930: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3B934: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3B938: 4800B441  bl 0x82f46d78
	ctx.lr = 0x82F3B93C;
	sub_82F46D78(ctx, base);
	// 82F3B93C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3B940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B950 size=104
    let mut pc: u32 = 0x82F3B950;
    'dispatch: loop {
        match pc {
            0x82F3B950 => {
    //   block [0x82F3B950..0x82F3B9B8)
	// 82F3B950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3B958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3B95C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B964: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3B968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3B96C: 392BF8BC  addi r9, r11, -0x744
	ctx.r[9].s64 = ctx.r[11].s64 + -1860;
	// 82F3B970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3B974: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F3B978: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F3B97C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F3B980: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F3B984: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3B988: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F3B98C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F3B990: 911F0018  stw r8, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82F3B994: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B998: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F3B99C: 4804185D  bl 0x82f7d1f8
	ctx.lr = 0x82F3B9A0;
	sub_82F7D1F8(ctx, base);
	// 82F3B9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3B9A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3B9A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3B9AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3B9B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3B9B8 size=128
    let mut pc: u32 = 0x82F3B9B8;
    'dispatch: loop {
        match pc {
            0x82F3B9B8 => {
    //   block [0x82F3B9B8..0x82F3BA38)
	// 82F3B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3B9BC: 4826C7B1  bl 0x831a816c
	ctx.lr = 0x82F3B9C0;
	sub_831A8130(ctx, base);
	// 82F3B9C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3B9C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3B9C8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3B9CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F3B9D0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3B9D4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82F3B9D8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3B9DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3B9E0: 4BF64D51  bl 0x82ea0730
	ctx.lr = 0x82F3B9E4;
	sub_82EA0730(ctx, base);
	// 82F3B9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3B9E8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3B9EC: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 82F3B9F0: 38E9F8BC  addi r7, r9, -0x744
	ctx.r[7].s64 = ctx.r[9].s64 + -1860;
	// 82F3B9F4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F3B9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3B9FC: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F3BA00: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 82F3BA04: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F3BA08: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F3BA0C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F3BA10: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3BA14: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F3BA18: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F3BA1C: 90BF0018  stw r5, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82F3BA20: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BA24: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F3BA28: 480417D1  bl 0x82f7d1f8
	ctx.lr = 0x82F3BA2C;
	sub_82F7D1F8(ctx, base);
	// 82F3BA2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3BA30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3BA34: 4826C788  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3BA38 size=68
    let mut pc: u32 = 0x82F3BA38;
    'dispatch: loop {
        match pc {
            0x82F3BA38 => {
    //   block [0x82F3BA38..0x82F3BA7C)
	// 82F3BA38: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3BA3C: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3BA40: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3BA44: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3BA48: 38EBB9B8  addi r7, r11, -0x4648
	ctx.r[7].s64 = ctx.r[11].s64 + -17992;
	// 82F3BA4C: 38CAA9C0  addi r6, r10, -0x5640
	ctx.r[6].s64 = ctx.r[10].s64 + -22080;
	// 82F3BA50: 38A9AA08  addi r5, r9, -0x55f8
	ctx.r[5].s64 = ctx.r[9].s64 + -22008;
	// 82F3BA54: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3BA58: 3888AA58  addi r4, r8, -0x55a8
	ctx.r[4].s64 = ctx.r[8].s64 + -21928;
	// 82F3BA5C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F3BA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3BA64: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F3BA68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3BA6C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F3BA70: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F3BA74: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 82F3BA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BA80 size=140
    let mut pc: u32 = 0x82F3BA80;
    'dispatch: loop {
        match pc {
            0x82F3BA80 => {
    //   block [0x82F3BA80..0x82F3BB0C)
	// 82F3BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BA84: 4826C6E9  bl 0x831a816c
	ctx.lr = 0x82F3BA88;
	sub_831A8130(ctx, base);
	// 82F3BA88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BA8C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BA90: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3BA94: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F3BA98: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3BA9C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82F3BAA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3BAA4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3BAA8: 4BF64C89  bl 0x82ea0730
	ctx.lr = 0x82F3BAAC;
	sub_82EA0730(ctx, base);
	// 82F3BAAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3BAB0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3BAB4: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 82F3BAB8: 38E9F8BC  addi r7, r9, -0x744
	ctx.r[7].s64 = ctx.r[9].s64 + -1860;
	// 82F3BABC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F3BAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3BAC4: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F3BAC8: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 82F3BACC: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F3BAD0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F3BAD4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F3BAD8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3BADC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F3BAE0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F3BAE4: 90BF0018  stw r5, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82F3BAE8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BAEC: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F3BAF0: 48041709  bl 0x82f7d1f8
	ctx.lr = 0x82F3BAF4;
	sub_82F7D1F8(ctx, base);
	// 82F3BAF4: 3C608213  lis r3, -0x7ded
	ctx.r[3].s64 = -2112684032;
	// 82F3BAF8: 3963F8FC  addi r11, r3, -0x704
	ctx.r[11].s64 = ctx.r[3].s64 + -1796;
	// 82F3BAFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3BB00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F3BB04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3BB08: 4826C6B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3BB10 size=64
    let mut pc: u32 = 0x82F3BB10;
    'dispatch: loop {
        match pc {
            0x82F3BB10 => {
    //   block [0x82F3BB10..0x82F3BB50)
	// 82F3BB10: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3BB14: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3BB18: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3BB1C: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3BB20: 38EBBA80  addi r7, r11, -0x4580
	ctx.r[7].s64 = ctx.r[11].s64 + -17792;
	// 82F3BB24: 38CA6D78  addi r6, r10, 0x6d78
	ctx.r[6].s64 = ctx.r[10].s64 + 28024;
	// 82F3BB28: 38A967E0  addi r5, r9, 0x67e0
	ctx.r[5].s64 = ctx.r[9].s64 + 26592;
	// 82F3BB2C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3BB30: 38886AA8  addi r4, r8, 0x6aa8
	ctx.r[4].s64 = ctx.r[8].s64 + 27304;
	// 82F3BB34: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F3BB38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F3BB3C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F3BB40: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F3BB44: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F3BB48: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82F3BB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BB50 size=204
    let mut pc: u32 = 0x82F3BB50;
    'dispatch: loop {
        match pc {
            0x82F3BB50 => {
    //   block [0x82F3BB50..0x82F3BC1C)
	// 82F3BB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3BB58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3BB5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3BB60: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BB64: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3BB68: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3BB6C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3BB70: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3BB74: 38CA6D78  addi r6, r10, 0x6d78
	ctx.r[6].s64 = ctx.r[10].s64 + 28024;
	// 82F3BB78: 38A967E0  addi r5, r9, 0x67e0
	ctx.r[5].s64 = ctx.r[9].s64 + 26592;
	// 82F3BB7C: 38886AA8  addi r4, r8, 0x6aa8
	ctx.r[4].s64 = ctx.r[8].s64 + 27304;
	// 82F3BB80: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3BB84: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F3BB88: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3BB8C: 38EBBA80  addi r7, r11, -0x4580
	ctx.r[7].s64 = ctx.r[11].s64 + -17792;
	// 82F3BB90: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3BB94: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F3BB98: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F3BB9C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3BBA0: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82F3BBA4: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F3BBA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3BBAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3BBB0: 4BFE9891  bl 0x82f25440
	ctx.lr = 0x82F3BBB4;
	sub_82F25440(ctx, base);
	// 82F3BBB4: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F3BBB8: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3BBBC: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F3BBC0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3BBC4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3BBC8: 3903B9B8  addi r8, r3, -0x4648
	ctx.r[8].s64 = ctx.r[3].s64 + -17992;
	// 82F3BBCC: 38CAAA08  addi r6, r10, -0x55f8
	ctx.r[6].s64 = ctx.r[10].s64 + -22008;
	// 82F3BBD0: 38A9AA58  addi r5, r9, -0x55a8
	ctx.r[5].s64 = ctx.r[9].s64 + -21928;
	// 82F3BBD4: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F3BBD8: 38EBA9C0  addi r7, r11, -0x5640
	ctx.r[7].s64 = ctx.r[11].s64 + -22080;
	// 82F3BBDC: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F3BBE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F3BBE4: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F3BBE8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F3BBEC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82F3BBF0: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F3BBF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F3BBF8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3BBFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3BC00: 4BFE9841  bl 0x82f25440
	ctx.lr = 0x82F3BC04;
	sub_82F25440(ctx, base);
	// 82F3BC04: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F3BC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3BC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3BC10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3BC14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3BC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BC20 size=152
    let mut pc: u32 = 0x82F3BC20;
    'dispatch: loop {
        match pc {
            0x82F3BC20 => {
    //   block [0x82F3BC20..0x82F3BCB8)
	// 82F3BC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3BC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3BC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3BC30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BC34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3BC38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3BC3C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F3BC40: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F3BC44: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F3BC48: 409A0020  bne cr6, 0x82f3bc68
	if !ctx.cr[6].eq {
	pc = 0x82F3BC68; continue 'dispatch;
	}
	// 82F3BC4C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BC50: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F3BC54: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F3BC58: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3BC5C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F3BC60: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F3BC64: 4BF64B4D  bl 0x82ea07b0
	ctx.lr = 0x82F3BC68;
	sub_82EA07B0(ctx, base);
	// 82F3BC68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3BC6C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F3BC70: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F3BC74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F3BC78: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3BC7C: 419A0020  beq cr6, 0x82f3bc9c
	if ctx.cr[6].eq {
	pc = 0x82F3BC9C; continue 'dispatch;
	}
	// 82F3BC80: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BC84: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3BC88: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F3BC8C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3BC90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F3BC94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3BC98: 4BF64B19  bl 0x82ea07b0
	ctx.lr = 0x82F3BC9C;
	sub_82EA07B0(ctx, base);
	// 82F3BC9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3BCA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3BCAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3BCB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3BCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BCB8 size=100
    let mut pc: u32 = 0x82F3BCB8;
    'dispatch: loop {
        match pc {
            0x82F3BCB8 => {
    //   block [0x82F3BCB8..0x82F3BD1C)
	// 82F3BCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3BCC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3BCC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BCC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3BCCC: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BCD0: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82F3BCD4: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82F3BCD8: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82F3BCDC: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F3BCE0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3BCE4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82F3BCE8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BCEC: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3BCF0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3BCF4: 4E800421  bctrl
	ctx.lr = 0x82F3BCF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3BCF8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82F3BCFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3BD00: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F3BD04: 48042345  bl 0x82f7e048
	ctx.lr = 0x82F3BD08;
	sub_82F7E048(ctx, base);
	// 82F3BD08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3BD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3BD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3BD14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3BD20 size=92
    let mut pc: u32 = 0x82F3BD20;
    'dispatch: loop {
        match pc {
            0x82F3BD20 => {
    //   block [0x82F3BD20..0x82F3BD7C)
	// 82F3BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3BD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BD2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3BD30: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3BD34: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3BD38: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F3BD3C: 390AF3E8  addi r8, r10, -0xc18
	ctx.r[8].s64 = ctx.r[10].s64 + -3096;
	// 82F3BD40: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F3BD44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3BD48: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3BD4C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3BD50: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3BD54: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82F3BD58: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82F3BD5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F3BD60: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3BD64: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F3BD68: 4800AA79  bl 0x82f467e0
	ctx.lr = 0x82F3BD6C;
	sub_82F467E0(ctx, base);
	// 82F3BD6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3BD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3BD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3BD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BD80 size=88
    let mut pc: u32 = 0x82F3BD80;
    'dispatch: loop {
        match pc {
            0x82F3BD80 => {
    //   block [0x82F3BD80..0x82F3BDD8)
	// 82F3BD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3BD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BD8C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3BD90: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3BD94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3BD98: 392AF400  addi r9, r10, -0xc00
	ctx.r[9].s64 = ctx.r[10].s64 + -3072;
	// 82F3BD9C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F3BDA0: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82F3BDA4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F3BDA8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3BDAC: 99610064  stb r11, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u8 ) };
	// 82F3BDB0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F3BDB4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82F3BDB8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3BDBC: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82F3BDC0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F3BDC4: 4800AFB5  bl 0x82f46d78
	ctx.lr = 0x82F3BDC8;
	sub_82F46D78(ctx, base);
	// 82F3BDC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3BDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3BDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3BDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3BDD8 size=176
    let mut pc: u32 = 0x82F3BDD8;
    'dispatch: loop {
        match pc {
            0x82F3BDD8 => {
    //   block [0x82F3BDD8..0x82F3BE88)
	// 82F3BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3BDE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3BDE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3BDE8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F3BDEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BDF0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3BDF4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3BDF8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3BDFC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3BE00: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BE04: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3BE08: 4BFFF921  bl 0x82f3b728
	ctx.lr = 0x82F3BE0C;
	sub_82F3B728(ctx, base);
	// 82F3BE0C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3BE10: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F3BE14: 40980044  bge cr6, 0x82f3be58
	if !ctx.cr[6].lt {
	pc = 0x82F3BE58; continue 'dispatch;
	}
	// 82F3BE18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3BE1C: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F3BE20: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BE88 size=324
    let mut pc: u32 = 0x82F3BE88;
    'dispatch: loop {
        match pc {
            0x82F3BE88 => {
    //   block [0x82F3BE88..0x82F3BFCC)
	// 82F3BE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BE8C: 4826C2E1  bl 0x831a816c
	ctx.lr = 0x82F3BE90;
	sub_831A8130(ctx, base);
	// 82F3BE90: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BE94: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3BE98: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F3BE9C: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 82F3BEA0: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F3BEA4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F3BEA8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3BEAC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3BEB0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F3BEB4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F3BEB8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F3BEBC: 4200FFF0  bdnz 0x82f3beac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3BEAC; continue 'dispatch;
	}
	// 82F3BEC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3BEC4: E8A60050  ld r5, 0x50(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F3BEC8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F3BECC: E8860058  ld r4, 0x58(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F3BED0: 392AC5B0  addi r9, r10, -0x3a50
	ctx.r[9].s64 = ctx.r[10].s64 + -14928;
	// 82F3BED4: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82F3BED8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3BEDC: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3BFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3BFD0 size=124
    let mut pc: u32 = 0x82F3BFD0;
    'dispatch: loop {
        match pc {
            0x82F3BFD0 => {
    //   block [0x82F3BFD0..0x82F3C04C)
	// 82F3BFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3BFD4: 4826C195  bl 0x831a8168
	ctx.lr = 0x82F3BFD8;
	sub_831A8130(ctx, base);
	// 82F3BFD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3BFDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3BFE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3BFE4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3BFE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F3BFEC: 40990040  ble cr6, 0x82f3c02c
	if !ctx.cr[6].gt {
	pc = 0x82F3C02C; continue 'dispatch;
	}
	// 82F3BFF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F3BFF4: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82F3BFF8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3BFFC: 7C8BF22E  lhzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F3C000: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82F3C004: 419A001C  beq cr6, 0x82f3c020
	if ctx.cr[6].eq {
	pc = 0x82F3C020; continue 'dispatch;
	}
	// 82F3C008: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C00C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F3C010: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C014: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3C018: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3C01C: 4E800421  bctrl
	ctx.lr = 0x82F3C020;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3C020: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F3C024: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82F3C028: 4082FFD0  bne 0x82f3bff8
	if !ctx.cr[0].eq {
	pc = 0x82F3BFF8; continue 'dispatch;
	}
	// 82F3C02C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C030: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3C034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3C038: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C03C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3C040: 4E800421  bctrl
	ctx.lr = 0x82F3C044;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3C044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3C048: 4826C170  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3C050 size=636
    let mut pc: u32 = 0x82F3C050;
    'dispatch: loop {
        match pc {
            0x82F3C050 => {
    //   block [0x82F3C050..0x82F3C2CC)
	// 82F3C050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C054: 4826C0E9  bl 0x831a813c
	ctx.lr = 0x82F3C058;
	sub_831A8130(ctx, base);
	// 82F3C058: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82F3C05C: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C060: 824D0000  lwz r18, 0(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C064: 3A200018  li r17, 0x18
	ctx.r[17].s64 = 24;
	// 82F3C068: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3C06C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F3C070: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F3C074: 7D71902E  lwzx r11, r17, r18
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82F3C078: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3C07C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3C080: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3C084: 40980020  bge cr6, 0x82f3c0a4
	if !ctx.cr[6].lt {
	pc = 0x82F3C0A4; continue 'dispatch;
	}
	// 82F3C088: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3C08C: 3909F938  addi r8, r9, -0x6c8
	ctx.r[8].s64 = ctx.r[9].s64 + -1736;
	// 82F3C090: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3C094: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3C098: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3C09C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3C0A0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3C0A4: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82F3C0A8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C0AC: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C0B0: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C0B4: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C0B8: 4BF64FA9  bl 0x82ea1060
	ctx.lr = 0x82F3C0BC;
	sub_82EA1060(ctx, base);
	// 82F3C0BC: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3C0C0: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82F3C0C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F3C0C8: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3C0CC: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82F3C0D0: 938100B4  stw r28, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 82F3C0D4: 92A10080  stw r21, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 82F3C0D8: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 82F3C0DC: 409901B4  ble cr6, 0x82f3c290
	if !ctx.cr[6].gt {
	pc = 0x82F3C290; continue 'dispatch;
	}
	// 82F3C0E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F3C0E4: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82F3C0E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F3C0EC: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 82F3C0F0: 3B00FFFF  li r24, -1
	ctx.r[24].s64 = -1;
	// 82F3C0F4: C3EA08A8  lfs f31, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3C0F8: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82F3C0FC: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82F3C100: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82F3C104: 3B4BFFA0  addi r26, r11, -0x60
	ctx.r[26].s64 = ctx.r[11].s64 + -96;
	// 82F3C108: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 82F3C10C: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 82F3C110: 928100D4  stw r20, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 82F3C114: 39410120  addi r10, r1, 0x120
	ctx.r[10].s64 = ctx.r[1].s64 + 288;
	// 82F3C118: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 82F3C11C: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82F3C120: 930100E0  stw r24, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[24].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3C2D0 size=636
    let mut pc: u32 = 0x82F3C2D0;
    'dispatch: loop {
        match pc {
            0x82F3C2D0 => {
    //   block [0x82F3C2D0..0x82F3C54C)
	// 82F3C2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C2D4: 4826BE69  bl 0x831a813c
	ctx.lr = 0x82F3C2D8;
	sub_831A8130(ctx, base);
	// 82F3C2D8: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82F3C2DC: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C2E0: 824D0000  lwz r18, 0(r13)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C2E4: 3A200018  li r17, 0x18
	ctx.r[17].s64 = 24;
	// 82F3C2E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3C2EC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3C2F0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F3C2F4: 7D71902E  lwzx r11, r17, r18
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82F3C2F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3C2FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3C300: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3C304: 40980020  bge cr6, 0x82f3c324
	if !ctx.cr[6].lt {
	pc = 0x82F3C324; continue 'dispatch;
	}
	// 82F3C308: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3C30C: 3909F938  addi r8, r9, -0x6c8
	ctx.r[8].s64 = ctx.r[9].s64 + -1736;
	// 82F3C310: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3C314: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3C318: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3C31C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3C320: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3C324: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82F3C328: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C32C: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C330: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C334: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C338: 4BF64D29  bl 0x82ea1060
	ctx.lr = 0x82F3C33C;
	sub_82EA1060(ctx, base);
	// 82F3C33C: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3C340: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82F3C344: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F3C348: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3C34C: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82F3C350: 938100B4  stw r28, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 82F3C354: 92A10080  stw r21, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 82F3C358: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 82F3C35C: 409901B4  ble cr6, 0x82f3c510
	if !ctx.cr[6].gt {
	pc = 0x82F3C510; continue 'dispatch;
	}
	// 82F3C360: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F3C364: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82F3C368: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F3C36C: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 82F3C370: 3B00FFFF  li r24, -1
	ctx.r[24].s64 = -1;
	// 82F3C374: C3EA08A8  lfs f31, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3C378: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82F3C37C: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82F3C380: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82F3C384: 3B4BFFA0  addi r26, r11, -0x60
	ctx.r[26].s64 = ctx.r[11].s64 + -96;
	// 82F3C388: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 82F3C38C: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 82F3C390: 928100D4  stw r20, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 82F3C394: 39410120  addi r10, r1, 0x120
	ctx.r[10].s64 = ctx.r[1].s64 + 288;
	// 82F3C398: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 82F3C39C: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82F3C3A0: 930100E0  stw r24, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[24].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3C550 size=436
    let mut pc: u32 = 0x82F3C550;
    'dispatch: loop {
        match pc {
            0x82F3C550 => {
    //   block [0x82F3C550..0x82F3C704)
	// 82F3C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C554: 4826BC01  bl 0x831a8154
	ctx.lr = 0x82F3C558;
	sub_831A8130(ctx, base);
	// 82F3C558: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C55C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C560: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F3C564: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F3C568: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3C56C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F3C570: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F3C574: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3C578: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3C57C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3C580: 40980020  bge cr6, 0x82f3c5a0
	if !ctx.cr[6].lt {
	pc = 0x82F3C5A0; continue 'dispatch;
	}
	// 82F3C584: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3C588: 3909F948  addi r8, r9, -0x6b8
	ctx.r[8].s64 = ctx.r[9].s64 + -1720;
	// 82F3C58C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3C590: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3C594: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3C598: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3C59C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3C5A0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82F3C5A4: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C5A8: 835C0000  lwz r26, 0(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C5AC: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C5B0: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3C5B4: 4BF64AAD  bl 0x82ea1060
	ctx.lr = 0x82F3C5B8;
	sub_82EA1060(ctx, base);
	// 82F3C5B8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F3C5BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3C5C0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F3C5C4: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82F3C5C8: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3C5CC: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3C5D0: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3C5D4: D00100E0  stfs f0, 0xe0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 82F3C5D8: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82F3C5DC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F3C5E0: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82F3C5E4: 912100E4  stw r9, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 82F3C5E8: 91610110  stw r11, 0x110(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 82F3C5EC: 910100F0  stw r8, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[8].u32 ) };
	// 82F3C5F0: 419A00DC  beq cr6, 0x82f3c6cc
	if ctx.cr[6].eq {
	pc = 0x82F3C6CC; continue 'dispatch;
	}
	// 82F3C5F4: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82F3C5F8: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3C5FC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82F3C600: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F3C604: 409A00C8  bne cr6, 0x82f3c6cc
	if !ctx.cr[6].eq {
	pc = 0x82F3C6CC; continue 'dispatch;
	}
	// 82F3C608: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3C708 size=20
    let mut pc: u32 = 0x82F3C708;
    'dispatch: loop {
        match pc {
            0x82F3C708 => {
    //   block [0x82F3C708..0x82F3C71C)
	// 82F3C708: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3C70C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F3C710: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3C714: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F3C718: 4BFFFE38  b 0x82f3c550
	sub_82F3C550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3C720 size=188
    let mut pc: u32 = 0x82F3C720;
    'dispatch: loop {
        match pc {
            0x82F3C720 => {
    //   block [0x82F3C720..0x82F3C7DC)
	// 82F3C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C724: 4826BA49  bl 0x831a816c
	ctx.lr = 0x82F3C728;
	sub_831A8130(ctx, base);
	// 82F3C728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C72C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F3C730: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3C734: 3BFD000C  addi r31, r29, 0xc
	ctx.r[31].s64 = ctx.r[29].s64 + 12;
	// 82F3C738: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F3C73C: 392BF964  addi r9, r11, -0x69c
	ctx.r[9].s64 = ctx.r[11].s64 + -1692;
	// 82F3C740: 90FD0008  stw r7, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F3C744: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F3C748: 61460004  ori r6, r10, 4
	ctx.r[6].u64 = ctx.r[10].u64 | 4;
	// 82F3C74C: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3C750: 38FF000C  addi r7, r31, 0xc
	ctx.r[7].s64 = ctx.r[31].s64 + 12;
	// 82F3C754: B11D0006  sth r8, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F3C758: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F3C75C: 90DD0014  stw r6, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82F3C760: 90FD000C  stw r7, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F3C764: 54C3003E  slwi r3, r6, 0
	ctx.r[3].u32 = ctx.r[6].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F3C768: 90BD0010  stw r5, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F3C76C: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C770: 546B00BE  clrlwi r11, r3, 2
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x3FFFFFFFu64;
	// 82F3C774: 83C40014  lwz r30, 0x14(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3C778: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F3C77C: 40980024  bge cr6, 0x82f3c7a0
	if !ctx.cr[6].lt {
	pc = 0x82F3C7A0; continue 'dispatch;
	}
	// 82F3C780: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F3C784: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F3C788: 41980008  blt cr6, 0x82f3c790
	if ctx.cr[6].lt {
	pc = 0x82F3C790; continue 'dispatch;
	}
	// 82F3C78C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82F3C790: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F3C794: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3C798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3C79C: 4BF6A05D  bl 0x82ea67f8
	ctx.lr = 0x82F3C7A0;
	sub_82EA67F8(ctx, base);
	// 82F3C7A0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82F3C7A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F3C7A8: 40990028  ble cr6, 0x82f3c7d0
	if !ctx.cr[6].gt {
	pc = 0x82F3C7D0; continue 'dispatch;
	}
	// 82F3C7AC: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82F3C7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F3C7B4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82F3C7B8: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82F3C7BC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C7C0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F3C7C4: 7D28532E  sthx r9, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 82F3C7C8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82F3C7CC: 4082FFF0  bne 0x82f3c7bc
	if !ctx.cr[0].eq {
	pc = 0x82F3C7BC; continue 'dispatch;
	}
	// 82F3C7D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F3C7D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3C7D8: 4826B9E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3C7E0 size=88
    let mut pc: u32 = 0x82F3C7E0;
    'dispatch: loop {
        match pc {
            0x82F3C7E0 => {
    //   block [0x82F3C7E0..0x82F3C838)
	// 82F3C7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C7E4: 4826B985  bl 0x831a8168
	ctx.lr = 0x82F3C7E8;
	sub_831A8130(ctx, base);
	// 82F3C7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C7EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C7F0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3C7F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3C7F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F3C7FC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3C800: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82F3C804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3C808: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3C80C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F3C810: 4BF63F21  bl 0x82ea0730
	ctx.lr = 0x82F3C814;
	sub_82EA0730(ctx, base);
	// 82F3C814: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82F3C818: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3C81C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3C820: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F3C824: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F3C828: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F3C82C: 4BFFFEF5  bl 0x82f3c720
	ctx.lr = 0x82F3C830;
	sub_82F3C720(ctx, base);
	// 82F3C830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3C834: 4826B984  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3C838 size=108
    let mut pc: u32 = 0x82F3C838;
    'dispatch: loop {
        match pc {
            0x82F3C838 => {
    //   block [0x82F3C838..0x82F3C8A4)
	// 82F3C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C83C: 4826B929  bl 0x831a8164
	ctx.lr = 0x82F3C840;
	sub_831A8130(ctx, base);
	// 82F3C840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C844: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3C848: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3C84C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3C850: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F3C854: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3C858: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82F3C85C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3C860: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3C864: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F3C868: 4BF63EC9  bl 0x82ea0730
	ctx.lr = 0x82F3C86C;
	sub_82EA0730(ctx, base);
	// 82F3C86C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F3C870: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82F3C874: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F3C878: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F3C87C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F3C880: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F3C884: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F3C888: 4BFFFE99  bl 0x82f3c720
	ctx.lr = 0x82F3C88C;
	sub_82F3C720(ctx, base);
	// 82F3C88C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3C890: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F3C894: 38E8F9A0  addi r7, r8, -0x660
	ctx.r[7].s64 = ctx.r[8].s64 + -1632;
	// 82F3C898: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3C89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3C8A0: 4826B914  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3C8A8 size=204
    let mut pc: u32 = 0x82F3C8A8;
    'dispatch: loop {
        match pc {
            0x82F3C8A8 => {
    //   block [0x82F3C8A8..0x82F3C974)
	// 82F3C8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3C8B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3C8B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3C8B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C8BC: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3C8C0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3C8C4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3C8C8: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3C8CC: 38CAC978  addi r6, r10, -0x3688
	ctx.r[6].s64 = ctx.r[10].s64 + -13960;
	// 82F3C8D0: 38A9C9C0  addi r5, r9, -0x3640
	ctx.r[5].s64 = ctx.r[9].s64 + -13888;
	// 82F3C8D4: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F3C8D8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3C8DC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F3C8E0: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3C8E4: 38EBC838  addi r7, r11, -0x37c8
	ctx.r[7].s64 = ctx.r[11].s64 + -14280;
	// 82F3C8E8: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3C8EC: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F3C8F0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F3C8F4: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3C8F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F3C8FC: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F3C900: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3C904: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3C908: 4BFE8B39  bl 0x82f25440
	ctx.lr = 0x82F3C90C;
	sub_82F25440(ctx, base);
	// 82F3C90C: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F3C910: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3C914: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F3C918: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3C91C: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F3C920: 3903C7E0  addi r8, r3, -0x3820
	ctx.r[8].s64 = ctx.r[3].s64 + -14368;
	// 82F3C924: 38CAC2D0  addi r6, r10, -0x3d30
	ctx.r[6].s64 = ctx.r[10].s64 + -15664;
	// 82F3C928: 38A9BB88  addi r5, r9, -0x4478
	ctx.r[5].s64 = ctx.r[9].s64 + -17528;
	// 82F3C92C: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F3C930: 38EBC550  addi r7, r11, -0x3ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -15024;
	// 82F3C934: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F3C938: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F3C93C: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F3C940: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F3C944: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F3C948: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F3C94C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82F3C950: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3C954: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3C958: 4BFE8AE9  bl 0x82f25440
	ctx.lr = 0x82F3C95C;
	sub_82F25440(ctx, base);
	// 82F3C95C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F3C960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3C964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3C968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3C96C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3C970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3C978 size=72
    let mut pc: u32 = 0x82F3C978;
    'dispatch: loop {
        match pc {
            0x82F3C978 => {
    //   block [0x82F3C978..0x82F3C9C0)
	// 82F3C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3C980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C984: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3C988: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3C98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3C990: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3C994: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3C998: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3C99C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3C9A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3C9A4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3C9A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3C9AC: 4BFFFBA5  bl 0x82f3c550
	ctx.lr = 0x82F3C9B0;
	sub_82F3C550(ctx, base);
	// 82F3C9B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3C9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3C9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3C9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3C9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3C9C0 size=76
    let mut pc: u32 = 0x82F3C9C0;
    'dispatch: loop {
        match pc {
            0x82F3C9C0 => {
    //   block [0x82F3C9C0..0x82F3CA0C)
	// 82F3C9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3C9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3C9C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3C9CC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3C9D0: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3C9D4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3C9D8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3C9DC: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3C9E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3C9E4: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3C9E8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3C9EC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3C9F0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3C9F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3C9F8: 4BFFF8D9  bl 0x82f3c2d0
	ctx.lr = 0x82F3C9FC;
	sub_82F3C2D0(ctx, base);
	// 82F3C9FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3CA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3CA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3CA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3CA10 size=716
    let mut pc: u32 = 0x82F3CA10;
    'dispatch: loop {
        match pc {
            0x82F3CA10 => {
    //   block [0x82F3CA10..0x82F3CCDC)
	// 82F3CA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CA14: 4826B71D  bl 0x831a8130
	ctx.lr = 0x82F3CA18;
	sub_831A8130(ctx, base);
	// 82F3CA18: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82F3CA1C: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CA20: 81CD0000  lwz r14, 0(r13)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CA24: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82F3CA28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F3CA2C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82F3CA30: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F3CA34: 7CCF3378  mr r15, r6
	ctx.r[15].u64 = ctx.r[6].u64;
	// 82F3CA38: 7D6B702E  lwzx r11, r11, r14
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F3CA3C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F3CA40: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3CA44: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3CA48: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3CA4C: 40980020  bge cr6, 0x82f3ca6c
	if !ctx.cr[6].lt {
	pc = 0x82F3CA6C; continue 'dispatch;
	}
	// 82F3CA50: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3CA54: 3909F938  addi r8, r9, -0x6c8
	ctx.r[8].s64 = ctx.r[9].s64 + -1736;
	// 82F3CA58: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3CA5C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3CA60: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3CA64: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3CA68: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3CA6C: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82F3CA70: 83380000  lwz r25, 0(r24)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CA74: 82BB0000  lwz r21, 0(r27)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CA78: 80B80008  lwz r5, 8(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3CA7C: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3CA80: 4BF645E1  bl 0x82ea1060
	ctx.lr = 0x82F3CA84;
	sub_82EA1060(ctx, base);
	// 82F3CA84: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82F3CA88: 81790014  lwz r11, 0x14(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3CA8C: 92810080  stw r20, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[20].u32 ) };
	// 82F3CA90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F3CA94: 92810084  stw r20, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[20].u32 ) };
	// 82F3CA98: 83590010  lwz r26, 0x10(r25)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F3CA9C: 40990200  ble cr6, 0x82f3cc9c
	if !ctx.cr[6].gt {
	pc = 0x82F3CC9C; continue 'dispatch;
	}
	// 82F3CAA0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F3CAA4: 7D705B78  mr r16, r11
	ctx.r[16].u64 = ctx.r[11].u64;
	// 82F3CAA8: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82F3CAAC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F3CAB0: 7E9EA378  mr r30, r20
	ctx.r[30].u64 = ctx.r[20].u64;
	// 82F3CAB4: C3EA08A8  lfs f31, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3CAB8: 3A200010  li r17, 0x10
	ctx.r[17].s64 = 16;
	// 82F3CABC: 3A40FFFF  li r18, -1
	ctx.r[18].s64 = -1;
	// 82F3CAC0: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 82F3CAC4: 6133FFFF  ori r19, r9, 0xffff
	ctx.r[19].u64 = ctx.r[9].u64 | 65535;
	// 82F3CAC8: 3AEBFFA0  addi r23, r11, -0x60
	ctx.r[23].s64 = ctx.r[11].s64 + -96;
	// 82F3CACC: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3CCE0 size=76
    let mut pc: u32 = 0x82F3CCE0;
    'dispatch: loop {
        match pc {
            0x82F3CCE0 => {
    //   block [0x82F3CCE0..0x82F3CD2C)
	// 82F3CCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3CCE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CCEC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3CCF0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3CCF4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3CCF8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3CCFC: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3CD00: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3CD04: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3CD08: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3CD0C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3CD10: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3CD14: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3CD18: 4BFFF339  bl 0x82f3c050
	ctx.lr = 0x82F3CD1C;
	sub_82F3C050(ctx, base);
	// 82F3CD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3CD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3CD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3CD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3CD30 size=68
    let mut pc: u32 = 0x82F3CD30;
    'dispatch: loop {
        match pc {
            0x82F3CD30 => {
    //   block [0x82F3CD30..0x82F3CD74)
	// 82F3CD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3CD38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CD3C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3CD40: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3CD44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F3CD48: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F3CD4C: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F3CD50: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F3CD54: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F3CD58: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3CD5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3CD60: 4BFFF7F1  bl 0x82f3c550
	ctx.lr = 0x82F3CD64;
	sub_82F3C550(ctx, base);
	// 82F3CD64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3CD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3CD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3CD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3CD78 size=176
    let mut pc: u32 = 0x82F3CD78;
    'dispatch: loop {
        match pc {
            0x82F3CD78 => {
    //   block [0x82F3CD78..0x82F3CE28)
	// 82F3CD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3CD80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3CD84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3CD88: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F3CD8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CD90: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3CD94: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3CD98: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3CD9C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3CDA0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CDA4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3CDA8: 4BFFFC69  bl 0x82f3ca10
	ctx.lr = 0x82F3CDAC;
	sub_82F3CA10(ctx, base);
	// 82F3CDAC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CDB0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F3CDB4: 40980044  bge cr6, 0x82f3cdf8
	if !ctx.cr[6].lt {
	pc = 0x82F3CDF8; continue 'dispatch;
	}
	// 82F3CDB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3CDBC: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F3CDC0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3CE28 size=152
    let mut pc: u32 = 0x82F3CE28;
    'dispatch: loop {
        match pc {
            0x82F3CE28 => {
    //   block [0x82F3CE28..0x82F3CEC0)
	// 82F3CE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3CE30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3CE34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3CE38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CE3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3CE40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3CE44: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3CE48: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F3CE4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F3CE50: 409A0020  bne cr6, 0x82f3ce70
	if !ctx.cr[6].eq {
	pc = 0x82F3CE70; continue 'dispatch;
	}
	// 82F3CE54: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CE58: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F3CE5C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F3CE60: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3CE64: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F3CE68: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F3CE6C: 4BF63945  bl 0x82ea07b0
	ctx.lr = 0x82F3CE70;
	sub_82EA07B0(ctx, base);
	// 82F3CE70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3CE74: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F3CE78: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F3CE7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F3CE80: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F3CE84: 419A0020  beq cr6, 0x82f3cea4
	if ctx.cr[6].eq {
	pc = 0x82F3CEA4; continue 'dispatch;
	}
	// 82F3CE88: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CE8C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3CE90: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F3CE94: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3CE98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F3CE9C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3CEA0: 4BF63911  bl 0x82ea07b0
	ctx.lr = 0x82F3CEA4;
	sub_82EA07B0(ctx, base);
	// 82F3CEA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3CEA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3CEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3CEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3CEB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3CEB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3CEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3CEC0 size=224
    let mut pc: u32 = 0x82F3CEC0;
    'dispatch: loop {
        match pc {
            0x82F3CEC0 => {
    //   block [0x82F3CEC0..0x82F3CFA0)
	// 82F3CEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CEC4: 4826B2A5  bl 0x831a8168
	ctx.lr = 0x82F3CEC8;
	sub_831A8130(ctx, base);
	// 82F3CEC8: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CECC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3CED0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F3CED4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F3CED8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F3CEDC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F3CEE0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F3CEE4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F3CEE8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3CEEC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F3CEF0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F3CEF4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F3CEF8: 4200FFF0  bdnz 0x82f3cee8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3CEE8; continue 'dispatch;
	}
	// 82F3CEFC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3CF00: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F3CF04: E9060058  ld r8, 0x58(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F3CF08: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F3CF0C: 38CAC5B0  addi r6, r10, -0x3a50
	ctx.r[6].s64 = ctx.r[10].s64 + -14928;
	// 82F3CF10: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F3CF14: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3CF18: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3CFA0 size=136
    let mut pc: u32 = 0x82F3CFA0;
    'dispatch: loop {
        match pc {
            0x82F3CFA0 => {
    //   block [0x82F3CFA0..0x82F3D028)
	// 82F3CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3CFA4: 4826B1C9  bl 0x831a816c
	ctx.lr = 0x82F3CFA8;
	sub_831A8130(ctx, base);
	// 82F3CFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3CFAC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3CFB0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3CFB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3CFB8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3CFBC: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82F3CFC0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3CFC4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3CFC8: 4BF63769  bl 0x82ea0730
	ctx.lr = 0x82F3CFCC;
	sub_82EA0730(ctx, base);
	// 82F3CFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3CFD0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3CFD4: 3900002C  li r8, 0x2c
	ctx.r[8].s64 = 44;
	// 82F3CFD8: 38E9F9DC  addi r7, r9, -0x624
	ctx.r[7].s64 = ctx.r[9].s64 + -1572;
	// 82F3CFDC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F3CFE0: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82F3CFE4: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F3CFE8: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F3CFEC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F3CFF0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F3CFF4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82F3CFF8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3CFFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3D000: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82F3D004: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82F3D008: 4200FFF8  bdnz 0x82f3d000
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3D000; continue 'dispatch;
	}
	// 82F3D00C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D010: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82F3D014: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F3D018: 4804AED1  bl 0x82f87ee8
	ctx.lr = 0x82F3D01C;
	sub_82F87EE8(ctx, base);
	// 82F3D01C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3D020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3D024: 4826B198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3D028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3D028 size=116
    let mut pc: u32 = 0x82F3D028;
    'dispatch: loop {
        match pc {
            0x82F3D028 => {
    //   block [0x82F3D028..0x82F3D09C)
	// 82F3D028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3D02C: 4826B13D  bl 0x831a8168
	ctx.lr = 0x82F3D030;
	sub_831A8130(ctx, base);
	// 82F3D030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3D034: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3D038: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3D03C: 3BFE001C  addi r31, r30, 0x1c
	ctx.r[31].s64 = ctx.r[30].s64 + 28;
	// 82F3D040: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82F3D044: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D048: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82F3D04C: 419A001C  beq cr6, 0x82f3d068
	if ctx.cr[6].eq {
	pc = 0x82F3D068; continue 'dispatch;
	}
	// 82F3D050: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3D054: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F3D058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D05C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3D060: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3D064: 4E800421  bctrl
	ctx.lr = 0x82F3D068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3D068: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F3D06C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82F3D070: 4082FFD4  bne 0x82f3d044
	if !ctx.cr[0].eq {
	pc = 0x82F3D044; continue 'dispatch;
	}
	// 82F3D074: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82F3D078: 419A001C  beq cr6, 0x82f3d094
	if ctx.cr[6].eq {
	pc = 0x82F3D094; continue 'dispatch;
	}
	// 82F3D07C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D080: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3D084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3D088: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D08C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3D090: 4E800421  bctrl
	ctx.lr = 0x82F3D094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3D094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3D098: 4826B120  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3D0A0 size=148
    let mut pc: u32 = 0x82F3D0A0;
    'dispatch: loop {
        match pc {
            0x82F3D0A0 => {
    //   block [0x82F3D0A0..0x82F3D134)
	// 82F3D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3D0A4: 4826B0C9  bl 0x831a816c
	ctx.lr = 0x82F3D0A8;
	sub_831A8130(ctx, base);
	// 82F3D0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3D0AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D0B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3D0B4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3D0B8: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82F3D0BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3D0C0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3D0C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3D0C8: 4BF63669  bl 0x82ea0730
	ctx.lr = 0x82F3D0CC;
	sub_82EA0730(ctx, base);
	// 82F3D0CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3D0D0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3D0D4: 3900002C  li r8, 0x2c
	ctx.r[8].s64 = 44;
	// 82F3D0D8: 38E9F9DC  addi r7, r9, -0x624
	ctx.r[7].s64 = ctx.r[9].s64 + -1572;
	// 82F3D0DC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F3D0E0: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82F3D0E4: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F3D0E8: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F3D0EC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F3D0F0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F3D0F4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82F3D0F8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3D0FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3D100: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82F3D104: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82F3D108: 4200FFF8  bdnz 0x82f3d100
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F3D100; continue 'dispatch;
	}
	// 82F3D10C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D110: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82F3D114: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F3D118: 4804ADD1  bl 0x82f87ee8
	ctx.lr = 0x82F3D11C;
	sub_82F87EE8(ctx, base);
	// 82F3D11C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F3D120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3D124: 394BFA18  addi r10, r11, -0x5e8
	ctx.r[10].s64 = ctx.r[11].s64 + -1512;
	// 82F3D128: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F3D12C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3D130: 4826B08C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3D138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3D138 size=560
    let mut pc: u32 = 0x82F3D138;
    'dispatch: loop {
        match pc {
            0x82F3D138 => {
    //   block [0x82F3D138..0x82F3D368)
	// 82F3D138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3D13C: 4826B009  bl 0x831a8144
	ctx.lr = 0x82F3D140;
	sub_831A8130(ctx, base);
	// 82F3D140: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82F3D144: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3D148: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82F3D14C: 3BA100A0  addi r29, r1, 0xa0
	ctx.r[29].s64 = ctx.r[1].s64 + 160;
	// 82F3D150: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F3D154: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F3D158: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F3D15C: 83380000  lwz r25, 0(r24)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D160: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3D164: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3D168: 3BE10080  addi r31, r1, 0x80
	ctx.r[31].s64 = ctx.r[1].s64 + 128;
	// 82F3D16C: 39390020  addi r9, r25, 0x20
	ctx.r[9].s64 = ctx.r[25].s64 + 32;
	// 82F3D170: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3D174: 39490020  addi r10, r9, 0x20
	ctx.r[10].s64 = ctx.r[9].s64 + 32;
	// 82F3D178: 7D29E850  subf r9, r9, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82F3D17C: EBCB0000  ld r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3D180: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82F3D184: EB8B0008  ld r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3D188: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82F3D18C: EB6B0010  ld r27, 0x10(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3D190: EACB0018  ld r22, 0x18(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3D194: EAAB0020  ld r21, 0x20(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3D198: EBAB0028  ld r29, 0x28(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3D19C: EA8B0030  ld r20, 0x30(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3D1A0: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3D1A4: FBC60000  std r30, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82F3D1A8: FB860008  std r28, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[28].u64 ) };
	// 82F3D1AC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F3D1B0: FB650000  std r27, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82F3D1B4: FAC50008  std r22, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3D1B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F3D1BC: FAA40000  std r21, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82F3D1C0: FBA40008  std r29, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 82F3D1C4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3D1C8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3D1CC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F3D1D0: FA9F0000  std r20, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3D368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3D368 size=564
    let mut pc: u32 = 0x82F3D368;
    'dispatch: loop {
        match pc {
            0x82F3D368 => {
    //   block [0x82F3D368..0x82F3D59C)
	// 82F3D368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3D36C: 4826ADD9  bl 0x831a8144
	ctx.lr = 0x82F3D370;
	sub_831A8130(ctx, base);
	// 82F3D370: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82F3D374: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3D378: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F3D37C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82F3D380: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F3D384: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F3D388: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3D38C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3D390: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82F3D394: 833A0000  lwz r25, 0(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D398: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F3D39C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3D3A0: 38790020  addi r3, r25, 0x20
	ctx.r[3].s64 = ctx.r[25].s64 + 32;
	// 82F3D3A4: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82F3D3A8: EBCB0000  ld r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3D3AC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82F3D3B0: EBAB0008  ld r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3D3B4: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F3D3B8: EB8B0010  ld r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3D3BC: 7CE33850  subf r7, r3, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[3].s64;
	// 82F3D3C0: EAEB0018  ld r23, 0x18(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3D3C4: EACB0020  ld r22, 0x20(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3D3C8: EAAB0028  ld r21, 0x28(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3D3CC: EA8B0030  ld r20, 0x30(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3D3D0: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3D3D4: FBC60000  std r30, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82F3D3D8: FBA60008  std r29, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 82F3D3DC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F3D3E0: FB850000  std r28, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 82F3D3E4: FAE50008  std r23, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[23].u64 ) };
	// 82F3D3E8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3D3EC: FAC40000  std r22, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82F3D3F0: FAA40008  std r21, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82F3D3F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3D3F8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3D3FC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F3D400: FA9F0000  std r20, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3D5A0 size=720
    let mut pc: u32 = 0x82F3D5A0;
    'dispatch: loop {
        match pc {
            0x82F3D5A0 => {
    //   block [0x82F3D5A0..0x82F3D870)
	// 82F3D5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3D5A4: 4826AB95  bl 0x831a8138
	ctx.lr = 0x82F3D5A8;
	sub_831A8130(ctx, base);
	// 82F3D5A8: DBE1FF70  stfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82F3D5AC: 9421FDD0  stwu r1, -0x230(r1)
	ea = ctx.r[1].u32.wrapping_add(-560 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3D5B0: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D5B4: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F3D5B8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F3D5BC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F3D5C0: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F3D5C4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3D5C8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3D5CC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3D5D0: 40980020  bge cr6, 0x82f3d5f0
	if !ctx.cr[6].lt {
	pc = 0x82F3D5F0; continue 'dispatch;
	}
	// 82F3D5D4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3D5D8: 3909FA64  addi r8, r9, -0x59c
	ctx.r[8].s64 = ctx.r[9].s64 + -1436;
	// 82F3D5DC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3D5E0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3D5E4: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F3D5E8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3D5EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F3D5F0: 83650000  lwz r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D5F4: 3AC100F0  addi r22, r1, 0xf0
	ctx.r[22].s64 = ctx.r[1].s64 + 240;
	// 82F3D5F8: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3D5FC: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82F3D600: 391B0020  addi r8, r27, 0x20
	ctx.r[8].s64 = ctx.r[27].s64 + 32;
	// 82F3D604: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D608: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3D60C: 39480020  addi r10, r8, 0x20
	ctx.r[10].s64 = ctx.r[8].s64 + 32;
	// 82F3D610: 7D08B050  subf r8, r8, r22
	ctx.r[8].s64 = ctx.r[22].s64 - ctx.r[8].s64;
	// 82F3D614: EB8B0000  ld r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3D618: 3BE10090  addi r31, r1, 0x90
	ctx.r[31].s64 = ctx.r[1].s64 + 144;
	// 82F3D61C: EAAB0008  ld r21, 8(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3D620: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 82F3D624: EA8B0010  ld r20, 0x10(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3D628: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82F3D62C: EA6B0018  ld r19, 0x18(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3D630: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82F3D634: EA4B0020  ld r18, 0x20(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3D638: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3D63C: EA2B0030  ld r17, 0x30(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3D640: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3D644: FB870000  std r28, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 82F3D648: FAA70008  std r21, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82F3D64C: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82F3D650: FA9F0000  std r20, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82F3D654: FA7F0008  std r19, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 82F3D658: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 82F3D65C: FA5E0000  std r18, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82F3D660: FADE0008  std r22, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3D664: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 82F3D668: F97D0008  std r11, 8(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3D66C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82F3D670: FA3D0000  std r17, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3D870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3D870 size=728
    let mut pc: u32 = 0x82F3D870;
    'dispatch: loop {
        match pc {
            0x82F3D870 => {
    //   block [0x82F3D870..0x82F3DB48)
	// 82F3D870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3D874: 4826A8C5  bl 0x831a8138
	ctx.lr = 0x82F3D878;
	sub_831A8130(ctx, base);
	// 82F3D878: DBE1FF70  stfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82F3D87C: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3D880: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D884: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F3D888: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F3D88C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F3D890: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F3D894: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F3D898: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3D89C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3D8A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3D8A4: 40980020  bge cr6, 0x82f3d8c4
	if !ctx.cr[6].lt {
	pc = 0x82F3D8C4; continue 'dispatch;
	}
	// 82F3D8A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3D8AC: 3909FA64  addi r8, r9, -0x59c
	ctx.r[8].s64 = ctx.r[9].s64 + -1436;
	// 82F3D8B0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3D8B4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3D8B8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3D8BC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3D8C0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3D8C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3D8C8: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F3D8CC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3D8D0: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D8D4: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3D8D8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3D8DC: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 82F3D8E0: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82F3D8E4: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3D8E8: 38FC0020  addi r7, r28, 0x20
	ctx.r[7].s64 = ctx.r[28].s64 + 32;
	// 82F3D8EC: EACB0008  ld r22, 8(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3D8F0: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82F3D8F4: EAAB0010  ld r21, 0x10(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3D8F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82F3D8FC: EA8B0018  ld r20, 0x18(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3D900: 39470020  addi r10, r7, 0x20
	ctx.r[10].s64 = ctx.r[7].s64 + 32;
	// 82F3D904: EA6B0020  ld r19, 0x20(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3D908: 7CC73050  subf r6, r7, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 82F3D90C: EA4B0028  ld r18, 0x28(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3D910: EA2B0030  ld r17, 0x30(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3D914: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3D918: FAE50000  std r23, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F3D91C: FAC50008  std r22, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3D920: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3D924: FAA40000  std r21, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82F3D928: FA840008  std r20, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[20].u64 ) };
	// 82F3D92C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3D930: FA7E0000  std r19, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F3D934: FA5E0008  std r18, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[18].u64 ) };
	// 82F3D938: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 82F3D93C: F97D0008  std r11, 8(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3D940: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82F3D944: FA3D0000  std r17, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3DB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3DB48 size=204
    let mut pc: u32 = 0x82F3DB48;
    'dispatch: loop {
        match pc {
            0x82F3DB48 => {
    //   block [0x82F3DB48..0x82F3DC14)
	// 82F3DB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3DB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3DB50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3DB54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3DB58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3DB5C: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3DB60: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3DB64: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3DB68: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3DB6C: 38EBD0A0  addi r7, r11, -0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + -12128;
	// 82F3DB70: 38CADC60  addi r6, r10, -0x23a0
	ctx.r[6].s64 = ctx.r[10].s64 + -9120;
	// 82F3DB74: 38A9E068  addi r5, r9, -0x1f98
	ctx.r[5].s64 = ctx.r[9].s64 + -8088;
	// 82F3DB78: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3DB7C: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F3DB80: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3DB84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F3DB88: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3DB8C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F3DB90: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3DB94: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F3DB98: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82F3DB9C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F3DBA0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F3DBA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3DBA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3DBAC: 4BFE7895  bl 0x82f25440
	ctx.lr = 0x82F3DBB0;
	sub_82F25440(ctx, base);
	// 82F3DBB0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3DBB4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3DBB8: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F3DBBC: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3DBC0: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F3DBC4: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F3DBC8: 38CACFA0  addi r6, r10, -0x3060
	ctx.r[6].s64 = ctx.r[10].s64 + -12384;
	// 82F3DBCC: 38A9D368  addi r5, r9, -0x2c98
	ctx.r[5].s64 = ctx.r[9].s64 + -11416;
	// 82F3DBD0: 3888D870  addi r4, r8, -0x2790
	ctx.r[4].s64 = ctx.r[8].s64 + -10128;
	// 82F3DBD4: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F3DBD8: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F3DBDC: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F3DBE0: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F3DBE4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82F3DBE8: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F3DBEC: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82F3DBF0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3DBF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3DBF8: 4BFE7849  bl 0x82f25440
	ctx.lr = 0x82F3DBFC;
	sub_82F25440(ctx, base);
	// 82F3DBFC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F3DC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3DC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3DC08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3DC0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3DC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3DC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3DC18 size=72
    let mut pc: u32 = 0x82F3DC18;
    'dispatch: loop {
        match pc {
            0x82F3DC18 => {
    //   block [0x82F3DC18..0x82F3DC60)
	// 82F3DC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3DC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3DC20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3DC24: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3DC28: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3DC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3DC30: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3DC34: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3DC38: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3DC3C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3DC40: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3DC44: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3DC48: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3DC4C: 4BFFF4ED  bl 0x82f3d138
	ctx.lr = 0x82F3DC50;
	sub_82F3D138(ctx, base);
	// 82F3DC50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3DC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3DC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3DC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3DC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3DC60 size=72
    let mut pc: u32 = 0x82F3DC60;
    'dispatch: loop {
        match pc {
            0x82F3DC60 => {
    //   block [0x82F3DC60..0x82F3DCA8)
	// 82F3DC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3DC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3DC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3DC6C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3DC70: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3DC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3DC78: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3DC7C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3DC80: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3DC84: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3DC88: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3DC8C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3DC90: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3DC94: 4BFFF6D5  bl 0x82f3d368
	ctx.lr = 0x82F3DC98;
	sub_82F3D368(ctx, base);
	// 82F3DC98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3DC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3DCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3DCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3DCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3DCA8 size=876
    let mut pc: u32 = 0x82F3DCA8;
    'dispatch: loop {
        match pc {
            0x82F3DCA8 => {
    //   block [0x82F3DCA8..0x82F3E014)
	// 82F3DCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3DCAC: 4826A48D  bl 0x831a8138
	ctx.lr = 0x82F3DCB0;
	sub_831A8130(ctx, base);
	// 82F3DCB0: DBE1FF70  stfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82F3DCB4: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3DCB8: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3DCBC: 3A000018  li r16, 0x18
	ctx.r[16].s64 = 24;
	// 82F3DCC0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F3DCC4: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82F3DCC8: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82F3DCCC: 7CD23378  mr r18, r6
	ctx.r[18].u64 = ctx.r[6].u64;
	// 82F3DCD0: 7D70882E  lwzx r11, r16, r17
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[16].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82F3DCD4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F3DCD8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3DCDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3DCE0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3DCE4: 40980020  bge cr6, 0x82f3dd04
	if !ctx.cr[6].lt {
	pc = 0x82F3DD04; continue 'dispatch;
	}
	// 82F3DCE8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3DCEC: 3909FA50  addi r8, r9, -0x5b0
	ctx.r[8].s64 = ctx.r[9].s64 + -1456;
	// 82F3DCF0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3DCF4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3DCF8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3DCFC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3DD00: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3DD04: 82D30000  lwz r22, 0(r19)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3DD08: 3BC100C0  addi r30, r1, 0xc0
	ctx.r[30].s64 = ctx.r[1].s64 + 192;
	// 82F3DD0C: 81730008  lwz r11, 8(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3DD10: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F3DD14: 39160020  addi r8, r22, 0x20
	ctx.r[8].s64 = ctx.r[22].s64 + 32;
	// 82F3DD18: 80F80000  lwz r7, 0(r24)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3DD1C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3DD20: 39480020  addi r10, r8, 0x20
	ctx.r[10].s64 = ctx.r[8].s64 + 32;
	// 82F3DD24: 7D08F050  subf r8, r8, r30
	ctx.r[8].s64 = ctx.r[30].s64 - ctx.r[8].s64;
	// 82F3DD28: EBEB0000  ld r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3DD2C: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F3DD30: EBAB0008  ld r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3DD34: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F3DD38: EB6B0010  ld r27, 0x10(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3DD3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F3DD40: EB4B0018  ld r26, 0x18(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3DD44: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82F3DD48: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3DD4C: EBCB0028  ld r30, 0x28(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3DD50: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3DD54: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3DD58: FBE60000  std r31, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 82F3DD5C: FBA60008  std r29, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 82F3DD60: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82F3DD64: FB650000  std r27, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82F3DD68: FB450008  std r26, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[26].u64 ) };
	// 82F3DD6C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F3DD70: FAE40000  std r23, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F3DD74: FBC40008  std r30, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[30].u64 ) };
	// 82F3DD78: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3DD7C: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3DD80: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82F3DD84: FAA30000  std r21, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3E018 size=76
    let mut pc: u32 = 0x82F3E018;
    'dispatch: loop {
        match pc {
            0x82F3E018 => {
    //   block [0x82F3E018..0x82F3E064)
	// 82F3E018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3E020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E024: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3E028: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3E02C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3E030: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3E034: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3E038: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3E03C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3E040: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3E044: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3E048: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3E04C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3E050: 4BFFF551  bl 0x82f3d5a0
	ctx.lr = 0x82F3E054;
	sub_82F3D5A0(ctx, base);
	// 82F3E054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3E058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3E05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3E060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3E068 size=76
    let mut pc: u32 = 0x82F3E068;
    'dispatch: loop {
        match pc {
            0x82F3E068 => {
    //   block [0x82F3E068..0x82F3E0B4)
	// 82F3E068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3E070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E074: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3E078: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3E07C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3E080: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3E084: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3E088: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3E08C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3E090: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3E094: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3E098: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3E09C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3E0A0: 4BFFF7D1  bl 0x82f3d870
	ctx.lr = 0x82F3E0A4;
	sub_82F3D870(ctx, base);
	// 82F3E0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3E0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3E0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3E0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3E0B8 size=176
    let mut pc: u32 = 0x82F3E0B8;
    'dispatch: loop {
        match pc {
            0x82F3E0B8 => {
    //   block [0x82F3E0B8..0x82F3E168)
	// 82F3E0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3E0C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3E0C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3E0C8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F3E0CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E0D0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3E0D4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3E0D8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3E0DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3E0E0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E0E4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3E0E8: 4BFFFBC1  bl 0x82f3dca8
	ctx.lr = 0x82F3E0EC;
	sub_82F3DCA8(ctx, base);
	// 82F3E0EC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E0F0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F3E0F4: 40980044  bge cr6, 0x82f3e138
	if !ctx.cr[6].lt {
	pc = 0x82F3E138; continue 'dispatch;
	}
	// 82F3E0F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3E0FC: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F3E100: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E168 size=128
    let mut pc: u32 = 0x82F3E168;
    'dispatch: loop {
        match pc {
            0x82F3E168 => {
    //   block [0x82F3E168..0x82F3E1E8)
	// 82F3E168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E16C: 4826A001  bl 0x831a816c
	ctx.lr = 0x82F3E170;
	sub_831A8130(ctx, base);
	// 82F3E170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E174: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E178: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3E17C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3E180: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3E184: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82F3E188: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3E18C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3E190: 4BF625A1  bl 0x82ea0730
	ctx.lr = 0x82F3E194;
	sub_82EA0730(ctx, base);
	// 82F3E194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3E198: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82F3E19C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3E1A0: 612BFFFF  ori r11, r9, 0xffff
	ctx.r[11].u64 = ctx.r[9].u64 | 65535;
	// 82F3E1A4: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 82F3E1A8: 38C8FA80  addi r6, r8, -0x580
	ctx.r[6].s64 = ctx.r[8].s64 + -1408;
	// 82F3E1AC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F3E1B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F3E1B4: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82F3E1B8: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F3E1BC: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82F3E1C0: B0BF0006  sth r5, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82F3E1C4: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82F3E1C8: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82F3E1CC: B17F0010  sth r11, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82F3E1D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E1D4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F3E1D8: 48049DE1  bl 0x82f87fb8
	ctx.lr = 0x82F3E1DC;
	sub_82F87FB8(ctx, base);
	// 82F3E1DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3E1E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3E1E4: 48269FD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E1E8 size=116
    let mut pc: u32 = 0x82F3E1E8;
    'dispatch: loop {
        match pc {
            0x82F3E1E8 => {
    //   block [0x82F3E1E8..0x82F3E25C)
	// 82F3E1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E1EC: 48269F7D  bl 0x831a8168
	ctx.lr = 0x82F3E1F0;
	sub_831A8130(ctx, base);
	// 82F3E1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E1F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3E1F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3E1FC: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82F3E200: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82F3E204: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E208: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82F3E20C: 419A001C  beq cr6, 0x82f3e228
	if ctx.cr[6].eq {
	pc = 0x82F3E228; continue 'dispatch;
	}
	// 82F3E210: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3E214: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F3E218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E21C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3E220: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3E224: 4E800421  bctrl
	ctx.lr = 0x82F3E228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3E228: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F3E22C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82F3E230: 4082FFD4  bne 0x82f3e204
	if !ctx.cr[0].eq {
	pc = 0x82F3E204; continue 'dispatch;
	}
	// 82F3E234: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82F3E238: 419A001C  beq cr6, 0x82f3e254
	if ctx.cr[6].eq {
	pc = 0x82F3E254; continue 'dispatch;
	}
	// 82F3E23C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E240: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3E244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3E248: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E24C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3E250: 4E800421  bctrl
	ctx.lr = 0x82F3E254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3E254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3E258: 48269F60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E260 size=140
    let mut pc: u32 = 0x82F3E260;
    'dispatch: loop {
        match pc {
            0x82F3E260 => {
    //   block [0x82F3E260..0x82F3E2EC)
	// 82F3E260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E264: 48269F09  bl 0x831a816c
	ctx.lr = 0x82F3E268;
	sub_831A8130(ctx, base);
	// 82F3E268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E26C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E270: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3E274: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3E278: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82F3E27C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3E280: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F3E284: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3E288: 4BF624A9  bl 0x82ea0730
	ctx.lr = 0x82F3E28C;
	sub_82EA0730(ctx, base);
	// 82F3E28C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3E290: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82F3E294: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3E298: 612BFFFF  ori r11, r9, 0xffff
	ctx.r[11].u64 = ctx.r[9].u64 | 65535;
	// 82F3E29C: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 82F3E2A0: 38C8FA80  addi r6, r8, -0x580
	ctx.r[6].s64 = ctx.r[8].s64 + -1408;
	// 82F3E2A4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F3E2A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F3E2AC: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82F3E2B0: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F3E2B4: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82F3E2B8: B0BF0006  sth r5, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82F3E2BC: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82F3E2C0: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82F3E2C4: B17F0010  sth r11, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82F3E2C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E2CC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F3E2D0: 48049CE9  bl 0x82f87fb8
	ctx.lr = 0x82F3E2D4;
	sub_82F87FB8(ctx, base);
	// 82F3E2D4: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F3E2D8: 3864FABC  addi r3, r4, -0x544
	ctx.r[3].s64 = ctx.r[4].s64 + -1348;
	// 82F3E2DC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F3E2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F3E2E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3E2E8: 48269ED4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E2F0 size=432
    let mut pc: u32 = 0x82F3E2F0;
    'dispatch: loop {
        match pc {
            0x82F3E2F0 => {
    //   block [0x82F3E2F0..0x82F3E4A0)
	// 82F3E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E2F4: 48269E51  bl 0x831a8144
	ctx.lr = 0x82F3E2F8;
	sub_831A8130(ctx, base);
	// 82F3E2F8: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E2FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3E300: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82F3E304: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E308: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82F3E30C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F3E310: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E314: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F3E318: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F3E31C: EB2B0000  ld r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3E320: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82F3E324: EB0B0008  ld r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3E328: 3B410060  addi r26, r1, 0x60
	ctx.r[26].s64 = ctx.r[1].s64 + 96;
	// 82F3E32C: EAEB0010  ld r23, 0x10(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3E330: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F3E334: EACB0018  ld r22, 0x18(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3E338: 3BC100A0  addi r30, r1, 0xa0
	ctx.r[30].s64 = ctx.r[1].s64 + 160;
	// 82F3E33C: EAAB0020  ld r21, 0x20(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3E340: 3BFC0030  addi r31, r28, 0x30
	ctx.r[31].s64 = ctx.r[28].s64 + 48;
	// 82F3E344: EA8B0028  ld r20, 0x28(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3E348: EA6B0030  ld r19, 0x30(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3E34C: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3E350: FB280000  std r25, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82F3E354: FB080008  std r24, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F3E358: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F3E35C: FAE70000  std r23, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F3E360: FAC70008  std r22, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3E364: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3E368: FABB0000  std r21, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82F3E36C: FA9B0008  std r20, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[20].u64 ) };
	// 82F3E370: 3B610060  addi r27, r1, 0x60
	ctx.r[27].s64 = ctx.r[1].s64 + 96;
	// 82F3E374: F97A0008  std r11, 8(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3E378: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82F3E37C: FA7A0000  std r19, 0(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E4A0 size=564
    let mut pc: u32 = 0x82F3E4A0;
    'dispatch: loop {
        match pc {
            0x82F3E4A0 => {
    //   block [0x82F3E4A0..0x82F3E6D4)
	// 82F3E4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E4A4: 48269CA5  bl 0x831a8148
	ctx.lr = 0x82F3E4A8;
	sub_831A8130(ctx, base);
	// 82F3E4A8: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E4AC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3E4B0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F3E4B4: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E4B8: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82F3E4BC: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3E4C0: 3B810070  addi r28, r1, 0x70
	ctx.r[28].s64 = ctx.r[1].s64 + 112;
	// 82F3E4C4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F3E4C8: EB4B0000  ld r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3E4CC: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82F3E4D0: EB2B0008  ld r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3E4D4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3E4D8: EB0B0010  ld r24, 0x10(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3E4DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3E4E0: EAEB0018  ld r23, 0x18(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3E4E4: 39010100  addi r8, r1, 0x100
	ctx.r[8].s64 = ctx.r[1].s64 + 256;
	// 82F3E4E8: EACB0020  ld r22, 0x20(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3E4EC: 39250030  addi r9, r5, 0x30
	ctx.r[9].s64 = ctx.r[5].s64 + 48;
	// 82F3E4F0: EAAB0028  ld r21, 0x28(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3E4F4: EA8B0030  ld r20, 0x30(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3E4F8: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3E4FC: FB5D0000  std r26, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82F3E500: FB3D0008  std r25, 8(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[25].u64 ) };
	// 82F3E504: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82F3E508: FB1C0000  std r24, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82F3E50C: FAFC0008  std r23, 8(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[23].u64 ) };
	// 82F3E510: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 82F3E514: FAC30000  std r22, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82F3E518: FAA30008  std r21, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82F3E51C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F3E520: F97B0008  std r11, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3E524: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F3E528: FA9B0000  std r20, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E6D8 size=660
    let mut pc: u32 = 0x82F3E6D8;
    'dispatch: loop {
        match pc {
            0x82F3E6D8 => {
    //   block [0x82F3E6D8..0x82F3E96C)
	// 82F3E6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E6DC: 48269A65  bl 0x831a8140
	ctx.lr = 0x82F3E6E0;
	sub_831A8130(ctx, base);
	// 82F3E6E0: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E6E4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E6E8: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82F3E6EC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F3E6F0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F3E6F4: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F3E6F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3E6FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3E700: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3E704: 40980020  bge cr6, 0x82f3e724
	if !ctx.cr[6].lt {
	pc = 0x82F3E724; continue 'dispatch;
	}
	// 82F3E708: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3E70C: 3909FB04  addi r8, r9, -0x4fc
	ctx.r[8].s64 = ctx.r[9].s64 + -1276;
	// 82F3E710: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3E714: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3E718: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F3E71C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3E720: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F3E724: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3E728: 3B610060  addi r27, r1, 0x60
	ctx.r[27].s64 = ctx.r[1].s64 + 96;
	// 82F3E72C: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82F3E730: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82F3E734: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82F3E738: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3E73C: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E740: 3B210070  addi r25, r1, 0x70
	ctx.r[25].s64 = ctx.r[1].s64 + 112;
	// 82F3E744: 3B010050  addi r24, r1, 0x50
	ctx.r[24].s64 = ctx.r[1].s64 + 80;
	// 82F3E748: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E74C: E88B0010  ld r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3E750: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3E754: EAEB0018  ld r23, 0x18(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3E758: 39010130  addi r8, r1, 0x130
	ctx.r[8].s64 = ctx.r[1].s64 + 304;
	// 82F3E75C: EA4B0000  ld r18, 0(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3E760: 39260030  addi r9, r6, 0x30
	ctx.r[9].s64 = ctx.r[6].s64 + 48;
	// 82F3E764: EACB0020  ld r22, 0x20(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3E768: EAAB0028  ld r21, 0x28(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3E76C: EA8B0030  ld r20, 0x30(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3E770: EA6B0038  ld r19, 0x38(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3E774: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3E778: F97B0008  std r11, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3E77C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F3E780: F89A0000  std r4, 0(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 82F3E784: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3E788: FA5B0000  std r18, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82F3E78C: 3B610070  addi r27, r1, 0x70
	ctx.r[27].s64 = ctx.r[1].s64 + 112;
	// 82F3E790: FAFA0008  std r23, 8(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[23].u64 ) };
	// 82F3E794: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 82F3E798: FAD90000  std r22, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3E970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3E970 size=684
    let mut pc: u32 = 0x82F3E970;
    'dispatch: loop {
        match pc {
            0x82F3E970 => {
    //   block [0x82F3E970..0x82F3EC1C)
	// 82F3E970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3E974: 482697D1  bl 0x831a8144
	ctx.lr = 0x82F3E978;
	sub_831A8130(ctx, base);
	// 82F3E978: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3E97C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E980: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F3E984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F3E988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3E98C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F3E990: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F3E994: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F3E998: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3E99C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3E9A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3E9A4: 40980020  bge cr6, 0x82f3e9c4
	if !ctx.cr[6].lt {
	pc = 0x82F3E9C4; continue 'dispatch;
	}
	// 82F3E9A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3E9AC: 3909FB04  addi r8, r9, -0x4fc
	ctx.r[8].s64 = ctx.r[9].s64 + -1276;
	// 82F3E9B0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3E9B4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3E9B8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3E9BC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3E9C0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3E9C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E9C8: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82F3E9CC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F3E9D0: 480495E9  bl 0x82f87fb8
	ctx.lr = 0x82F3E9D4;
	sub_82F87FB8(ctx, base);
	// 82F3E9D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3E9D8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82F3E9DC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F3E9E0: 93C100B4  stw r30, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82F3E9E4: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3E9E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F3E9EC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E9F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F3E9F4: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3E9F8: 3BE10080  addi r31, r1, 0x80
	ctx.r[31].s64 = ctx.r[1].s64 + 128;
	// 82F3E9FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3EA00: 39010150  addi r8, r1, 0x150
	ctx.r[8].s64 = ctx.r[1].s64 + 336;
	// 82F3EA04: 39260030  addi r9, r6, 0x30
	ctx.r[9].s64 = ctx.r[6].s64 + 48;
	// 82F3EA08: EB2B0010  ld r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3EA0C: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3EA10: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3EA14: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3EA18: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3EA1C: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3EA20: EA8B0038  ld r20, 0x38(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3EA24: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3EA28: F9650008  std r11, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3EA2C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F3EA30: FA650000  std r19, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F3EA34: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F3EA38: FB240000  std r25, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3EC20 size=608
    let mut pc: u32 = 0x82F3EC20;
    'dispatch: loop {
        match pc {
            0x82F3EC20 => {
    //   block [0x82F3EC20..0x82F3EE80)
	// 82F3EC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3EC24: 48269519  bl 0x831a813c
	ctx.lr = 0x82F3EC28;
	sub_831A8130(ctx, base);
	// 82F3EC28: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3EC2C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EC30: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F3EC34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F3EC38: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F3EC3C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82F3EC40: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F3EC44: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3EC48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3EC4C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3EC50: 40980020  bge cr6, 0x82f3ec70
	if !ctx.cr[6].lt {
	pc = 0x82F3EC70; continue 'dispatch;
	}
	// 82F3EC54: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3EC58: 3909FB04  addi r8, r9, -0x4fc
	ctx.r[8].s64 = ctx.r[9].s64 + -1276;
	// 82F3EC5C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3EC60: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3EC64: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82F3EC68: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3EC6C: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F3EC70: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3EC74: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F3EC78: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82F3EC7C: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EC80: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3EC84: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EC88: 3B210070  addi r25, r1, 0x70
	ctx.r[25].s64 = ctx.r[1].s64 + 112;
	// 82F3EC8C: 3B010050  addi r24, r1, 0x50
	ctx.r[24].s64 = ctx.r[1].s64 + 80;
	// 82F3EC90: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3EC94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3EC98: EACB0008  ld r22, 8(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3EC9C: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82F3ECA0: EAAB0010  ld r21, 0x10(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3ECA4: 39250030  addi r9, r5, 0x30
	ctx.r[9].s64 = ctx.r[5].s64 + 48;
	// 82F3ECA8: EA8B0018  ld r20, 0x18(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3ECAC: EA6B0020  ld r19, 0x20(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3ECB0: EA4B0028  ld r18, 0x28(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3ECB4: EA2B0030  ld r17, 0x30(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3ECB8: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3ECBC: FAE40000  std r23, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F3ECC0: FAC40008  std r22, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3ECC4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3ECC8: FABA0000  std r21, 0(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82F3ECCC: FA9A0008  std r20, 8(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[20].u64 ) };
	// 82F3ECD0: 3B410070  addi r26, r1, 0x70
	ctx.r[26].s64 = ctx.r[1].s64 + 112;
	// 82F3ECD4: FA790000  std r19, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F3ECD8: FA590008  std r18, 8(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[18].u64 ) };
	// 82F3ECDC: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 82F3ECE0: F9780008  std r11, 8(r24)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3ECE4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F3ECE8: FA380000  std r17, 0(r24)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3EE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3EE80 size=628
    let mut pc: u32 = 0x82F3EE80;
    'dispatch: loop {
        match pc {
            0x82F3EE80 => {
    //   block [0x82F3EE80..0x82F3F0F4)
	// 82F3EE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3EE84: 482692BD  bl 0x831a8140
	ctx.lr = 0x82F3EE88;
	sub_831A8130(ctx, base);
	// 82F3EE88: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3EE8C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EE90: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F3EE94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3EE98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F3EE9C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F3EEA0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F3EEA4: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F3EEA8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3EEAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3EEB0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3EEB4: 40980020  bge cr6, 0x82f3eed4
	if !ctx.cr[6].lt {
	pc = 0x82F3EED4; continue 'dispatch;
	}
	// 82F3EEB8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3EEBC: 3909FB04  addi r8, r9, -0x4fc
	ctx.r[8].s64 = ctx.r[9].s64 + -1276;
	// 82F3EEC0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3EEC4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3EEC8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3EECC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3EED0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3EED4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EED8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F3EEDC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F3EEE0: 480490D9  bl 0x82f87fb8
	ctx.lr = 0x82F3EEE4;
	sub_82F87FB8(ctx, base);
	// 82F3EEE4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3EEE8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3EEEC: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EEF0: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3EEF4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3EEF8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3EEFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F3EF00: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 82F3EF04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3EF08: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F3EF0C: 39260030  addi r9, r6, 0x30
	ctx.r[9].s64 = ctx.r[6].s64 + 48;
	// 82F3EF10: EB0B0000  ld r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3EF14: EAEB0008  ld r23, 8(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3EF18: EACB0010  ld r22, 0x10(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3EF1C: EAAB0018  ld r21, 0x18(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3EF20: EA8B0020  ld r20, 0x20(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3EF24: EA6B0028  ld r19, 0x28(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3EF28: EA4B0030  ld r18, 0x30(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3EF2C: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3EF30: FB050000  std r24, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82F3EF34: FAE50008  std r23, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[23].u64 ) };
	// 82F3EF38: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F3EF3C: FAC40000  std r22, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82F3EF40: FAA40008  std r21, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82F3EF44: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F3EF48: FA830000  std r20, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82F3EF4C: FA630008  std r19, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 82F3EF50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F3EF54: F9790008  std r11, 8(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3EF58: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82F3EF5C: FA590000  std r18, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3F0F8 size=64
    let mut pc: u32 = 0x82F3F0F8;
    'dispatch: loop {
        match pc {
            0x82F3F0F8 => {
    //   block [0x82F3F0F8..0x82F3F138)
	// 82F3F0F8: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3F0FC: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3F100: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3F104: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F3F108: 38EBE168  addi r7, r11, -0x1e98
	ctx.r[7].s64 = ctx.r[11].s64 + -7832;
	// 82F3F10C: 38CAEE80  addi r6, r10, -0x1180
	ctx.r[6].s64 = ctx.r[10].s64 + -4480;
	// 82F3F110: 38A9E970  addi r5, r9, -0x1690
	ctx.r[5].s64 = ctx.r[9].s64 + -5776;
	// 82F3F114: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3F118: 3888BB88  addi r4, r8, -0x4478
	ctx.r[4].s64 = ctx.r[8].s64 + -17528;
	// 82F3F11C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F3F120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F3F124: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F3F128: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F3F12C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F3F130: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82F3F134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3F138 size=68
    let mut pc: u32 = 0x82F3F138;
    'dispatch: loop {
        match pc {
            0x82F3F138 => {
    //   block [0x82F3F138..0x82F3F17C)
	// 82F3F138: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3F13C: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3F140: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3F144: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3F148: 38EBE260  addi r7, r11, -0x1da0
	ctx.r[7].s64 = ctx.r[11].s64 + -7584;
	// 82F3F14C: 38CAF668  addi r6, r10, -0x998
	ctx.r[6].s64 = ctx.r[10].s64 + -2456;
	// 82F3F150: 38A9F700  addi r5, r9, -0x900
	ctx.r[5].s64 = ctx.r[9].s64 + -2304;
	// 82F3F154: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F3F158: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F3F15C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F3F160: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F3F164: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F3F168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F3F16C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F3F170: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F3F174: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 82F3F178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F180 size=204
    let mut pc: u32 = 0x82F3F180;
    'dispatch: loop {
        match pc {
            0x82F3F180 => {
    //   block [0x82F3F180..0x82F3F24C)
	// 82F3F180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F188: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3F18C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3F190: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F194: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3F198: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3F19C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3F1A0: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3F1A4: 38EBE260  addi r7, r11, -0x1da0
	ctx.r[7].s64 = ctx.r[11].s64 + -7584;
	// 82F3F1A8: 38CAF668  addi r6, r10, -0x998
	ctx.r[6].s64 = ctx.r[10].s64 + -2456;
	// 82F3F1AC: 38A9F700  addi r5, r9, -0x900
	ctx.r[5].s64 = ctx.r[9].s64 + -2304;
	// 82F3F1B0: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3F1B4: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F3F1B8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3F1BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F3F1C0: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3F1C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F3F1C8: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3F1CC: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F3F1D0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82F3F1D4: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F3F1D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F3F1DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3F1E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3F1E4: 4BFE625D  bl 0x82f25440
	ctx.lr = 0x82F3F1E8;
	sub_82F25440(ctx, base);
	// 82F3F1E8: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3F1EC: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3F1F0: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F3F1F4: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3F1F8: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F3F1FC: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F3F200: 38CAE168  addi r6, r10, -0x1e98
	ctx.r[6].s64 = ctx.r[10].s64 + -7832;
	// 82F3F204: 38A9EE80  addi r5, r9, -0x1180
	ctx.r[5].s64 = ctx.r[9].s64 + -4480;
	// 82F3F208: 3888E970  addi r4, r8, -0x1690
	ctx.r[4].s64 = ctx.r[8].s64 + -5776;
	// 82F3F20C: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F3F210: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F3F214: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F3F218: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F3F21C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82F3F220: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F3F224: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82F3F228: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3F22C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3F230: 4BFE6211  bl 0x82f25440
	ctx.lr = 0x82F3F234;
	sub_82F25440(ctx, base);
	// 82F3F234: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F3F238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3F244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3F248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F250 size=204
    let mut pc: u32 = 0x82F3F250;
    'dispatch: loop {
        match pc {
            0x82F3F250 => {
    //   block [0x82F3F250..0x82F3F31C)
	// 82F3F250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3F25C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3F260: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F264: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F3F268: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3F26C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3F270: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3F274: 38EBE260  addi r7, r11, -0x1da0
	ctx.r[7].s64 = ctx.r[11].s64 + -7584;
	// 82F3F278: 38CAF668  addi r6, r10, -0x998
	ctx.r[6].s64 = ctx.r[10].s64 + -2456;
	// 82F3F27C: 38A9F700  addi r5, r9, -0x900
	ctx.r[5].s64 = ctx.r[9].s64 + -2304;
	// 82F3F280: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F3F284: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F3F288: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F3F28C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F3F290: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F3F294: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F3F298: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F3F29C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F3F2A0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82F3F2A4: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F3F2A8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F3F2AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F3F2B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3F2B4: 4BFE62A5  bl 0x82f25558
	ctx.lr = 0x82F3F2B8;
	sub_82F25558(ctx, base);
	// 82F3F2B8: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F3F2BC: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F3F2C0: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F3F2C4: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F3F2C8: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F3F2CC: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F3F2D0: 38CAE168  addi r6, r10, -0x1e98
	ctx.r[6].s64 = ctx.r[10].s64 + -7832;
	// 82F3F2D4: 38A9EE80  addi r5, r9, -0x1180
	ctx.r[5].s64 = ctx.r[9].s64 + -4480;
	// 82F3F2D8: 3888E970  addi r4, r8, -0x1690
	ctx.r[4].s64 = ctx.r[8].s64 + -5776;
	// 82F3F2DC: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F3F2E0: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F3F2E4: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F3F2E8: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F3F2EC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82F3F2F0: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F3F2F4: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82F3F2F8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3F2FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3F300: 4BFE6259  bl 0x82f25558
	ctx.lr = 0x82F3F304;
	sub_82F25558(ctx, base);
	// 82F3F304: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F3F308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F3F314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3F318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F320 size=764
    let mut pc: u32 = 0x82F3F320;
    'dispatch: loop {
        match pc {
            0x82F3F320 => {
    //   block [0x82F3F320..0x82F3F61C)
	// 82F3F320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F324: 48268E19  bl 0x831a813c
	ctx.lr = 0x82F3F328;
	sub_831A8130(ctx, base);
	// 82F3F328: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F32C: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F330: 3A800018  li r20, 0x18
	ctx.r[20].s64 = 24;
	// 82F3F334: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F3F338: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82F3F33C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82F3F340: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82F3F344: 7D74A82E  lwzx r11, r20, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82F3F348: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F3F34C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3F350: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3F354: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3F358: 40980020  bge cr6, 0x82f3f378
	if !ctx.cr[6].lt {
	pc = 0x82F3F378; continue 'dispatch;
	}
	// 82F3F35C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3F360: 3909FAF4  addi r8, r9, -0x50c
	ctx.r[8].s64 = ctx.r[9].s64 + -1292;
	// 82F3F364: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3F368: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3F36C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3F370: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3F374: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3F378: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3F37C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F3F380: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3F384: 80D70000  lwz r6, 0(r23)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F388: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3F38C: 80F80000  lwz r7, 0(r24)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F390: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F3F394: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82F3F398: EBCB0000  ld r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3F39C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3F3A0: EB6B0008  ld r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3F3A4: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82F3F3A8: EB4B0010  ld r26, 0x10(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3F3AC: 39260030  addi r9, r6, 0x30
	ctx.r[9].s64 = ctx.r[6].s64 + 48;
	// 82F3F3B0: EB2B0018  ld r25, 0x18(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3F3B4: EA6B0020  ld r19, 0x20(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3F3B8: EA4B0028  ld r18, 0x28(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3F3BC: EA2B0030  ld r17, 0x30(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3F3C0: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3F3C4: FBC50000  std r30, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82F3F3C8: FB650008  std r27, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[27].u64 ) };
	// 82F3F3CC: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3F3D0: FB440000  std r26, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82F3F3D4: FB240008  std r25, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[25].u64 ) };
	// 82F3F3D8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F3F3DC: FA630000  std r19, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F3F3E0: FA430008  std r18, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[18].u64 ) };
	// 82F3F3E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F3F3E8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3F3EC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F3F3F0: FA3F0000  std r17, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F620 size=72
    let mut pc: u32 = 0x82F3F620;
    'dispatch: loop {
        match pc {
            0x82F3F620 => {
    //   block [0x82F3F620..0x82F3F668)
	// 82F3F620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F62C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3F630: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3F634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3F638: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3F63C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3F640: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3F644: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3F648: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3F64C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3F650: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3F654: 4BFFF5CD  bl 0x82f3ec20
	ctx.lr = 0x82F3F658;
	sub_82F3EC20(ctx, base);
	// 82F3F658: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3F65C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F668 size=72
    let mut pc: u32 = 0x82F3F668;
    'dispatch: loop {
        match pc {
            0x82F3F668 => {
    //   block [0x82F3F668..0x82F3F6B0)
	// 82F3F668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F674: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F3F678: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3F67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F3F680: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F3F684: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F3F688: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3F68C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3F690: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3F694: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3F698: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3F69C: 4BFFF7E5  bl 0x82f3ee80
	ctx.lr = 0x82F3F6A0;
	sub_82F3EE80(ctx, base);
	// 82F3F6A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3F6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3F6B0 size=76
    let mut pc: u32 = 0x82F3F6B0;
    'dispatch: loop {
        match pc {
            0x82F3F6B0 => {
    //   block [0x82F3F6B0..0x82F3F6FC)
	// 82F3F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F6B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F6BC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3F6C0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F3F6C4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3F6C8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3F6CC: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3F6D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3F6D4: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3F6D8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3F6DC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3F6E0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3F6E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3F6E8: 4BFFEFF1  bl 0x82f3e6d8
	ctx.lr = 0x82F3F6EC;
	sub_82F3E6D8(ctx, base);
	// 82F3F6EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3F6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3F700 size=76
    let mut pc: u32 = 0x82F3F700;
    'dispatch: loop {
        match pc {
            0x82F3F700 => {
    //   block [0x82F3F700..0x82F3F74C)
	// 82F3F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F70C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F3F710: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F3F714: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3F718: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F3F71C: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F3F720: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F3F724: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3F728: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F3F72C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F3F730: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3F734: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F3F738: 4BFFF239  bl 0x82f3e970
	ctx.lr = 0x82F3F73C;
	sub_82F3E970(ctx, base);
	// 82F3F73C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F3F740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3F750 size=176
    let mut pc: u32 = 0x82F3F750;
    'dispatch: loop {
        match pc {
            0x82F3F750 => {
    //   block [0x82F3F750..0x82F3F800)
	// 82F3F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F3F75C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3F760: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F3F764: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F768: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F3F76C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F3F770: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F3F774: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F3F778: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F77C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F3F780: 4BFFFBA1  bl 0x82f3f320
	ctx.lr = 0x82F3F784;
	sub_82F3F320(ctx, base);
	// 82F3F784: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F788: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F3F78C: 40980044  bge cr6, 0x82f3f7d0
	if !ctx.cr[6].lt {
	pc = 0x82F3F7D0; continue 'dispatch;
	}
	// 82F3F790: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F3F794: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F3F798: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F800 size=116
    let mut pc: u32 = 0x82F3F800;
    'dispatch: loop {
        match pc {
            0x82F3F800 => {
    //   block [0x82F3F800..0x82F3F874)
	// 82F3F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F3F808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F3F80C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F810: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F814: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F3F818: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F3F81C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82F3F820: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F3F824: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F3F828: 4BF60F09  bl 0x82ea0730
	ctx.lr = 0x82F3F82C;
	sub_82EA0730(ctx, base);
	// 82F3F82C: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82F3F830: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F3F834: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F3F838: 612BFFFF  ori r11, r9, 0xffff
	ctx.r[11].u64 = ctx.r[9].u64 | 65535;
	// 82F3F83C: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82F3F840: 38C8FB18  addi r6, r8, -0x4e8
	ctx.r[6].s64 = ctx.r[8].s64 + -1256;
	// 82F3F844: B163000C  sth r11, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82F3F848: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F3F84C: B0E30004  sth r7, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82F3F850: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F3F854: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82F3F858: B163000E  sth r11, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82F3F85C: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 82F3F860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F3F864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F3F868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F3F86C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F3F870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F878 size=116
    let mut pc: u32 = 0x82F3F878;
    'dispatch: loop {
        match pc {
            0x82F3F878 => {
    //   block [0x82F3F878..0x82F3F8EC)
	// 82F3F878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F87C: 482688ED  bl 0x831a8168
	ctx.lr = 0x82F3F880;
	sub_831A8130(ctx, base);
	// 82F3F880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F884: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F3F888: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F3F88C: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82F3F890: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82F3F894: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F898: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 82F3F89C: 419A001C  beq cr6, 0x82f3f8b8
	if ctx.cr[6].eq {
	pc = 0x82F3F8B8; continue 'dispatch;
	}
	// 82F3F8A0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3F8A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F3F8A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F8AC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F3F8B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3F8B4: 4E800421  bctrl
	ctx.lr = 0x82F3F8B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3F8B8: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F3F8BC: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82F3F8C0: 4082FFD4  bne 0x82f3f894
	if !ctx.cr[0].eq {
	pc = 0x82F3F894; continue 'dispatch;
	}
	// 82F3F8C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82F3F8C8: 419A001C  beq cr6, 0x82f3f8e4
	if ctx.cr[6].eq {
	pc = 0x82F3F8E4; continue 'dispatch;
	}
	// 82F3F8CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F8D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F3F8D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F3F8D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F8DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F3F8E0: 4E800421  bctrl
	ctx.lr = 0x82F3F8E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F3F8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F3F8E8: 482688D0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3F8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3F8F0 size=648
    let mut pc: u32 = 0x82F3F8F0;
    'dispatch: loop {
        match pc {
            0x82F3F8F0 => {
    //   block [0x82F3F8F0..0x82F3FB78)
	// 82F3F8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3F8F4: 48268851  bl 0x831a8144
	ctx.lr = 0x82F3F8F8;
	sub_831A8130(ctx, base);
	// 82F3F8F8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3F8FC: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F900: 3B400018  li r26, 0x18
	ctx.r[26].s64 = 24;
	// 82F3F904: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F3F908: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F3F90C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F3F910: 7D7AD82E  lwzx r11, r26, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F3F914: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3F918: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3F91C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3F920: 40980020  bge cr6, 0x82f3f940
	if !ctx.cr[6].lt {
	pc = 0x82F3F940; continue 'dispatch;
	}
	// 82F3F924: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3F928: 3909FB50  addi r8, r9, -0x4b0
	ctx.r[8].s64 = ctx.r[9].s64 + -1200;
	// 82F3F92C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3F930: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3F934: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F3F938: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3F93C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F3F940: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3F944: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F3F948: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3F94C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F950: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F3F954: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3F958: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F3F95C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F3F960: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3F964: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3F968: EB0B0008  ld r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3F96C: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82F3F970: EAEB0010  ld r23, 0x10(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3F974: 393C0030  addi r9, r28, 0x30
	ctx.r[9].s64 = ctx.r[28].s64 + 48;
	// 82F3F978: EACB0018  ld r22, 0x18(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3F97C: EAAB0020  ld r21, 0x20(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3F980: EA8B0028  ld r20, 0x28(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3F984: EA6B0030  ld r19, 0x30(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3F988: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3F98C: F8670000  std r3, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F3F990: FB070008  std r24, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F3F994: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F3F998: FAE60000  std r23, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F3F99C: FAC60008  std r22, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3F9A0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F3F9A4: FAA50000  std r21, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82F3F9A8: FA850008  std r20, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[20].u64 ) };
	// 82F3F9AC: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F3F9B0: F9640008  std r11, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3F9B4: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F3F9B8: FA640000  std r19, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3FB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F3FB78 size=20
    let mut pc: u32 = 0x82F3FB78;
    'dispatch: loop {
        match pc {
            0x82F3FB78 => {
    //   block [0x82F3FB78..0x82F3FB8C)
	// 82F3FB78: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F3FB7C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F3FB80: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F3FB84: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F3FB88: 4BFFFD68  b 0x82f3f8f0
	sub_82F3F8F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F3FB90 size=436
    let mut pc: u32 = 0x82F3FB90;
    'dispatch: loop {
        match pc {
            0x82F3FB90 => {
    //   block [0x82F3FB90..0x82F3FD44)
	// 82F3FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3FB94: 482685BD  bl 0x831a8150
	ctx.lr = 0x82F3FB98;
	sub_831A8130(ctx, base);
	// 82F3FB98: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3FB9C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82F3FBA0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3FBA4: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F3FBA8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F3FBAC: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3FBB0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FBB4: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 82F3FBB8: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FBBC: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 82F3FBC0: D007001C  stfs f0, 0x1c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82F3FBC4: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82F3FBC8: D007003C  stfs f0, 0x3c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82F3FBCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3FBD0: D007005C  stfs f0, 0x5c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82F3FBD4: EB8B0010  ld r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3FBD8: EB6B0018  ld r27, 0x18(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3FBDC: 390100B0  addi r8, r1, 0xb0
	ctx.r[8].s64 = ctx.r[1].s64 + 176;
	// 82F3FBE0: EB4B0020  ld r26, 0x20(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3FBE4: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 82F3FBE8: EB2B0028  ld r25, 0x28(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3FBEC: EACB0008  ld r22, 8(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3FBF0: EB0B0030  ld r24, 0x30(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3FBF4: EAEB0038  ld r23, 0x38(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3FBF8: E96B0000  ld r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3FBFC: FB9F0000  std r28, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 82F3FC00: FB7F0008  std r27, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u64 ) };
	// 82F3FC04: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 82F3FC08: FB5E0000  std r26, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82F3FC0C: FB3E0008  std r25, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[25].u64 ) };
	// 82F3FC10: 3BC10080  addi r30, r1, 0x80
	ctx.r[30].s64 = ctx.r[1].s64 + 128;
	// 82F3FC14: FAC50008  std r22, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3FC18: FB1D0000  std r24, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82F3FC1C: FAFD0008  std r23, 8(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[23].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3FD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3FD48 size=424
    let mut pc: u32 = 0x82F3FD48;
    'dispatch: loop {
        match pc {
            0x82F3FD48 => {
    //   block [0x82F3FD48..0x82F3FEF0)
	// 82F3FD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3FD4C: 48268401  bl 0x831a814c
	ctx.lr = 0x82F3FD50;
	sub_831A8130(ctx, base);
	// 82F3FD50: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3FD54: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3FD58: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82F3FD5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F3FD60: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FD64: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3FD68: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FD6C: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 82F3FD70: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
	// 82F3FD74: EB6B0000  ld r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3FD78: 3B810080  addi r28, r1, 0x80
	ctx.r[28].s64 = ctx.r[1].s64 + 128;
	// 82F3FD7C: EB4B0008  ld r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3FD80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3FD84: EB2B0010  ld r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3FD88: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82F3FD8C: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3FD90: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82F3FD94: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3FD98: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3FD9C: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3FDA0: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3FDA4: FB660000  std r27, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82F3FDA8: FB460008  std r26, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[26].u64 ) };
	// 82F3FDAC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F3FDB0: FB3E0000  std r25, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82F3FDB4: FB1E0008  std r24, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F3FDB8: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 82F3FDBC: FAFD0000  std r23, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F3FDC0: FADD0008  std r22, 8(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F3FDC4: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82F3FDC8: F97C0008  std r11, 8(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3FDCC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F3FDD0: FABC0000  std r21, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F3FEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F3FEF0 size=572
    let mut pc: u32 = 0x82F3FEF0;
    'dispatch: loop {
        match pc {
            0x82F3FEF0 => {
    //   block [0x82F3FEF0..0x82F4012C)
	// 82F3FEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F3FEF4: 48268251  bl 0x831a8144
	ctx.lr = 0x82F3FEF8;
	sub_831A8130(ctx, base);
	// 82F3FEF8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F3FEFC: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FF00: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F3FF04: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F3FF08: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F3FF0C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F3FF10: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F3FF14: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F3FF18: 40980020  bge cr6, 0x82f3ff38
	if !ctx.cr[6].lt {
	pc = 0x82F3FF38; continue 'dispatch;
	}
	// 82F3FF1C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F3FF20: 3909FB50  addi r8, r9, -0x4b0
	ctx.r[8].s64 = ctx.r[9].s64 + -1200;
	// 82F3FF24: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F3FF28: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F3FF2C: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F3FF30: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F3FF34: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F3FF38: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F3FF3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F3FF40: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82F3FF44: 3B810070  addi r28, r1, 0x70
	ctx.r[28].s64 = ctx.r[1].s64 + 112;
	// 82F3FF48: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82F3FF4C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F3FF50: 3B610080  addi r27, r1, 0x80
	ctx.r[27].s64 = ctx.r[1].s64 + 128;
	// 82F3FF54: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FF58: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 82F3FF5C: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F3FF60: EB2B0010  ld r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F3FF64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F3FF68: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F3FF6C: 390100F0  addi r8, r1, 0xf0
	ctx.r[8].s64 = ctx.r[1].s64 + 240;
	// 82F3FF70: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F3FF74: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82F3FF78: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F3FF7C: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F3FF80: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F3FF84: EA8B0038  ld r20, 0x38(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F3FF88: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F3FF8C: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F3FF90: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F3FF94: FA630000  std r19, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F3FF98: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F3FF9C: FB3C0000  std r25, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F40130 size=572
    let mut pc: u32 = 0x82F40130;
    'dispatch: loop {
        match pc {
            0x82F40130 => {
    //   block [0x82F40130..0x82F4036C)
	// 82F40130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F40134: 48268011  bl 0x831a8144
	ctx.lr = 0x82F40138;
	sub_831A8130(ctx, base);
	// 82F40138: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4013C: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40140: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F40144: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F40148: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F4014C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F40150: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F40154: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F40158: 40980020  bge cr6, 0x82f40178
	if !ctx.cr[6].lt {
	pc = 0x82F40178; continue 'dispatch;
	}
	// 82F4015C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F40160: 3909FB50  addi r8, r9, -0x4b0
	ctx.r[8].s64 = ctx.r[9].s64 + -1200;
	// 82F40164: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F40168: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4016C: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F40170: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F40174: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F40178: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4017C: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 82F40180: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82F40184: 3B610070  addi r27, r1, 0x70
	ctx.r[27].s64 = ctx.r[1].s64 + 112;
	// 82F40188: 908100B4  stw r4, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[4].u32 ) };
	// 82F4018C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F40190: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40194: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82F40198: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 82F4019C: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F401A0: E86B0010  ld r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F401A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F401A8: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F401AC: 390100F0  addi r8, r1, 0xf0
	ctx.r[8].s64 = ctx.r[1].s64 + 240;
	// 82F401B0: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F401B4: 39260030  addi r9, r6, 0x30
	ctx.r[9].s64 = ctx.r[6].s64 + 48;
	// 82F401B8: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F401BC: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F401C0: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F401C4: EA8B0038  ld r20, 0x38(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F401C8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F401CC: F97C0008  std r11, 8(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F401D0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F401D4: F87B0000  std r3, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F401D8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F401DC: FA7C0000  std r19, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F401E0: 3B810080  addi r28, r1, 0x80
	ctx.r[28].s64 = ctx.r[1].s64 + 128;
	// 82F401E4: FB1B0008  std r24, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F401E8: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82F401EC: FAFA0000  std r23, 0(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F40370 size=104
    let mut pc: u32 = 0x82F40370;
    'dispatch: loop {
        match pc {
            0x82F40370 => {
    //   block [0x82F40370..0x82F403D8)
	// 82F40370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F40374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F40378: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4037C: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F40380: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F40384: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F40388: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F4038C: 38CAF800  addi r6, r10, -0x800
	ctx.r[6].s64 = ctx.r[10].s64 + -2048;
	// 82F40390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F40394: 38A9F8F0  addi r5, r9, -0x710
	ctx.r[5].s64 = ctx.r[9].s64 + -1808;
	// 82F40398: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F4039C: 38880130  addi r4, r8, 0x130
	ctx.r[4].s64 = ctx.r[8].s64 + 304;
	// 82F403A0: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F403A4: 3947BB88  addi r10, r7, -0x4478
	ctx.r[10].s64 = ctx.r[7].s64 + -17528;
	// 82F403A8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F403AC: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82F403B0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82F403B4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82F403B8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82F403BC: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82F403C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F403C4: 4BFE507D  bl 0x82f25440
	ctx.lr = 0x82F403C8;
	sub_82F25440(ctx, base);
	// 82F403C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F403CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F403D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F403D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F403D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F403D8 size=768
    let mut pc: u32 = 0x82F403D8;
    'dispatch: loop {
        match pc {
            0x82F403D8 => {
    //   block [0x82F403D8..0x82F406D8)
	// 82F403D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F403DC: 48267D61  bl 0x831a813c
	ctx.lr = 0x82F403E0;
	sub_831A8130(ctx, base);
	// 82F403E0: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F403E4: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F403E8: 3A800018  li r20, 0x18
	ctx.r[20].s64 = 24;
	// 82F403EC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F403F0: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82F403F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F403F8: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82F403FC: 7D74A82E  lwzx r11, r20, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82F40400: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F40404: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F40408: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4040C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F40410: 40980020  bge cr6, 0x82f40430
	if !ctx.cr[6].lt {
	pc = 0x82F40430; continue 'dispatch;
	}
	// 82F40414: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F40418: 3909FB50  addi r8, r9, -0x4b0
	ctx.r[8].s64 = ctx.r[9].s64 + -1200;
	// 82F4041C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F40420: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F40424: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F40428: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4042C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F40430: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40434: C0160004  lfs f0, 4(r22)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F40438: D00100EC  stfs f0, 0xec(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 82F4043C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F40440: D001010C  stfs f0, 0x10c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 82F40444: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F40448: D001012C  stfs f0, 0x12c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 82F4044C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F40450: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F40454: 80970000  lwz r4, 0(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40458: EBCB0010  ld r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F4045C: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82F40460: EB4B0018  ld r26, 0x18(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F40464: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F40468: EB2B0020  ld r25, 0x20(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F4046C: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F40470: EB0B0028  ld r24, 0x28(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F40474: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82F40478: EA2B0000  ld r17, 0(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F4047C: EA6B0030  ld r19, 0x30(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F40480: EA4B0038  ld r18, 0x38(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F40484: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F40488: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F4048C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F40490: FA270000  std r17, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82F40494: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82F40498: FBC50000  std r30, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F406D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F406D8 size=104
    let mut pc: u32 = 0x82F406D8;
    'dispatch: loop {
        match pc {
            0x82F406D8 => {
    //   block [0x82F406D8..0x82F40740)
	// 82F406D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F406DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F406E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F406E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F406E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F406EC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F406F0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F406F4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F406F8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F406FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F40700: 4BF60031  bl 0x82ea0730
	ctx.lr = 0x82F40704;
	sub_82EA0730(ctx, base);
	// 82F40704: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F40708: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F4070C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F40710: 38E9FB60  addi r7, r9, -0x4a0
	ctx.r[7].s64 = ctx.r[9].s64 + -1184;
	// 82F40714: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F40718: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F4071C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F40720: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F40724: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F40728: B0A3000C  sth r5, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F4072C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F40730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F40734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F40738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4073C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F40740 size=104
    let mut pc: u32 = 0x82F40740;
    'dispatch: loop {
        match pc {
            0x82F40740 => {
    //   block [0x82F40740..0x82F407A8)
	// 82F40740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F40744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F40748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4074C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F40750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F40754: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F40758: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4075C: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82F40760: 419A001C  beq cr6, 0x82f4077c
	if ctx.cr[6].eq {
	pc = 0x82F4077C; continue 'dispatch;
	}
	// 82F40764: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40768: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F4076C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40770: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F40774: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F40778: 4E800421  bctrl
	ctx.lr = 0x82F4077C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4077C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40780: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F40784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F40788: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4078C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F40790: 4E800421  bctrl
	ctx.lr = 0x82F40794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F40794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F40798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4079C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F407A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F407A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F407A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F407A8 size=104
    let mut pc: u32 = 0x82F407A8;
    'dispatch: loop {
        match pc {
            0x82F407A8 => {
    //   block [0x82F407A8..0x82F40810)
	// 82F407A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F407AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F407B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F407B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F407B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F407BC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F407C0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F407C4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F407C8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F407CC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F407D0: 4BF5FF61  bl 0x82ea0730
	ctx.lr = 0x82F407D4;
	sub_82EA0730(ctx, base);
	// 82F407D4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F407D8: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F407DC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F407E0: 38E9FB9C  addi r7, r9, -0x464
	ctx.r[7].s64 = ctx.r[9].s64 + -1124;
	// 82F407E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F407E8: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F407EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F407F0: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F407F4: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F407F8: B0A3000C  sth r5, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F407FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F40800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F40804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F40808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4080C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F40810 size=360
    let mut pc: u32 = 0x82F40810;
    'dispatch: loop {
        match pc {
            0x82F40810 => {
    //   block [0x82F40810..0x82F40978)
	// 82F40810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F40814: 48267949  bl 0x831a815c
	ctx.lr = 0x82F40818;
	sub_831A8130(ctx, base);
	// 82F40818: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4081C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40820: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F40824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F40828: 7F8A5A14  add r28, r10, r11
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4082C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F40830: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F40834: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F40838: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4083C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F40840: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F40844: 40980020  bge cr6, 0x82f40864
	if !ctx.cr[6].lt {
	pc = 0x82F40864; continue 'dispatch;
	}
	// 82F40848: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4084C: 3909FBD4  addi r8, r9, -0x42c
	ctx.r[8].s64 = ctx.r[9].s64 + -1068;
	// 82F40850: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F40854: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F40858: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4085C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F40860: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F40864: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40868: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4086C: 835E0000  lwz r26, 0(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40870: 3BAB0030  addi r29, r11, 0x30
	ctx.r[29].s64 = ctx.r[11].s64 + 48;
	// 82F40874: 833F0000  lwz r25, 0(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40878: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4087C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F40880: 4BF66631  bl 0x82ea6eb0
	ctx.lr = 0x82F40884;
	sub_82EA6EB0(ctx, base);
	// 82F40884: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F40888: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82F4088C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40890: 392BBD40  addi r9, r11, -0x42c0
	ctx.r[9].s64 = ctx.r[11].s64 + -17088;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F40978 size=20
    let mut pc: u32 = 0x82F40978;
    'dispatch: loop {
        match pc {
            0x82F40978 => {
    //   block [0x82F40978..0x82F4098C)
	// 82F40978: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F4097C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F40980: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F40984: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F40988: 4BFFFE88  b 0x82f40810
	sub_82F40810(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F40990 size=900
    let mut pc: u32 = 0x82F40990;
    'dispatch: loop {
        match pc {
            0x82F40990 => {
    //   block [0x82F40990..0x82F40D14)
	// 82F40990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F40994: 482677C1  bl 0x831a8154
	ctx.lr = 0x82F40998;
	sub_831A8130(ctx, base);
	// 82F40998: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4099C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F409A0: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F409A4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F409A8: 7EEA5A14  add r23, r10, r11
	ctx.r[23].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F409AC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F409B0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F409B4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F409B8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82F409BC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F409C0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F409C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F409C8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F409CC: 40980020  bge cr6, 0x82f409ec
	if !ctx.cr[6].lt {
	pc = 0x82F409EC; continue 'dispatch;
	}
	// 82F409D0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F409D4: 3909FBD4  addi r8, r9, -0x42c
	ctx.r[8].s64 = ctx.r[9].s64 + -1068;
	// 82F409D8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F409DC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F409E0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F409E4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F409E8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F409EC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F409F0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F409F4: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F409F8: 3BDB0008  addi r30, r27, 8
	ctx.r[30].s64 = ctx.r[27].s64 + 8;
	// 82F409FC: 3BAB0030  addi r29, r11, 0x30
	ctx.r[29].s64 = ctx.r[11].s64 + 48;
	// 82F40A00: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40A04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F40A08: 4BF664A9  bl 0x82ea6eb0
	ctx.lr = 0x82F40A0C;
	sub_82EA6EB0(ctx, base);
	// 82F40A0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F40A10: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82F40A14: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40A18: 38EBBD40  addi r7, r11, -0x42c0
	ctx.r[7].s64 = ctx.r[11].s64 + -17088;
	// 82F40A1C: 1180038C  vspltisw v12, 0
	for i in 0..4 {
		ctx.v[12].u32[i] = 0;
	}
	// 82F40A20: 39690020  addi r11, r9, 0x20
	ctx.r[11].s64 = ctx.r[9].s64 + 32;
	// 82F40A24: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40A28: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F40A2C: 3CA08338  lis r5, -0x7cc8
	ctx.r[5].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F40D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F40D18 size=792
    let mut pc: u32 = 0x82F40D18;
    'dispatch: loop {
        match pc {
            0x82F40D18 => {
    //   block [0x82F40D18..0x82F41030)
	// 82F40D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F40D1C: 48267441  bl 0x831a815c
	ctx.lr = 0x82F40D20;
	sub_831A8130(ctx, base);
	// 82F40D20: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F40D24: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40D28: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F40D2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F40D30: 7F4A5A14  add r26, r10, r11
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F40D34: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F40D38: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F40D3C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F40D40: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F40D44: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F40D48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F40D4C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F40D50: 40980020  bge cr6, 0x82f40d70
	if !ctx.cr[6].lt {
	pc = 0x82F40D70; continue 'dispatch;
	}
	// 82F40D54: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F40D58: 3909FBD4  addi r8, r9, -0x42c
	ctx.r[8].s64 = ctx.r[9].s64 + -1068;
	// 82F40D5C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F40D60: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F40D64: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F40D68: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F40D6C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F40D70: 93E100A0  stw r31, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u32 ) };
	// 82F40D74: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F40D78: 93C100A4  stw r30, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82F40D7C: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	// 82F40D80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40D84: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F40D88: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 82F40D8C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F40D90: 4BF66121  bl 0x82ea6eb0
	ctx.lr = 0x82F40D94;
	sub_82EA6EB0(ctx, base);
	// 82F40D94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F40D98: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F40D9C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40DA0: 38EBBD40  addi r7, r11, -0x42c0
	ctx.r[7].s64 = ctx.r[11].s64 + -17088;
	// 82F40DA4: 1180038C  vspltisw v12, 0
	for i in 0..4 {
		ctx.v[12].u32[i] = 0;
	}
	// 82F40DA8: 396A0020  addi r11, r10, 0x20
	ctx.r[11].s64 = ctx.r[10].s64 + 32;
	// 82F40DAC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F40DB0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F40DB4: 3CA08338  lis r5, -0x7cc8
	ctx.r[5].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41030 size=792
    let mut pc: u32 = 0x82F41030;
    'dispatch: loop {
        match pc {
            0x82F41030 => {
    //   block [0x82F41030..0x82F41348)
	// 82F41030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F41034: 48267129  bl 0x831a815c
	ctx.lr = 0x82F41038;
	sub_831A8130(ctx, base);
	// 82F41038: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4103C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41040: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F41044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F41048: 7F4A5A14  add r26, r10, r11
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4104C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F41050: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F41054: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F41058: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F4105C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F41060: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F41064: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F41068: 40980020  bge cr6, 0x82f41088
	if !ctx.cr[6].lt {
	pc = 0x82F41088; continue 'dispatch;
	}
	// 82F4106C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F41070: 3909FBD4  addi r8, r9, -0x42c
	ctx.r[8].s64 = ctx.r[9].s64 + -1068;
	// 82F41074: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F41078: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4107C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F41080: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F41084: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F41088: 93E100A0  stw r31, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u32 ) };
	// 82F4108C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F41090: 93C100A4  stw r30, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82F41094: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	// 82F41098: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4109C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F410A0: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 82F410A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F410A8: 4BF65E09  bl 0x82ea6eb0
	ctx.lr = 0x82F410AC;
	sub_82EA6EB0(ctx, base);
	// 82F410AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F410B0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F410B4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F410B8: 38EBBD40  addi r7, r11, -0x42c0
	ctx.r[7].s64 = ctx.r[11].s64 + -17088;
	// 82F410BC: 1180038C  vspltisw v12, 0
	for i in 0..4 {
		ctx.v[12].u32[i] = 0;
	}
	// 82F410C0: 396A0020  addi r11, r10, 0x20
	ctx.r[11].s64 = ctx.r[10].s64 + 32;
	// 82F410C4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F410C8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F410CC: 3CA08338  lis r5, -0x7cc8
	ctx.r[5].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41348 size=204
    let mut pc: u32 = 0x82F41348;
    'dispatch: loop {
        match pc {
            0x82F41348 => {
    //   block [0x82F41348..0x82F41414)
	// 82F41348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4134C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F41354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F41358: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4135C: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F41360: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F41364: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F41368: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F4136C: 38EB07A8  addi r7, r11, 0x7a8
	ctx.r[7].s64 = ctx.r[11].s64 + 1960;
	// 82F41370: 38CA1460  addi r6, r10, 0x1460
	ctx.r[6].s64 = ctx.r[10].s64 + 5216;
	// 82F41374: 38A914F8  addi r5, r9, 0x14f8
	ctx.r[5].s64 = ctx.r[9].s64 + 5368;
	// 82F41378: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F4137C: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F41380: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F41384: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F41388: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F4138C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F41390: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F41394: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F41398: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F4139C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F413A0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82F413A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F413A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F413AC: 4BFE4095  bl 0x82f25440
	ctx.lr = 0x82F413B0;
	sub_82F25440(ctx, base);
	// 82F413B0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F413B4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F413B8: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F413BC: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F413C0: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F413C4: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F413C8: 38CA06D8  addi r6, r10, 0x6d8
	ctx.r[6].s64 = ctx.r[10].s64 + 1752;
	// 82F413CC: 38A90810  addi r5, r9, 0x810
	ctx.r[5].s64 = ctx.r[9].s64 + 2064;
	// 82F413D0: 38881030  addi r4, r8, 0x1030
	ctx.r[4].s64 = ctx.r[8].s64 + 4144;
	// 82F413D4: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F413D8: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F413DC: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F413E0: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F413E4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82F413E8: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F413EC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F413F0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F413F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F413F8: 4BFE4049  bl 0x82f25440
	ctx.lr = 0x82F413FC;
	sub_82F25440(ctx, base);
	// 82F413FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F41400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F41404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F41408: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4140C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F41410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41418 size=68
    let mut pc: u32 = 0x82F41418;
    'dispatch: loop {
        match pc {
            0x82F41418 => {
    //   block [0x82F41418..0x82F4145C)
	// 82F41418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4141C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41424: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F41428: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F4142C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F41430: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F41434: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F41438: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F4143C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F41440: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F41444: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F41448: 4BFFF3C9  bl 0x82f40810
	ctx.lr = 0x82F4144C;
	sub_82F40810(ctx, base);
	// 82F4144C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F41450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F41454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F41458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41460 size=72
    let mut pc: u32 = 0x82F41460;
    'dispatch: loop {
        match pc {
            0x82F41460 => {
    //   block [0x82F41460..0x82F414A8)
	// 82F41460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F41464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4146C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F41470: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F41474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F41478: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F4147C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F41480: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F41484: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F41488: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F4148C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F41490: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F41494: 4BFFF37D  bl 0x82f40810
	ctx.lr = 0x82F41498;
	sub_82F40810(ctx, base);
	// 82F41498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4149C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F414A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F414A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


