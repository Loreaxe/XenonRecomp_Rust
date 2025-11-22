pub fn sub_82C13678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13678 size=164
    let mut pc: u32 = 0x82C13678;
    'dispatch: loop {
        match pc {
            0x82C13678 => {
    //   block [0x82C13678..0x82C1371C)
	// 82C13678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1367C: 48095D91  bl 0x82ca940c
	ctx.lr = 0x82C13680;
	sub_82CA93D0(ctx, base);
	// 82C13680: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13720 size=152
    let mut pc: u32 = 0x82C13720;
    'dispatch: loop {
        match pc {
            0x82C13720 => {
    //   block [0x82C13720..0x82C137B8)
	// 82C13720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1372C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C13730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13734: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13738: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C1373C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13740: 419A0054  beq cr6, 0x82c13794
	if ctx.cr[6].eq {
	pc = 0x82C13794; continue 'dispatch;
	}
	// 82C13744: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1374C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13750: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13754: 4E800421  bctrl
	ctx.lr = 0x82C13758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13758: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1375C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C13760: 409A0050  bne cr6, 0x82c137b0
	if !ctx.cr[6].eq {
	pc = 0x82C137B0; continue 'dispatch;
	}
	// 82C13764: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C13768: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C1376C: 409A001C  bne cr6, 0x82c13788
	if !ctx.cr[6].eq {
	pc = 0x82C13788; continue 'dispatch;
	}
	// 82C13770: 83FF0054  lwz r31, 0x54(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C13774: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13778: 419A001C  beq cr6, 0x82c13794
	if ctx.cr[6].eq {
	pc = 0x82C13794; continue 'dispatch;
	}
	// 82C1377C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C13780: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13784: 419A0008  beq cr6, 0x82c1378c
	if ctx.cr[6].eq {
	pc = 0x82C1378C; continue 'dispatch;
	}
	// 82C13788: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C1378C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13790: 409AFFB4  bne cr6, 0x82c13744
	if !ctx.cr[6].eq {
	pc = 0x82C13744; continue 'dispatch;
	}
	// 82C13794: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C13798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1379C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C137A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C137A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C137A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C137AC: 4E800020  blr
	return;
	// 82C137B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C137B4: 4BFFFFE4  b 0x82c13798
	pc = 0x82C13798; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C137B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C137B8 size=128
    let mut pc: u32 = 0x82C137B8;
    'dispatch: loop {
        match pc {
            0x82C137B8 => {
    //   block [0x82C137B8..0x82C13838)
	// 82C137B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C137BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C137C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C137C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C137C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C137CC: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C137D0: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C137D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C137D8: 419A0048  beq cr6, 0x82c13820
	if ctx.cr[6].eq {
	pc = 0x82C13820; continue 'dispatch;
	}
	// 82C137DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C137E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C137E4: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C137E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C137EC: 4E800421  bctrl
	ctx.lr = 0x82C137F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C137F0: 813F005C  lwz r9, 0x5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C137F4: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C137F8: 409A001C  bne cr6, 0x82c13814
	if !ctx.cr[6].eq {
	pc = 0x82C13814; continue 'dispatch;
	}
	// 82C137FC: 83FF0054  lwz r31, 0x54(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C13800: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13804: 419A001C  beq cr6, 0x82c13820
	if ctx.cr[6].eq {
	pc = 0x82C13820; continue 'dispatch;
	}
	// 82C13808: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C1380C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13810: 419A0008  beq cr6, 0x82c13818
	if ctx.cr[6].eq {
	pc = 0x82C13818; continue 'dispatch;
	}
	// 82C13814: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13818: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1381C: 409AFFC0  bne cr6, 0x82c137dc
	if !ctx.cr[6].eq {
	pc = 0x82C137DC; continue 'dispatch;
	}
	// 82C13820: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1382C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C13830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C13834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13838 size=120
    let mut pc: u32 = 0x82C13838;
    'dispatch: loop {
        match pc {
            0x82C13838 => {
    //   block [0x82C13838..0x82C138B0)
	// 82C13838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1383C: 48095BCD  bl 0x82ca9408
	ctx.lr = 0x82C13840;
	sub_82CA93D0(ctx, base);
	// 82C13840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13844: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13848: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C1384C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C13850: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C13854: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13858: 419A0050  beq cr6, 0x82c138a8
	if ctx.cr[6].eq {
	pc = 0x82C138A8; continue 'dispatch;
	}
	// 82C1385C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13860: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C13864: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C13868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1386C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C13870: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13874: 4E800421  bctrl
	ctx.lr = 0x82C13878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13878: 813F005C  lwz r9, 0x5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C1387C: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13880: 409A001C  bne cr6, 0x82c1389c
	if !ctx.cr[6].eq {
	pc = 0x82C1389C; continue 'dispatch;
	}
	// 82C13884: 83FF0054  lwz r31, 0x54(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C13888: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1388C: 419A001C  beq cr6, 0x82c138a8
	if ctx.cr[6].eq {
	pc = 0x82C138A8; continue 'dispatch;
	}
	// 82C13890: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C13894: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13898: 419A0008  beq cr6, 0x82c138a0
	if ctx.cr[6].eq {
	pc = 0x82C138A0; continue 'dispatch;
	}
	// 82C1389C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C138A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C138A4: 409AFFB8  bne cr6, 0x82c1385c
	if !ctx.cr[6].eq {
	pc = 0x82C1385C; continue 'dispatch;
	}
	// 82C138A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C138AC: 48095BAC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C138B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C138B0 size=164
    let mut pc: u32 = 0x82C138B0;
    'dispatch: loop {
        match pc {
            0x82C138B0 => {
    //   block [0x82C138B0..0x82C13954)
	// 82C138B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C138B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C138B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C138BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C138C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C138C4: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C138C8: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C138CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C138D0: 419A006C  beq cr6, 0x82c1393c
	if ctx.cr[6].eq {
	pc = 0x82C1393C; continue 'dispatch;
	}
	// 82C138D4: 897F0044  lbz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C138D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C138DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C138E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C138E4: 419A001C  beq cr6, 0x82c13900
	if ctx.cr[6].eq {
	pc = 0x82C13900; continue 'dispatch;
	}
	// 82C138E8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C138EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C138F0: 388001F4  li r4, 0x1f4
	ctx.r[4].s64 = 500;
	// 82C138F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C138F8: 4E800421  bctrl
	ctx.lr = 0x82C138FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C138FC: 48000010  b 0x82c1390c
	pc = 0x82C1390C; continue 'dispatch;
	// 82C13900: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13904: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13908: 4E800421  bctrl
	ctx.lr = 0x82C1390C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1390C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C13910: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13914: 409A001C  bne cr6, 0x82c13930
	if !ctx.cr[6].eq {
	pc = 0x82C13930; continue 'dispatch;
	}
	// 82C13918: 83FF0054  lwz r31, 0x54(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C1391C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13920: 419A001C  beq cr6, 0x82c1393c
	if ctx.cr[6].eq {
	pc = 0x82C1393C; continue 'dispatch;
	}
	// 82C13924: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C13928: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C1392C: 419A0008  beq cr6, 0x82c13934
	if ctx.cr[6].eq {
	pc = 0x82C13934; continue 'dispatch;
	}
	// 82C13930: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13934: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13938: 409AFF9C  bne cr6, 0x82c138d4
	if !ctx.cr[6].eq {
	pc = 0x82C138D4; continue 'dispatch;
	}
	// 82C1393C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C13948: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1394C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C13950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13958 size=84
    let mut pc: u32 = 0x82C13958;
    'dispatch: loop {
        match pc {
            0x82C13958 => {
    //   block [0x82C13958..0x82C139AC)
	// 82C13958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1395C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13960: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C13964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13968: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C1396C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13970: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C13974: 80640038  lwz r3, 0x38(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C13978: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1397C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C13980: 419A0014  beq cr6, 0x82c13994
	if ctx.cr[6].eq {
	pc = 0x82C13994; continue 'dispatch;
	}
	// 82C13984: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13988: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1398C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13990: 4E800421  bctrl
	ctx.lr = 0x82C13994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1399C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C139A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C139A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C139A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C139B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C139B0 size=152
    let mut pc: u32 = 0x82C139B0;
    'dispatch: loop {
        match pc {
            0x82C139B0 => {
    //   block [0x82C139B0..0x82C13A48)
	// 82C139B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C139B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C139B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C139BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C139C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C139C4: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C139C8: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C139CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C139D0: 419A0054  beq cr6, 0x82c13a24
	if ctx.cr[6].eq {
	pc = 0x82C13A24; continue 'dispatch;
	}
	// 82C139D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C139D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C139DC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C139E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C139E4: 4E800421  bctrl
	ctx.lr = 0x82C139E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C139E8: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C139EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C139F0: 409A0050  bne cr6, 0x82c13a40
	if !ctx.cr[6].eq {
	pc = 0x82C13A40; continue 'dispatch;
	}
	// 82C139F4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C139F8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C139FC: 409A001C  bne cr6, 0x82c13a18
	if !ctx.cr[6].eq {
	pc = 0x82C13A18; continue 'dispatch;
	}
	// 82C13A00: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13A04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13A08: 419A001C  beq cr6, 0x82c13a24
	if ctx.cr[6].eq {
	pc = 0x82C13A24; continue 'dispatch;
	}
	// 82C13A0C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13A10: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13A14: 419A0008  beq cr6, 0x82c13a1c
	if ctx.cr[6].eq {
	pc = 0x82C13A1C; continue 'dispatch;
	}
	// 82C13A18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13A1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13A20: 409AFFB4  bne cr6, 0x82c139d4
	if !ctx.cr[6].eq {
	pc = 0x82C139D4; continue 'dispatch;
	}
	// 82C13A24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C13A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C13A34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C13A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C13A3C: 4E800020  blr
	return;
	// 82C13A40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C13A44: 4BFFFFE4  b 0x82c13a28
	pc = 0x82C13A28; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13A48 size=128
    let mut pc: u32 = 0x82C13A48;
    'dispatch: loop {
        match pc {
            0x82C13A48 => {
    //   block [0x82C13A48..0x82C13AC8)
	// 82C13A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C13A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C13A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13A5C: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13A60: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C13A64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13A68: 419A0048  beq cr6, 0x82c13ab0
	if ctx.cr[6].eq {
	pc = 0x82C13AB0; continue 'dispatch;
	}
	// 82C13A6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13A74: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C13A78: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13A7C: 4E800421  bctrl
	ctx.lr = 0x82C13A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13A80: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13A84: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13A88: 409A001C  bne cr6, 0x82c13aa4
	if !ctx.cr[6].eq {
	pc = 0x82C13AA4; continue 'dispatch;
	}
	// 82C13A8C: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13A90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13A94: 419A001C  beq cr6, 0x82c13ab0
	if ctx.cr[6].eq {
	pc = 0x82C13AB0; continue 'dispatch;
	}
	// 82C13A98: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13A9C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13AA0: 419A0008  beq cr6, 0x82c13aa8
	if ctx.cr[6].eq {
	pc = 0x82C13AA8; continue 'dispatch;
	}
	// 82C13AA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13AA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13AAC: 409AFFC0  bne cr6, 0x82c13a6c
	if !ctx.cr[6].eq {
	pc = 0x82C13A6C; continue 'dispatch;
	}
	// 82C13AB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C13ABC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C13AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C13AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13AC8 size=120
    let mut pc: u32 = 0x82C13AC8;
    'dispatch: loop {
        match pc {
            0x82C13AC8 => {
    //   block [0x82C13AC8..0x82C13B40)
	// 82C13AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13ACC: 4809593D  bl 0x82ca9408
	ctx.lr = 0x82C13AD0;
	sub_82CA93D0(ctx, base);
	// 82C13AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13AD4: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13AD8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C13ADC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C13AE0: 3BC3001C  addi r30, r3, 0x1c
	ctx.r[30].s64 = ctx.r[3].s64 + 28;
	// 82C13AE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13AE8: 419A0050  beq cr6, 0x82c13b38
	if ctx.cr[6].eq {
	pc = 0x82C13B38; continue 'dispatch;
	}
	// 82C13AEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13AF0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C13AF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C13AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13AFC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13B00: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13B04: 4E800421  bctrl
	ctx.lr = 0x82C13B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13B08: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13B0C: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13B10: 409A001C  bne cr6, 0x82c13b2c
	if !ctx.cr[6].eq {
	pc = 0x82C13B2C; continue 'dispatch;
	}
	// 82C13B14: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13B18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13B1C: 419A001C  beq cr6, 0x82c13b38
	if ctx.cr[6].eq {
	pc = 0x82C13B38; continue 'dispatch;
	}
	// 82C13B20: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13B24: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13B28: 419A0008  beq cr6, 0x82c13b30
	if ctx.cr[6].eq {
	pc = 0x82C13B30; continue 'dispatch;
	}
	// 82C13B2C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13B30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13B34: 409AFFB8  bne cr6, 0x82c13aec
	if !ctx.cr[6].eq {
	pc = 0x82C13AEC; continue 'dispatch;
	}
	// 82C13B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C13B3C: 4809591C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13B40 size=208
    let mut pc: u32 = 0x82C13B40;
    'dispatch: loop {
        match pc {
            0x82C13B40 => {
    //   block [0x82C13B40..0x82C13C10)
	// 82C13B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13B44: 480958C5  bl 0x82ca9408
	ctx.lr = 0x82C13B48;
	sub_82CA93D0(ctx, base);
	// 82C13B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13B4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C13B50: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 82C13B54: 4BFFF92D  bl 0x82c13480
	ctx.lr = 0x82C13B58;
	sub_82C13480(ctx, base);
	// 82C13B58: 3BDD001C  addi r30, r29, 0x1c
	ctx.r[30].s64 = ctx.r[29].s64 + 28;
	// 82C13B5C: 83FD001C  lwz r31, 0x1c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13B60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13B64: 419A007C  beq cr6, 0x82c13be0
	if ctx.cr[6].eq {
	pc = 0x82C13BE0; continue 'dispatch;
	}
	// 82C13B68: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82C13B6C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82C13B70: 4BFFF911  bl 0x82c13480
	ctx.lr = 0x82C13B74;
	sub_82C13480(ctx, base);
	// 82C13B74: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C13B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C13B7C: 419A0034  beq cr6, 0x82c13bb0
	if ctx.cr[6].eq {
	pc = 0x82C13BB0; continue 'dispatch;
	}
	// 82C13B80: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C13B84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C13B88: 41990024  bgt cr6, 0x82c13bac
	if ctx.cr[6].gt {
	pc = 0x82C13BAC; continue 'dispatch;
	}
	// 82C13B8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13B94: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C13B98: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13B9C: 4E800421  bctrl
	ctx.lr = 0x82C13BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13BA0: 813F003C  lwz r9, 0x3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C13BA4: 9869000C  stb r3, 0xc(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[3].u8 ) };
	// 82C13BA8: 48000008  b 0x82c13bb0
	pc = 0x82C13BB0; continue 'dispatch;
	// 82C13BAC: 9B8B000C  stb r28, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u8 ) };
	// 82C13BB0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13BB4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13BB8: 409A001C  bne cr6, 0x82c13bd4
	if !ctx.cr[6].eq {
	pc = 0x82C13BD4; continue 'dispatch;
	}
	// 82C13BBC: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13BC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13BC4: 419A001C  beq cr6, 0x82c13be0
	if ctx.cr[6].eq {
	pc = 0x82C13BE0; continue 'dispatch;
	}
	// 82C13BC8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13BCC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13BD0: 419A0008  beq cr6, 0x82c13bd8
	if ctx.cr[6].eq {
	pc = 0x82C13BD8; continue 'dispatch;
	}
	// 82C13BD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13BD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13BDC: 409AFF90  bne cr6, 0x82c13b6c
	if !ctx.cr[6].eq {
	pc = 0x82C13B6C; continue 'dispatch;
	}
	// 82C13BE0: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C13BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C13BE8: 419A0020  beq cr6, 0x82c13c08
	if ctx.cr[6].eq {
	pc = 0x82C13C08; continue 'dispatch;
	}
	// 82C13BEC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13BF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C13BF4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13BF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13BFC: 4E800421  bctrl
	ctx.lr = 0x82C13C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13C00: 813D0038  lwz r9, 0x38(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C13C04: 98690008  stb r3, 8(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[3].u8 ) };
	// 82C13C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C13C0C: 4809584C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13C10 size=104
    let mut pc: u32 = 0x82C13C10;
    'dispatch: loop {
        match pc {
            0x82C13C10 => {
    //   block [0x82C13C10..0x82C13C78)
	// 82C13C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13C14: 480957F9  bl 0x82ca940c
	ctx.lr = 0x82C13C18;
	sub_82CA93D0(ctx, base);
	// 82C13C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13C1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C13C20: 3BDD001C  addi r30, r29, 0x1c
	ctx.r[30].s64 = ctx.r[29].s64 + 28;
	// 82C13C24: 83FD001C  lwz r31, 0x1c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13C28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13C2C: 419A003C  beq cr6, 0x82c13c68
	if ctx.cr[6].eq {
	pc = 0x82C13C68; continue 'dispatch;
	}
	// 82C13C30: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82C13C34: 4BFFF8AD  bl 0x82c134e0
	ctx.lr = 0x82C13C38;
	sub_82C134E0(ctx, base);
	// 82C13C38: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13C3C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13C40: 409A001C  bne cr6, 0x82c13c5c
	if !ctx.cr[6].eq {
	pc = 0x82C13C5C; continue 'dispatch;
	}
	// 82C13C44: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C13C48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13C4C: 419A001C  beq cr6, 0x82c13c68
	if ctx.cr[6].eq {
	pc = 0x82C13C68; continue 'dispatch;
	}
	// 82C13C50: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13C54: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13C58: 419A0008  beq cr6, 0x82c13c60
	if ctx.cr[6].eq {
	pc = 0x82C13C60; continue 'dispatch;
	}
	// 82C13C5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13C60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13C64: 409AFFCC  bne cr6, 0x82c13c30
	if !ctx.cr[6].eq {
	pc = 0x82C13C30; continue 'dispatch;
	}
	// 82C13C68: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 82C13C6C: 4BFFF875  bl 0x82c134e0
	ctx.lr = 0x82C13C70;
	sub_82C134E0(ctx, base);
	// 82C13C70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13C74: 480957E8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13C78 size=180
    let mut pc: u32 = 0x82C13C78;
    'dispatch: loop {
        match pc {
            0x82C13C78 => {
    //   block [0x82C13C78..0x82C13D2C)
	// 82C13C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13C80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C13C84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C13C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13C90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13C94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C13C98: 392BACD4  addi r9, r11, -0x532c
	ctx.r[9].s64 = ctx.r[11].s64 + -21292;
	// 82C13C9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C13CA0: 390AA3F8  addi r8, r10, -0x5c08
	ctx.r[8].s64 = ctx.r[10].s64 + -23560;
	// 82C13CA4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C13CA8: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82C13CAC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C13CB0: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C13CB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C13CB8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C13CBC: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	// 82C13CC0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C13CC4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C13CC8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C13CCC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82C13CD0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82C13CD4: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82C13CD8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13D30 size=192
    let mut pc: u32 = 0x82C13D30;
    'dispatch: loop {
        match pc {
            0x82C13D30 => {
    //   block [0x82C13D30..0x82C13DF0)
	// 82C13D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13D34: 480956D9  bl 0x82ca940c
	ctx.lr = 0x82C13D38;
	sub_82CA93D0(ctx, base);
	// 82C13D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13D3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13D40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13D44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C13D48: 394BACD4  addi r10, r11, -0x532c
	ctx.r[10].s64 = ctx.r[11].s64 + -21292;
	// 82C13D4C: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C13D50: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C13D54: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C13D58: 419A002C  beq cr6, 0x82c13d84
	if ctx.cr[6].eq {
	pc = 0x82C13D84; continue 'dispatch;
	}
	// 82C13D5C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13D60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13D64: 419A0014  beq cr6, 0x82c13d78
	if ctx.cr[6].eq {
	pc = 0x82C13D78; continue 'dispatch;
	}
	// 82C13D68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13D6C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13D70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13D74: 4E800421  bctrl
	ctx.lr = 0x82C13D78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13D78: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C13D7C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C13D80: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82C13D84: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C13D88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C13D8C: 419A001C  beq cr6, 0x82c13da8
	if ctx.cr[6].eq {
	pc = 0x82C13DA8; continue 'dispatch;
	}
	// 82C13D90: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C13D94: 386B0048  addi r3, r11, 0x48
	ctx.r[3].s64 = ctx.r[11].s64 + 72;
	// 82C13D98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C13D9C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C13DA0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82C13DA4: 4BFFE2F5  bl 0x82c12098
	ctx.lr = 0x82C13DA8;
	sub_82C12098(ctx, base);
	// 82C13DA8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C13DAC: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	// 82C13DB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13DB4: 419A0014  beq cr6, 0x82c13dc8
	if ctx.cr[6].eq {
	pc = 0x82C13DC8; continue 'dispatch;
	}
	// 82C13DB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13DBC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13DC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13DC4: 4E800421  bctrl
	ctx.lr = 0x82C13DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13DC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13DCC: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C13DD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C13DD4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C13DD8: 392BA37C  addi r9, r11, -0x5c84
	ctx.r[9].s64 = ctx.r[11].s64 + -23684;
	// 82C13DDC: 390AD0F8  addi r8, r10, -0x2f08
	ctx.r[8].s64 = ctx.r[10].s64 + -12040;
	// 82C13DE0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C13DE4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C13DE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13DEC: 48095670  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13DF0 size=160
    let mut pc: u32 = 0x82C13DF0;
    'dispatch: loop {
        match pc {
            0x82C13DF0 => {
    //   block [0x82C13DF0..0x82C13E90)
	// 82C13DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13DF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C13DFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C13E00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13E04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13E08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13E0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C13E10: 394BACDC  addi r10, r11, -0x5324
	ctx.r[10].s64 = ctx.r[11].s64 + -21284;
	// 82C13E14: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C13E18: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C13E1C: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 82C13E20: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C13E24: 3909AC9C  addi r8, r9, -0x5364
	ctx.r[8].s64 = ctx.r[9].s64 + -21348;
	// 82C13E28: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C13E2C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C13E30: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C13E34: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82C13E38: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C13E3C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C13E40: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82C13E44: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82C13E48: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82C13E4C: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82C13E50: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82C13E54: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82C13E58: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C13E5C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82C13E60: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82C13E64: 4B5E399D  bl 0x821f7800
	ctx.lr = 0x82C13E68;
	sub_821F7800(ctx, base);
	// 82C13E68: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82C13E6C: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82C13E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13E74: 93FF0030  stw r31, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82C13E78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C13E84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C13E88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C13E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13E90 size=224
    let mut pc: u32 = 0x82C13E90;
    'dispatch: loop {
        match pc {
            0x82C13E90 => {
    //   block [0x82C13E90..0x82C13F70)
	// 82C13E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13E94: 48095575  bl 0x82ca9408
	ctx.lr = 0x82C13E98;
	sub_82CA93D0(ctx, base);
	// 82C13E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13EA4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C13EA8: 394BACDC  addi r10, r11, -0x5324
	ctx.r[10].s64 = ctx.r[11].s64 + -21284;
	// 82C13EAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13EB0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C13EB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13EB8: 419A001C  beq cr6, 0x82c13ed4
	if ctx.cr[6].eq {
	pc = 0x82C13ED4; continue 'dispatch;
	}
	// 82C13EBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13EC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C13EC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13EC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13ECC: 4E800421  bctrl
	ctx.lr = 0x82C13ED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13ED0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C13ED4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C13ED8: 3BBF0048  addi r29, r31, 0x48
	ctx.r[29].s64 = ctx.r[31].s64 + 72;
	// 82C13EDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13EE0: 419A0014  beq cr6, 0x82c13ef4
	if ctx.cr[6].eq {
	pc = 0x82C13EF4; continue 'dispatch;
	}
	// 82C13EE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13EE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13EEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13EF0: 4E800421  bctrl
	ctx.lr = 0x82C13EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13EF4: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C13EF8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 82C13EFC: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C13F00: 4B600ED9  bl 0x82214dd8
	ctx.lr = 0x82C13F04;
	sub_82214DD8(ctx, base);
	// 82C13F04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13F08: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82C13F0C: 394BA37C  addi r10, r11, -0x5c84
	ctx.r[10].s64 = ctx.r[11].s64 + -23684;
	// 82C13F10: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82C13F14: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C13F18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13F1C: 419A0014  beq cr6, 0x82c13f30
	if ctx.cr[6].eq {
	pc = 0x82C13F30; continue 'dispatch;
	}
	// 82C13F20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13F24: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13F28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13F2C: 4E800421  bctrl
	ctx.lr = 0x82C13F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13F30: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C13F34: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 82C13F38: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C13F3C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13F40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13F44: 419A0014  beq cr6, 0x82c13f58
	if ctx.cr[6].eq {
	pc = 0x82C13F58; continue 'dispatch;
	}
	// 82C13F48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13F4C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13F50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13F54: 4E800421  bctrl
	ctx.lr = 0x82C13F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13F58: 93DC0004  stw r30, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C13F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13F60: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C13F64: 4BFF21AD  bl 0x82c06110
	ctx.lr = 0x82C13F68;
	sub_82C06110(ctx, base);
	// 82C13F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C13F6C: 480954EC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C13F70 size=280
    let mut pc: u32 = 0x82C13F70;
    'dispatch: loop {
        match pc {
            0x82C13F70 => {
    //   block [0x82C13F70..0x82C14088)
	// 82C13F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13F74: 48095499  bl 0x82ca940c
	ctx.lr = 0x82C13F78;
	sub_82CA93D0(ctx, base);
	// 82C13F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13F80: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C13F84: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C13F88: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82C13F8C: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C13F90: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82C13F94: 419A0098  beq cr6, 0x82c1402c
	if ctx.cr[6].eq {
	pc = 0x82C1402C; continue 'dispatch;
	}
	// 82C13F98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C13F9C: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82C13FA0: 4BFF2001  bl 0x82c05fa0
	ctx.lr = 0x82C13FA4;
	sub_82C05FA0(ctx, base);
	// 82C13FA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C13FA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C13FAC: 4BFFDD1D  bl 0x82c11cc8
	ctx.lr = 0x82C13FB0;
	sub_82C11CC8(ctx, base);
	// 82C13FB0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C13FB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13FB8: 419A0014  beq cr6, 0x82c13fcc
	if ctx.cr[6].eq {
	pc = 0x82C13FCC; continue 'dispatch;
	}
	// 82C13FBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13FC0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13FC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13FC8: 4E800421  bctrl
	ctx.lr = 0x82C13FCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13FCC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13FD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C13FD4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82C13FD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C13FDC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C13FE0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13FE4: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C13FE8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C13FEC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C13FF0: 4E800421  bctrl
	ctx.lr = 0x82C13FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13FF4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13FF8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C13FFC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C14000: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14004: C0280C14  lfs f1, 0xc14(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C14008: 80C7000C  lwz r6, 0xc(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1400C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C14010: 4E800421  bctrl
	ctx.lr = 0x82C14014;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14014: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1401C: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14020: 8165001C  lwz r11, 0x1c(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C14024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C14028: 4E800421  bctrl
	ctx.lr = 0x82C1402C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1402C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C14030: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82C14034: 4BFF1F6D  bl 0x82c05fa0
	ctx.lr = 0x82C14038;
	sub_82C05FA0(ctx, base);
	// 82C14038: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1403C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C14040: 4BFFDC89  bl 0x82c11cc8
	ctx.lr = 0x82C14044;
	sub_82C11CC8(ctx, base);
	// 82C14044: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C14048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1404C: 419A0014  beq cr6, 0x82c14060
	if ctx.cr[6].eq {
	pc = 0x82C14060; continue 'dispatch;
	}
	// 82C14050: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14054: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14058: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1405C: 4E800421  bctrl
	ctx.lr = 0x82C14060;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14060: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14064: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C14068: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82C1406C: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82C14070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14074: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C14078: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1407C: 4E800421  bctrl
	ctx.lr = 0x82C14080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C14084: 480953D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14088 size=416
    let mut pc: u32 = 0x82C14088;
    'dispatch: loop {
        match pc {
            0x82C14088 => {
    //   block [0x82C14088..0x82C14228)
	// 82C14088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1408C: 48095379  bl 0x82ca9404
	ctx.lr = 0x82C14090;
	sub_82CA93D0(ctx, base);
	// 82C14090: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1409C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C140A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C140A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C140A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C140AC: 409A0024  bne cr6, 0x82c140d0
	if !ctx.cr[6].eq {
	pc = 0x82C140D0; continue 'dispatch;
	}
	// 82C140B0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C140B4: 556A07BC  rlwinm r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C140B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C140BC: 419A0164  beq cr6, 0x82c14220
	if ctx.cr[6].eq {
	pc = 0x82C14220; continue 'dispatch;
	}
	// 82C140C0: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82C140C4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C140C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C140CC: 48095388  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C140D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C140D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C140D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C140DC: 4E800421  bctrl
	ctx.lr = 0x82C140E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C140E0: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C140E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C140E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C140EC: 419A0030  beq cr6, 0x82c1411c
	if ctx.cr[6].eq {
	pc = 0x82C1411C; continue 'dispatch;
	}
	// 82C140F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C140F4: 419A0018  beq cr6, 0x82c1410c
	if ctx.cr[6].eq {
	pc = 0x82C1410C; continue 'dispatch;
	}
	// 82C140F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C140FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C14100: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14104: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14108: 4E800421  bctrl
	ctx.lr = 0x82C1410C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1410C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C14110: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C14114: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C14118: 4809533C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C1411C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14120: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C14124: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14128: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1412C: 4E800421  bctrl
	ctx.lr = 0x82C14130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14130: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14134: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14138: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82C1413C: 3B7F0024  addi r27, r31, 0x24
	ctx.r[27].s64 = ctx.r[31].s64 + 36;
	// 82C14140: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82C14144: 3B8BAB10  addi r28, r11, -0x54f0
	ctx.r[28].s64 = ctx.r[11].s64 + -21744;
	// 82C14148: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C1414C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C14150: 83FF0024  lwz r31, 0x24(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C14154: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C14158: 419A00BC  beq cr6, 0x82c14214
	if ctx.cr[6].eq {
	pc = 0x82C14214; continue 'dispatch;
	}
	// 82C1415C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C14160: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C14164: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C14168: 419A000C  beq cr6, 0x82c14174
	if ctx.cr[6].eq {
	pc = 0x82C14174; continue 'dispatch;
	}
	// 82C1416C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82C14170: 48000024  b 0x82c14194
	pc = 0x82C14194; continue 'dispatch;
	// 82C14174: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1417C: 419A0014  beq cr6, 0x82c14190
	if ctx.cr[6].eq {
	pc = 0x82C14190; continue 'dispatch;
	}
	// 82C14180: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C14184: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82C14188: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C1418C: 409A0008  bne cr6, 0x82c14194
	if !ctx.cr[6].eq {
	pc = 0x82C14194; continue 'dispatch;
	}
	// 82C14190: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82C14194: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14198: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C1419C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C141A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C141A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C141A8: 4E800421  bctrl
	ctx.lr = 0x82C141AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C141AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C141B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C141B4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C141B8: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C141BC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C141C0: 4E800421  bctrl
	ctx.lr = 0x82C141C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C141C4: 5467063E  clrlwi r7, r3, 0x18
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C141C8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C141CC: 419A003C  beq cr6, 0x82c14208
	if ctx.cr[6].eq {
	pc = 0x82C14208; continue 'dispatch;
	}
	// 82C141D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C141D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C141D8: 4BFFDB81  bl 0x82c11d58
	ctx.lr = 0x82C141DC;
	sub_82C11D58(ctx, base);
	// 82C141DC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C141E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C141E4: 419A001C  beq cr6, 0x82c14200
	if ctx.cr[6].eq {
	pc = 0x82C14200; continue 'dispatch;
	}
	// 82C141E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C141EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C141F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C141F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C141F8: 4E800421  bctrl
	ctx.lr = 0x82C141FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C141FC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C14200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14204: 4BC315AD  bl 0x828457b0
	ctx.lr = 0x82C14208;
	sub_828457B0(ctx, base);
	// 82C14208: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82C1420C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C14210: 409AFF50  bne cr6, 0x82c14160
	if !ctx.cr[6].eq {
	pc = 0x82C14160; continue 'dispatch;
	}
	// 82C14214: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C14218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1421C: 4BFF1EE5  bl 0x82c06100
	ctx.lr = 0x82C14220;
	sub_82C06100(ctx, base);
	// 82C14220: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C14224: 48095230  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14228 size=164
    let mut pc: u32 = 0x82C14228;
    'dispatch: loop {
        match pc {
            0x82C14228 => {
    //   block [0x82C14228..0x82C142CC)
	// 82C14228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1422C: 480951E1  bl 0x82ca940c
	ctx.lr = 0x82C14230;
	sub_82CA93D0(ctx, base);
	// 82C14230: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C14234: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14238: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C1423C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C14240: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C14244: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1424C: 419A0074  beq cr6, 0x82c142c0
	if ctx.cr[6].eq {
	pc = 0x82C142C0; continue 'dispatch;
	}
	// 82C14250: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C14254: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C14258: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1425C: 409A0064  bne cr6, 0x82c142c0
	if !ctx.cr[6].eq {
	pc = 0x82C142C0; continue 'dispatch;
	}
	// 82C14260: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C14264: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 82C14268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1426C: 409A0038  bne cr6, 0x82c142a4
	if !ctx.cr[6].eq {
	pc = 0x82C142A4; continue 'dispatch;
	}
	// 82C14270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14274: 4BFF1D2D  bl 0x82c05fa0
	ctx.lr = 0x82C14278;
	sub_82C05FA0(ctx, base);
	// 82C14278: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1427C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14280: 4BFFDA49  bl 0x82c11cc8
	ctx.lr = 0x82C14284;
	sub_82C11CC8(ctx, base);
	// 82C14284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14288: 4BFF1C71  bl 0x82c05ef8
	ctx.lr = 0x82C1428C;
	sub_82C05EF8(ctx, base);
	// 82C1428C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C14294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14298: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1429C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C142A0: 4E800421  bctrl
	ctx.lr = 0x82C142A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C142A4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C142A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C142AC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C142B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C142B4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C142B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C142BC: 4E800421  bctrl
	ctx.lr = 0x82C142C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C142C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C142C4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C142C8: 48095194  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C142D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C142D0 size=164
    let mut pc: u32 = 0x82C142D0;
    'dispatch: loop {
        match pc {
            0x82C142D0 => {
    //   block [0x82C142D0..0x82C14374)
	// 82C142D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C142D4: 48095139  bl 0x82ca940c
	ctx.lr = 0x82C142D8;
	sub_82CA93D0(ctx, base);
	// 82C142D8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C142DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C142E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C142E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C142E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C142EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C142F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C142F4: 419A0074  beq cr6, 0x82c14368
	if ctx.cr[6].eq {
	pc = 0x82C14368; continue 'dispatch;
	}
	// 82C142F8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C142FC: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C14300: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14304: 409A0064  bne cr6, 0x82c14368
	if !ctx.cr[6].eq {
	pc = 0x82C14368; continue 'dispatch;
	}
	// 82C14308: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1430C: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 82C14310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14314: 409A0038  bne cr6, 0x82c1434c
	if !ctx.cr[6].eq {
	pc = 0x82C1434C; continue 'dispatch;
	}
	// 82C14318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1431C: 4BFF1C85  bl 0x82c05fa0
	ctx.lr = 0x82C14320;
	sub_82C05FA0(ctx, base);
	// 82C14320: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C14324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14328: 4BFFD9A1  bl 0x82c11cc8
	ctx.lr = 0x82C1432C;
	sub_82C11CC8(ctx, base);
	// 82C1432C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14330: 4BFF1BC9  bl 0x82c05ef8
	ctx.lr = 0x82C14334;
	sub_82C05EF8(ctx, base);
	// 82C14334: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14338: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1433C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14340: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C14344: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14348: 4E800421  bctrl
	ctx.lr = 0x82C1434C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1434C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14350: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C14354: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C14358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1435C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14360: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14364: 4E800421  bctrl
	ctx.lr = 0x82C14368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1436C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C14370: 480950EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14378 size=136
    let mut pc: u32 = 0x82C14378;
    'dispatch: loop {
        match pc {
            0x82C14378 => {
    //   block [0x82C14378..0x82C14400)
	// 82C14378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C14380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C14384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C14388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1438C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14390: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14394: 394BAC38  addi r10, r11, -0x53c8
	ctx.r[10].s64 = ctx.r[11].s64 + -21448;
	// 82C14398: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1439C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C143A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C143A4: 419A0014  beq cr6, 0x82c143b8
	if ctx.cr[6].eq {
	pc = 0x82C143B8; continue 'dispatch;
	}
	// 82C143A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C143AC: 4800D8B5  bl 0x82c21c60
	ctx.lr = 0x82C143B0;
	sub_82C21C60(ctx, base);
	// 82C143B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C143B4: 4BC313FD  bl 0x828457b0
	ctx.lr = 0x82C143B8;
	sub_828457B0(ctx, base);
	// 82C143B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C143BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C143C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C143C4: 4BFFE83D  bl 0x82c12c00
	ctx.lr = 0x82C143C8;
	sub_82C12C00(ctx, base);
	// 82C143C8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C143CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C143D0: 419A0018  beq cr6, 0x82c143e8
	if ctx.cr[6].eq {
	pc = 0x82C143E8; continue 'dispatch;
	}
	// 82C143D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C143D8: 4BFFE9F9  bl 0x82c12dd0
	ctx.lr = 0x82C143DC;
	sub_82C12DD0(ctx, base);
	// 82C143DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C143E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C143E4: 409AFFF0  bne cr6, 0x82c143d4
	if !ctx.cr[6].eq {
	pc = 0x82C143D4; continue 'dispatch;
	}
	// 82C143E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C143EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C143F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C143F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C143F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C143FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14400 size=348
    let mut pc: u32 = 0x82C14400;
    'dispatch: loop {
        match pc {
            0x82C14400 => {
    //   block [0x82C14400..0x82C1455C)
	// 82C14400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14404: 48094FFD  bl 0x82ca9400
	ctx.lr = 0x82C14408;
	sub_82CA93D0(ctx, base);
	// 82C14408: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1440C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14410: 8383000C  lwz r28, 0xc(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C14414: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C14418: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82C1441C: 3B4BAB10  addi r26, r11, -0x54f0
	ctx.r[26].s64 = ctx.r[11].s64 + -21744;
	// 82C14420: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82C14424: 3BA3000C  addi r29, r3, 0xc
	ctx.r[29].s64 = ctx.r[3].s64 + 12;
	// 82C14428: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82C1442C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C14430: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82C14434: 419A0114  beq cr6, 0x82c14548
	if ctx.cr[6].eq {
	pc = 0x82C14548; continue 'dispatch;
	}
	// 82C14438: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1443C: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 82C14440: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C14444: 419A000C  beq cr6, 0x82c14450
	if ctx.cr[6].eq {
	pc = 0x82C14450; continue 'dispatch;
	}
	// 82C14448: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82C1444C: 48000024  b 0x82c14470
	pc = 0x82C14470; continue 'dispatch;
	// 82C14450: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14458: 419A0014  beq cr6, 0x82c1446c
	if ctx.cr[6].eq {
	pc = 0x82C1446C; continue 'dispatch;
	}
	// 82C1445C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C14460: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82C14464: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C14468: 409A0008  bne cr6, 0x82c14470
	if !ctx.cr[6].eq {
	pc = 0x82C14470; continue 'dispatch;
	}
	// 82C1446C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82C14470: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14474: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C14478: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1447C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14480: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14484: 4E800421  bctrl
	ctx.lr = 0x82C14488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14488: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1448C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C14490: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14494: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C14498: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C1449C: 4E800421  bctrl
	ctx.lr = 0x82C144A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C144A0: 5467063E  clrlwi r7, r3, 0x18
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C144A4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C144A8: 419A0094  beq cr6, 0x82c1453c
	if ctx.cr[6].eq {
	pc = 0x82C1453C; continue 'dispatch;
	}
	// 82C144AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C144B0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C144B4: 409A005C  bne cr6, 0x82c14510
	if !ctx.cr[6].eq {
	pc = 0x82C14510; continue 'dispatch;
	}
	// 82C144B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C144BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C144C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C144C4: 409A0024  bne cr6, 0x82c144e8
	if !ctx.cr[6].eq {
	pc = 0x82C144E8; continue 'dispatch;
	}
	// 82C144C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C144CC: 409A0010  bne cr6, 0x82c144dc
	if !ctx.cr[6].eq {
	pc = 0x82C144DC; continue 'dispatch;
	}
	// 82C144D0: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C144D4: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C144D8: 4800002C  b 0x82c14504
	pc = 0x82C14504; continue 'dispatch;
	// 82C144DC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C144E0: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C144E4: 48000020  b 0x82c14504
	pc = 0x82C14504; continue 'dispatch;
	// 82C144E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C144EC: 409A0010  bne cr6, 0x82c144fc
	if !ctx.cr[6].eq {
	pc = 0x82C144FC; continue 'dispatch;
	}
	// 82C144F0: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C144F4: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C144F8: 4800000C  b 0x82c14504
	pc = 0x82C14504; continue 'dispatch;
	// 82C144FC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C14500: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C14504: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C14508: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C1450C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C14510: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14514: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14518: 419A001C  beq cr6, 0x82c14534
	if ctx.cr[6].eq {
	pc = 0x82C14534; continue 'dispatch;
	}
	// 82C1451C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14520: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C14524: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14528: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1452C: 4E800421  bctrl
	ctx.lr = 0x82C14530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14530: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C14534: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C14538: 4BC31279  bl 0x828457b0
	ctx.lr = 0x82C1453C;
	sub_828457B0(ctx, base);
	// 82C1453C: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82C14540: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C14544: 409AFEF4  bne cr6, 0x82c14438
	if !ctx.cr[6].eq {
	pc = 0x82C14438; continue 'dispatch;
	}
	// 82C14548: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82C1454C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14550: 4BFF1BB1  bl 0x82c06100
	ctx.lr = 0x82C14554;
	sub_82C06100(ctx, base);
	// 82C14554: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C14558: 48094EF8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14560 size=124
    let mut pc: u32 = 0x82C14560;
    'dispatch: loop {
        match pc {
            0x82C14560 => {
    //   block [0x82C14560..0x82C145DC)
	// 82C14560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14564: 48094EA9  bl 0x82ca940c
	ctx.lr = 0x82C14568;
	sub_82CA93D0(ctx, base);
	// 82C14568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1456C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C14570: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C14574: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C14578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1457C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C14580: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C14584: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14588: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1458C: 419A0048  beq cr6, 0x82c145d4
	if ctx.cr[6].eq {
	pc = 0x82C145D4; continue 'dispatch;
	}
	// 82C14590: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 82C14594: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C14598: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 82C1459C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C145A0: 4BFFDD01  bl 0x82c122a0
	ctx.lr = 0x82C145A4;
	sub_82C122A0(ctx, base);
	// 82C145A4: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C145A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C145AC: 419A0028  beq cr6, 0x82c145d4
	if ctx.cr[6].eq {
	pc = 0x82C145D4; continue 'dispatch;
	}
	// 82C145B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C145B4: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C145B8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C145BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C145C0: 419A0014  beq cr6, 0x82c145d4
	if ctx.cr[6].eq {
	pc = 0x82C145D4; continue 'dispatch;
	}
	// 82C145C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C145C8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C145CC: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C145D0: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C145D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C145D8: 48094E84  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C145E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C145E0 size=112
    let mut pc: u32 = 0x82C145E0;
    'dispatch: loop {
        match pc {
            0x82C145E0 => {
    //   block [0x82C145E0..0x82C14650)
	// 82C145E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C145E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C145E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C145EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C145F0: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C145F4: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82C145F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C145FC: 419A0040  beq cr6, 0x82c1463c
	if ctx.cr[6].eq {
	pc = 0x82C1463C; continue 'dispatch;
	}
	// 82C14600: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C14604: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82C14608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1460C: 4BFFDC95  bl 0x82c122a0
	ctx.lr = 0x82C14610;
	sub_82C122A0(ctx, base);
	// 82C14610: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C14614: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14618: 419A0024  beq cr6, 0x82c1463c
	if ctx.cr[6].eq {
	pc = 0x82C1463C; continue 'dispatch;
	}
	// 82C1461C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14620: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C14624: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C14628: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1462C: 419A0010  beq cr6, 0x82c1463c
	if ctx.cr[6].eq {
	pc = 0x82C1463C; continue 'dispatch;
	}
	// 82C14630: 894B000B  lbz r10, 0xb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82C14634: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C14638: 994B000B  stb r10, 0xb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11 as u32), ctx.r[10].u8 ) };
	// 82C1463C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C14640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C14644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C14648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1464C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14650 size=120
    let mut pc: u32 = 0x82C14650;
    'dispatch: loop {
        match pc {
            0x82C14650 => {
    //   block [0x82C14650..0x82C146C8)
	// 82C14650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C14658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1465C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14660: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14664: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82C14668: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1466C: 419A0048  beq cr6, 0x82c146b4
	if ctx.cr[6].eq {
	pc = 0x82C146B4; continue 'dispatch;
	}
	// 82C14670: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C14674: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82C14678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1467C: 4BFFDC25  bl 0x82c122a0
	ctx.lr = 0x82C14680;
	sub_82C122A0(ctx, base);
	// 82C14680: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C14684: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14688: 419A002C  beq cr6, 0x82c146b4
	if ctx.cr[6].eq {
	pc = 0x82C146B4; continue 'dispatch;
	}
	// 82C1468C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14690: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C14694: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C14698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1469C: 419A0018  beq cr6, 0x82c146b4
	if ctx.cr[6].eq {
	pc = 0x82C146B4; continue 'dispatch;
	}
	// 82C146A0: 894B000B  lbz r10, 0xb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82C146A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C146A8: 419A000C  beq cr6, 0x82c146b4
	if ctx.cr[6].eq {
	pc = 0x82C146B4; continue 'dispatch;
	}
	// 82C146AC: 394A00FF  addi r10, r10, 0xff
	ctx.r[10].s64 = ctx.r[10].s64 + 255;
	// 82C146B0: 994B000B  stb r10, 0xb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11 as u32), ctx.r[10].u8 ) };
	// 82C146B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C146B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C146BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C146C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C146C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C146C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C146C8 size=132
    let mut pc: u32 = 0x82C146C8;
    'dispatch: loop {
        match pc {
            0x82C146C8 => {
    //   block [0x82C146C8..0x82C1474C)
	// 82C146C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C146CC: 48094D3D  bl 0x82ca9408
	ctx.lr = 0x82C146D0;
	sub_82CA93D0(ctx, base);
	// 82C146D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C146D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C146D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C146DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C146E0: 392AAD10  addi r9, r10, -0x52f0
	ctx.r[9].s64 = ctx.r[10].s64 + -21232;
	// 82C146E4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C146E8: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82C146EC: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82C146F0: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82C146F4: 37BC006C  addic. r29, r28, 0x6c
	ctx.xer.ca = (ctx.r[28].u32 > (!(108 as u32)));
	ctx.r[29].s64 = ctx.r[28].s64 + 108;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C146F8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C146FC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C14700: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C14704: 41820018  beq 0x82c1471c
	if ctx.cr[0].eq {
	pc = 0x82C1471C; continue 'dispatch;
	}
	// 82C14708: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1470C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C14710: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14714: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14718: 4E800421  bctrl
	ctx.lr = 0x82C1471C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1471C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14720: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14724: 419A0014  beq cr6, 0x82c14738
	if ctx.cr[6].eq {
	pc = 0x82C14738; continue 'dispatch;
	}
	// 82C14728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1472C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14730: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14734: 4E800421  bctrl
	ctx.lr = 0x82C14738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14738: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C1473C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14740: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C14744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C14748: 48094D10  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14750 size=112
    let mut pc: u32 = 0x82C14750;
    'dispatch: loop {
        match pc {
            0x82C14750 => {
    //   block [0x82C14750..0x82C147C0)
	// 82C14750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C14758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1475C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C14760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14768: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1476C: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82C14770: 394BAD10  addi r10, r11, -0x52f0
	ctx.r[10].s64 = ctx.r[11].s64 + -21232;
	// 82C14774: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14778: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1477C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14780: 419A0014  beq cr6, 0x82c14794
	if ctx.cr[6].eq {
	pc = 0x82C14794; continue 'dispatch;
	}
	// 82C14784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14788: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1478C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14790: 4E800421  bctrl
	ctx.lr = 0x82C14794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C14798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1479C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C147A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C147A4: 4BFF195D  bl 0x82c06100
	ctx.lr = 0x82C147A8;
	sub_82C06100(ctx, base);
	// 82C147A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C147AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C147B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C147B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C147B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C147BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C147C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C147C0 size=180
    let mut pc: u32 = 0x82C147C0;
    'dispatch: loop {
        match pc {
            0x82C147C0 => {
    //   block [0x82C147C0..0x82C14874)
	// 82C147C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C147C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C147C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C147CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C147D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C147D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C147D8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C147DC: 409A0010  bne cr6, 0x82c147ec
	if !ctx.cr[6].eq {
	pc = 0x82C147EC; continue 'dispatch;
	}
	// 82C147E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C147E4: 386B023C  addi r3, r11, 0x23c
	ctx.r[3].s64 = ctx.r[11].s64 + 572;
	// 82C147E8: 48000074  b 0x82c1485c
	pc = 0x82C1485C; continue 'dispatch;
	// 82C147EC: 8083000C  lwz r4, 0xc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C147F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C147F4: 419A0064  beq cr6, 0x82c14858
	if ctx.cr[6].eq {
	pc = 0x82C14858; continue 'dispatch;
	}
	// 82C147F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C147FC: 4BFFEE25  bl 0x82c13620
	ctx.lr = 0x82C14800;
	sub_82C13620(ctx, base);
	// 82C14800: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C14804: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14808: 419A0048  beq cr6, 0x82c14850
	if ctx.cr[6].eq {
	pc = 0x82C14850; continue 'dispatch;
	}
	// 82C1480C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C14810: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82C14814: 409A0014  bne cr6, 0x82c14828
	if !ctx.cr[6].eq {
	pc = 0x82C14828; continue 'dispatch;
	}
	// 82C14818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1481C: 4BFF16DD  bl 0x82c05ef8
	ctx.lr = 0x82C14820;
	sub_82C05EF8(ctx, base);
	// 82C14820: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82C14824: 48000038  b 0x82c1485c
	pc = 0x82C1485C; continue 'dispatch;
	// 82C14828: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C1482C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14830: 419A0020  beq cr6, 0x82c14850
	if ctx.cr[6].eq {
	pc = 0x82C14850; continue 'dispatch;
	}
	// 82C14834: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82C14838: 409A0018  bne cr6, 0x82c14850
	if !ctx.cr[6].eq {
	pc = 0x82C14850; continue 'dispatch;
	}
	// 82C1483C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14840: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 82C14844: 4BFF16B5  bl 0x82c05ef8
	ctx.lr = 0x82C14848;
	sub_82C05EF8(ctx, base);
	// 82C14848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1484C: 48000010  b 0x82c1485c
	pc = 0x82C1485C; continue 'dispatch;
	// 82C14850: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14854: 4BFF16A5  bl 0x82c05ef8
	ctx.lr = 0x82C14858;
	sub_82C05EF8(ctx, base);
	// 82C14858: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1485C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C14860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C14864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C14868: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1486C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C14870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14878 size=76
    let mut pc: u32 = 0x82C14878;
    'dispatch: loop {
        match pc {
            0x82C14878 => {
    //   block [0x82C14878..0x82C148C4)
	// 82C14878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1487C: 48094B91  bl 0x82ca940c
	ctx.lr = 0x82C14880;
	sub_82CA93D0(ctx, base);
	// 82C14880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14884: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14888: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C1488C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C14890: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82C14894: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C14898: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C1489C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C148A0: 4E800421  bctrl
	ctx.lr = 0x82C148A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C148A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C148A8: 419A0014  beq cr6, 0x82c148bc
	if ctx.cr[6].eq {
	pc = 0x82C148BC; continue 'dispatch;
	}
	// 82C148AC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C148B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C148B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C148B8: 4BFFEB49  bl 0x82c13400
	ctx.lr = 0x82C148BC;
	sub_82C13400(ctx, base);
	// 82C148BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C148C0: 48094B9C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C148C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C148C8 size=108
    let mut pc: u32 = 0x82C148C8;
    'dispatch: loop {
        match pc {
            0x82C148C8 => {
    //   block [0x82C148C8..0x82C14934)
	// 82C148C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C148CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C148D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C148D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C148D8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C148DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C148E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C148E4: 409A0010  bne cr6, 0x82c148f4
	if !ctx.cr[6].eq {
	pc = 0x82C148F4; continue 'dispatch;
	}
	// 82C148E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C148EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C148F0: 4800002C  b 0x82c1491c
	pc = 0x82C1491C; continue 'dispatch;
	// 82C148F4: 814B0078  lwz r10, 0x78(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C148F8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C148FC: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C14900: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14904: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C14908: 419A0014  beq cr6, 0x82c1491c
	if ctx.cr[6].eq {
	pc = 0x82C1491C; continue 'dispatch;
	}
	// 82C1490C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14910: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14914: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14918: 4E800421  bctrl
	ctx.lr = 0x82C1491C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1491C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C14924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C14928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1492C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C14930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14938 size=8
    let mut pc: u32 = 0x82C14938;
    'dispatch: loop {
        match pc {
            0x82C14938 => {
    //   block [0x82C14938..0x82C14940)
	// 82C14938: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1493C: 4BFFE774  b 0x82c130b0
	sub_82C130B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C14940 size=152
    let mut pc: u32 = 0x82C14940;
    'dispatch: loop {
        match pc {
            0x82C14940 => {
    //   block [0x82C14940..0x82C149D8)
	// 82C14940: 39600250  li r11, 0x250
	ctx.r[11].s64 = 592;
	// 82C14944: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82C14948: 5489063E  clrlwi r9, r4, 0x18
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82C1494C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C14950: 13E358C7  vcmpequd (lvx128) v31, v3, v11
	tmp.u32 = ctx.r[3].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C149D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C149D8 size=104
    let mut pc: u32 = 0x82C149D8;
    'dispatch: loop {
        match pc {
            0x82C149D8 => {
    //   block [0x82C149D8..0x82C14A40)
	// 82C149D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C149DC: 48094A31  bl 0x82ca940c
	ctx.lr = 0x82C149E0;
	sub_82CA93D0(ctx, base);
	// 82C149E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C149E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C149E8: 3BDD003C  addi r30, r29, 0x3c
	ctx.r[30].s64 = ctx.r[29].s64 + 60;
	// 82C149EC: 83FD003C  lwz r31, 0x3c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C149F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C149F4: 419A003C  beq cr6, 0x82c14a30
	if ctx.cr[6].eq {
	pc = 0x82C14A30; continue 'dispatch;
	}
	// 82C149F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C149FC: 4BFFF215  bl 0x82c13c10
	ctx.lr = 0x82C14A00;
	sub_82C13C10(ctx, base);
	// 82C14A00: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C14A04: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C14A08: 409A001C  bne cr6, 0x82c14a24
	if !ctx.cr[6].eq {
	pc = 0x82C14A24; continue 'dispatch;
	}
	// 82C14A0C: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C14A10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C14A14: 419A001C  beq cr6, 0x82c14a30
	if ctx.cr[6].eq {
	pc = 0x82C14A30; continue 'dispatch;
	}
	// 82C14A18: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C14A1C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C14A20: 419A0008  beq cr6, 0x82c14a28
	if ctx.cr[6].eq {
	pc = 0x82C14A28; continue 'dispatch;
	}
	// 82C14A24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C14A28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C14A2C: 409AFFCC  bne cr6, 0x82c149f8
	if !ctx.cr[6].eq {
	pc = 0x82C149F8; continue 'dispatch;
	}
	// 82C14A30: 387D023C  addi r3, r29, 0x23c
	ctx.r[3].s64 = ctx.r[29].s64 + 572;
	// 82C14A34: 4BFFEAAD  bl 0x82c134e0
	ctx.lr = 0x82C14A38;
	sub_82C134E0(ctx, base);
	// 82C14A38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C14A3C: 48094A20  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14A40 size=28
    let mut pc: u32 = 0x82C14A40;
    'dispatch: loop {
        match pc {
            0x82C14A40 => {
    //   block [0x82C14A40..0x82C14A5C)
	// 82C14A40: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C14A44: 39640054  addi r11, r4, 0x54
	ctx.r[11].s64 = ctx.r[4].s64 + 84;
	// 82C14A48: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C14A4C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C14A50: 8124005C  lwz r9, 0x5c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C14A54: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C14A58: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14A5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14A5C size=12
    let mut pc: u32 = 0x82C14A5C;
    'dispatch: loop {
        match pc {
            0x82C14A5C => {
    //   block [0x82C14A5C..0x82C14A68)
	// 82C14A5C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14A60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14A64: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14A68 size=12
    let mut pc: u32 = 0x82C14A68;
    'dispatch: loop {
        match pc {
            0x82C14A68 => {
    //   block [0x82C14A68..0x82C14A74)
	// 82C14A68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14A6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14A70: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14A74 size=36
    let mut pc: u32 = 0x82C14A74;
    'dispatch: loop {
        match pc {
            0x82C14A74 => {
    //   block [0x82C14A74..0x82C14A98)
	// 82C14A74: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C14A78: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14A7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14A80: 409A0018  bne cr6, 0x82c14a98
	if !ctx.cr[6].eq {
		sub_82C14A98(ctx, base);
		return;
	}
	// 82C14A84: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C14A88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14A8C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C14A90: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C14A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14A98 size=32
    let mut pc: u32 = 0x82C14A98;
    'dispatch: loop {
        match pc {
            0x82C14A98 => {
    //   block [0x82C14A98..0x82C14AB8)
	// 82C14A98: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C14AA0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C14AA4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C14AA8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14AAC: 90880054  stw r4, 0x54(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82C14AB0: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C14AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14AB8 size=28
    let mut pc: u32 = 0x82C14AB8;
    'dispatch: loop {
        match pc {
            0x82C14AB8 => {
    //   block [0x82C14AB8..0x82C14AD4)
	// 82C14AB8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C14ABC: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82C14AC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C14AC4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C14AC8: 81240018  lwz r9, 0x18(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C14ACC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C14AD0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14AD4 size=12
    let mut pc: u32 = 0x82C14AD4;
    'dispatch: loop {
        match pc {
            0x82C14AD4 => {
    //   block [0x82C14AD4..0x82C14AE0)
	// 82C14AD4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14AD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14ADC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14AE0 size=12
    let mut pc: u32 = 0x82C14AE0;
    'dispatch: loop {
        match pc {
            0x82C14AE0 => {
    //   block [0x82C14AE0..0x82C14AEC)
	// 82C14AE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14AE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14AE8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14AEC size=36
    let mut pc: u32 = 0x82C14AEC;
    'dispatch: loop {
        match pc {
            0x82C14AEC => {
    //   block [0x82C14AEC..0x82C14B10)
	// 82C14AEC: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C14AF0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14AF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14AF8: 409A0018  bne cr6, 0x82c14b10
	if !ctx.cr[6].eq {
		sub_82C14B10(ctx, base);
		return;
	}
	// 82C14AFC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C14B00: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14B04: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C14B08: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C14B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C14B10 size=32
    let mut pc: u32 = 0x82C14B10;
    'dispatch: loop {
        match pc {
            0x82C14B10 => {
    //   block [0x82C14B10..0x82C14B30)
	// 82C14B10: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C14B18: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C14B1C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C14B20: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14B24: 90880010  stw r4, 0x10(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82C14B28: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C14B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14B30 size=100
    let mut pc: u32 = 0x82C14B30;
    'dispatch: loop {
        match pc {
            0x82C14B30 => {
    //   block [0x82C14B30..0x82C14B94)
	// 82C14B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C14B38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C14B3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C14B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14B44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C14B48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C14B4C: 897F0029  lbz r11, 0x29(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C14B50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14B54: 409A0028  bne cr6, 0x82c14b7c
	if !ctx.cr[6].eq {
	pc = 0x82C14B7C; continue 'dispatch;
	}
	// 82C14B58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C14B5C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C14B60: 4BFFFFD1  bl 0x82c14b30
	ctx.lr = 0x82C14B64;
	sub_82C14B30(ctx, base);
	// 82C14B64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14B68: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14B6C: 4BC30C45  bl 0x828457b0
	ctx.lr = 0x82C14B70;
	sub_828457B0(ctx, base);
	// 82C14B70: 897F0029  lbz r11, 0x29(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C14B74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14B78: 419AFFE0  beq cr6, 0x82c14b58
	if ctx.cr[6].eq {
	pc = 0x82C14B58; continue 'dispatch;
	}
	// 82C14B7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C14B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C14B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C14B88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C14B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C14B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14B98 size=100
    let mut pc: u32 = 0x82C14B98;
    'dispatch: loop {
        match pc {
            0x82C14B98 => {
    //   block [0x82C14B98..0x82C14BFC)
	// 82C14B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14B9C: 48094871  bl 0x82ca940c
	ctx.lr = 0x82C14BA0;
	sub_82CA93D0(ctx, base);
	// 82C14BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14BA4: F8610080  std r3, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[3].u64 ) };
	// 82C14BA8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82C14BAC: F8810088  std r4, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[4].u64 ) };
	// 82C14BB0: 83A1008C  lwz r29, 0x8c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C14BB4: 83C10088  lwz r30, 0x88(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C14BB8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C14BBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14BC0: 419A000C  beq cr6, 0x82c14bcc
	if ctx.cr[6].eq {
	pc = 0x82C14BCC; continue 'dispatch;
	}
	// 82C14BC4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C14BC8: 419A0008  beq cr6, 0x82c14bd0
	if ctx.cr[6].eq {
	pc = 0x82C14BD0; continue 'dispatch;
	}
	// 82C14BCC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C14BD0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C14BD4: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C14BD8: 419A001C  beq cr6, 0x82c14bf4
	if ctx.cr[6].eq {
	pc = 0x82C14BF4; continue 'dispatch;
	}
	// 82C14BDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14BE0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82C14BE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C14BE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C14BEC: 4BFFD60D  bl 0x82c121f8
	ctx.lr = 0x82C14BF0;
	sub_82C121F8(ctx, base);
	// 82C14BF0: 4BFFFFC8  b 0x82c14bb8
	pc = 0x82C14BB8; continue 'dispatch;
	// 82C14BF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C14BF8: 48094864  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14C00 size=96
    let mut pc: u32 = 0x82C14C00;
    'dispatch: loop {
        match pc {
            0x82C14C00 => {
    //   block [0x82C14C00..0x82C14C60)
	// 82C14C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C14C08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C14C0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C14C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14C14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14C18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14C1C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C14C20: 394BAC68  addi r10, r11, -0x5398
	ctx.r[10].s64 = ctx.r[11].s64 + -21400;
	// 82C14C24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C14C28: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14C2C: 48038F75  bl 0x82c4dba0
	ctx.lr = 0x82C14C30;
	sub_82C4DBA0(ctx, base);
	// 82C14C30: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C14C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14C38: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C14C3C: 419A000C  beq cr6, 0x82c14c48
	if ctx.cr[6].eq {
	pc = 0x82C14C48; continue 'dispatch;
	}
	// 82C14C40: 4BC30B71  bl 0x828457b0
	ctx.lr = 0x82C14C44;
	sub_828457B0(ctx, base);
	// 82C14C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14C48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C14C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C14C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C14C54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C14C58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C14C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14C60 size=140
    let mut pc: u32 = 0x82C14C60;
    'dispatch: loop {
        match pc {
            0x82C14C60 => {
    //   block [0x82C14C60..0x82C14CEC)
	// 82C14C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14C64: 480947A5  bl 0x82ca9408
	ctx.lr = 0x82C14C68;
	sub_82CA93D0(ctx, base);
	// 82C14C68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14C70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14C74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C14C78: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C14C7C: 48038E15  bl 0x82c4da90
	ctx.lr = 0x82C14C80;
	sub_82C4DA90(ctx, base);
	// 82C14C80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C14C84: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C14C88: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82C14C8C: 4B5E2B75  bl 0x821f7800
	ctx.lr = 0x82C14C90;
	sub_821F7800(ctx, base);
	// 82C14C90: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82C14C94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C14C98: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C14C9C: 4B650505  bl 0x822651a0
	ctx.lr = 0x82C14CA0;
	sub_822651A0(ctx, base);
	// 82C14CA0: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82C14CA4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C14CA8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C14CAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C14CB0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C14CB4: 48038525  bl 0x82c4d1d8
	ctx.lr = 0x82C14CB8;
	sub_82C4D1D8(ctx, base);
	// 82C14CB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C14CBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14CC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14CC4: 409A0014  bne cr6, 0x82c14cd8
	if !ctx.cr[6].eq {
	pc = 0x82C14CD8; continue 'dispatch;
	}
	// 82C14CC8: 4BFFCAA1  bl 0x82c11768
	ctx.lr = 0x82C14CCC;
	sub_82C11768(ctx, base);
	// 82C14CCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C14CD0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C14CD4: 48094784  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C14CD8: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C14CDC: 4BFFCA8D  bl 0x82c11768
	ctx.lr = 0x82C14CE0;
	sub_82C11768(ctx, base);
	// 82C14CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14CE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C14CE8: 48094770  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14CF0 size=268
    let mut pc: u32 = 0x82C14CF0;
    'dispatch: loop {
        match pc {
            0x82C14CF0 => {
    //   block [0x82C14CF0..0x82C14DFC)
	// 82C14CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14CF4: 48094715  bl 0x82ca9408
	ctx.lr = 0x82C14CF8;
	sub_82CA93D0(ctx, base);
	// 82C14CF8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14CFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C14D00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14D04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C14D08: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C14D0C: 48038D85  bl 0x82c4da90
	ctx.lr = 0x82C14D10;
	sub_82C4DA90(ctx, base);
	// 82C14D10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C14D14: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C14D18: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82C14D1C: 4B5E2AE5  bl 0x821f7800
	ctx.lr = 0x82C14D20;
	sub_821F7800(ctx, base);
	// 82C14D20: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82C14D24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C14D28: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C14D2C: 4B650475  bl 0x822651a0
	ctx.lr = 0x82C14D30;
	sub_822651A0(ctx, base);
	// 82C14D30: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82C14D34: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82C14D38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C14D3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C14D40: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C14D44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C14D48: 48038491  bl 0x82c4d1d8
	ctx.lr = 0x82C14D4C;
	sub_82C4D1D8(ctx, base);
	// 82C14D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14D50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C14D54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C14D58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C14D5C: 48038CDD  bl 0x82c4da38
	ctx.lr = 0x82C14D60;
	sub_82C4DA38(ctx, base);
	// 82C14D60: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C14D64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14D68: 419A0018  beq cr6, 0x82c14d80
	if ctx.cr[6].eq {
	pc = 0x82C14D80; continue 'dispatch;
	}
	// 82C14D6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14D70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C14D74: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14D78: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14D7C: 4E800421  bctrl
	ctx.lr = 0x82C14D80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14D80: 5783003E  slwi r3, r28, 0
	ctx.r[3].u32 = ctx.r[28].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C14D84: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82C14D88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14D8C: 419A001C  beq cr6, 0x82c14da8
	if ctx.cr[6].eq {
	pc = 0x82C14DA8; continue 'dispatch;
	}
	// 82C14D90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14D94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C14D98: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14D9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14DA0: 4E800421  bctrl
	ctx.lr = 0x82C14DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14DA4: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82C14DA8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C14DAC: 4B60002D  bl 0x82214dd8
	ctx.lr = 0x82C14DB0;
	sub_82214DD8(ctx, base);
	// 82C14DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14DB4: 4BF820C5  bl 0x82b96e78
	ctx.lr = 0x82C14DB8;
	sub_82B96E78(ctx, base);
	// 82C14DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C14DBC: 4BC309F5  bl 0x828457b0
	ctx.lr = 0x82C14DC0;
	sub_828457B0(ctx, base);
	// 82C14DC0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C14DC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14DC8: 419A001C  beq cr6, 0x82c14de4
	if ctx.cr[6].eq {
	pc = 0x82C14DE4; continue 'dispatch;
	}
	// 82C14DCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14DD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C14DD4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14DD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14DDC: 4E800421  bctrl
	ctx.lr = 0x82C14DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14DE0: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82C14DE4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C14DE8: 4B5FFFF1  bl 0x82214dd8
	ctx.lr = 0x82C14DEC;
	sub_82214DD8(ctx, base);
	// 82C14DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C14DF0: 4BF82089  bl 0x82b96e78
	ctx.lr = 0x82C14DF4;
	sub_82B96E78(ctx, base);
	// 82C14DF4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C14DF8: 48094660  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14E00 size=264
    let mut pc: u32 = 0x82C14E00;
    'dispatch: loop {
        match pc {
            0x82C14E00 => {
    //   block [0x82C14E00..0x82C14F08)
	// 82C14E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14E04: 48094605  bl 0x82ca9408
	ctx.lr = 0x82C14E08;
	sub_82CA93D0(ctx, base);
	// 82C14E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C14E10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14E14: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C14E18: 394BACA8  addi r10, r11, -0x5358
	ctx.r[10].s64 = ctx.r[11].s64 + -21336;
	// 82C14E1C: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C14E20: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14E24: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C14E28: 419A002C  beq cr6, 0x82c14e54
	if ctx.cr[6].eq {
	pc = 0x82C14E54; continue 'dispatch;
	}
	// 82C14E2C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14E34: 419A0014  beq cr6, 0x82c14e48
	if ctx.cr[6].eq {
	pc = 0x82C14E48; continue 'dispatch;
	}
	// 82C14E38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14E3C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14E40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14E44: 4E800421  bctrl
	ctx.lr = 0x82C14E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14E48: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C14E4C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C14E50: 93BF0084  stw r29, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82C14E54: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C14E58: 3B9F0078  addi r28, r31, 0x78
	ctx.r[28].s64 = ctx.r[31].s64 + 120;
	// 82C14E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14E60: 419A001C  beq cr6, 0x82c14e7c
	if ctx.cr[6].eq {
	pc = 0x82C14E7C; continue 'dispatch;
	}
	// 82C14E64: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C14E68: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 82C14E6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C14E70: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C14E74: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82C14E78: 4BFFD189  bl 0x82c12000
	ctx.lr = 0x82C14E7C;
	sub_82C12000(ctx, base);
	// 82C14E7C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C14E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14E84: 419A0010  beq cr6, 0x82c14e94
	if ctx.cr[6].eq {
	pc = 0x82C14E94; continue 'dispatch;
	}
	// 82C14E88: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C14E8C: 4BFFF7C5  bl 0x82c14650
	ctx.lr = 0x82C14E90;
	sub_82C14650(ctx, base);
	// 82C14E90: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82C14E94: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14E98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14E9C: 419A0014  beq cr6, 0x82c14eb0
	if ctx.cr[6].eq {
	pc = 0x82C14EB0; continue 'dispatch;
	}
	// 82C14EA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14EA4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14EA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14EAC: 4E800421  bctrl
	ctx.lr = 0x82C14EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14EB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14EB4: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C14EB8: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C14EBC: 3BDF004C  addi r30, r31, 0x4c
	ctx.r[30].s64 = ctx.r[31].s64 + 76;
	// 82C14EC0: 394BA37C  addi r10, r11, -0x5c84
	ctx.r[10].s64 = ctx.r[11].s64 + -23684;
	// 82C14EC4: 915F006C  stw r10, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82C14EC8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C14ECC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14ED0: 419A0014  beq cr6, 0x82c14ee4
	if ctx.cr[6].eq {
	pc = 0x82C14EE4; continue 'dispatch;
	}
	// 82C14ED4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14ED8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14EDC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14EE0: 4E800421  bctrl
	ctx.lr = 0x82C14EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14EE4: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C14EE8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C14EEC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C14EF0: 4B674B59  bl 0x82289a48
	ctx.lr = 0x82C14EF4;
	sub_82289A48(ctx, base);
	// 82C14EF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14EF8: 394BA388  addi r10, r11, -0x5c78
	ctx.r[10].s64 = ctx.r[11].s64 + -23672;
	// 82C14EFC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C14F04: 48094554  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C14F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C14F08 size=416
    let mut pc: u32 = 0x82C14F08;
    'dispatch: loop {
        match pc {
            0x82C14F08 => {
    //   block [0x82C14F08..0x82C150A8)
	// 82C14F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C14F0C: 480944F5  bl 0x82ca9400
	ctx.lr = 0x82C14F10;
	sub_82CA93D0(ctx, base);
	// 82C14F10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C14F14: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C14F18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C14F1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C14F20: 394BAD68  addi r10, r11, -0x5298
	ctx.r[10].s64 = ctx.r[11].s64 + -21144;
	// 82C14F24: 83FB003C  lwz r31, 0x3c(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C14F28: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C14F2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C14F30: 419A002C  beq cr6, 0x82c14f5c
	if ctx.cr[6].eq {
	pc = 0x82C14F5C; continue 'dispatch;
	}
	// 82C14F34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14F38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14F3C: 419A0014  beq cr6, 0x82c14f50
	if ctx.cr[6].eq {
	pc = 0x82C14F50; continue 'dispatch;
	}
	// 82C14F40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14F44: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14F48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14F4C: 4E800421  bctrl
	ctx.lr = 0x82C14F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14F50: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C14F54: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C14F58: 939B003C  stw r28, 0x3c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(60 as u32), ctx.r[28].u32 ) };
	// 82C14F5C: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C14F60: 3B5B0034  addi r26, r27, 0x34
	ctx.r[26].s64 = ctx.r[27].s64 + 52;
	// 82C14F64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14F68: 419A001C  beq cr6, 0x82c14f84
	if ctx.cr[6].eq {
	pc = 0x82C14F84; continue 'dispatch;
	}
	// 82C14F6C: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C14F70: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 82C14F74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C14F78: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C14F7C: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82C14F80: 4BFFD119  bl 0x82c12098
	ctx.lr = 0x82C14F84;
	sub_82C12098(ctx, base);
	// 82C14F84: 83BB001C  lwz r29, 0x1c(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C14F88: 3BFB001C  addi r31, r27, 0x1c
	ctx.r[31].s64 = ctx.r[27].s64 + 28;
	// 82C14F8C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C14F90: 419A00AC  beq cr6, 0x82c1503c
	if ctx.cr[6].eq {
	pc = 0x82C1503C; continue 'dispatch;
	}
	// 82C14F94: 807D007C  lwz r3, 0x7c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C14F98: 3BDD0078  addi r30, r29, 0x78
	ctx.r[30].s64 = ctx.r[29].s64 + 120;
	// 82C14F9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C14FA0: 419A0014  beq cr6, 0x82c14fb4
	if ctx.cr[6].eq {
	pc = 0x82C14FB4; continue 'dispatch;
	}
	// 82C14FA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14FA8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14FAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C14FB0: 4E800421  bctrl
	ctx.lr = 0x82C14FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C14FB4: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C14FB8: 393D0054  addi r9, r29, 0x54
	ctx.r[9].s64 = ctx.r[29].s64 + 84;
	// 82C14FBC: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C14FC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C14FC4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C14FC8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C14FCC: 815D005C  lwz r10, 0x5c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C14FD0: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C14FD4: 409A005C  bne cr6, 0x82c15030
	if !ctx.cr[6].eq {
	pc = 0x82C15030; continue 'dispatch;
	}
	// 82C14FD8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C14FDC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C14FE0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C14FE4: 409A0024  bne cr6, 0x82c15008
	if !ctx.cr[6].eq {
	pc = 0x82C15008; continue 'dispatch;
	}
	// 82C14FE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C14FEC: 409A0010  bne cr6, 0x82c14ffc
	if !ctx.cr[6].eq {
	pc = 0x82C14FFC; continue 'dispatch;
	}
	// 82C14FF0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C14FF4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C14FF8: 4800002C  b 0x82c15024
	pc = 0x82C15024; continue 'dispatch;
	// 82C14FFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C15000: 938B0058  stw r28, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82C15004: 48000020  b 0x82c15024
	pc = 0x82C15024; continue 'dispatch;
	// 82C15008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1500C: 409A0010  bne cr6, 0x82c1501c
	if !ctx.cr[6].eq {
	pc = 0x82C1501C; continue 'dispatch;
	}
	// 82C15010: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C15014: 938A0054  stw r28, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82C15018: 4800000C  b 0x82c15024
	pc = 0x82C15024; continue 'dispatch;
	// 82C1501C: 916A0054  stw r11, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C15020: 914B0058  stw r10, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82C15024: 93890004  stw r28, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C15028: 93890000  stw r28, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1502C: 93890008  stw r28, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82C15030: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15034: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C15038: 409AFF5C  bne cr6, 0x82c14f94
	if !ctx.cr[6].eq {
	pc = 0x82C14F94; continue 'dispatch;
	}
	// 82C1503C: 387B0060  addi r3, r27, 0x60
	ctx.r[3].s64 = ctx.r[27].s64 + 96;
	// 82C15040: 4B5FFD99  bl 0x82214dd8
	ctx.lr = 0x82C15044;
	sub_82214DD8(ctx, base);
	// 82C15044: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1504C: 419A0014  beq cr6, 0x82c15060
	if ctx.cr[6].eq {
	pc = 0x82C15060; continue 'dispatch;
	}
	// 82C15050: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15054: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15058: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1505C: 4E800421  bctrl
	ctx.lr = 0x82C15060;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15060: 939A0004  stw r28, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C15064: 3BFB0028  addi r31, r27, 0x28
	ctx.r[31].s64 = ctx.r[27].s64 + 40;
	// 82C15068: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1506C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15070: 4BFFE471  bl 0x82c134e0
	ctx.lr = 0x82C15074;
	sub_82C134E0(ctx, base);
	// 82C15074: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15078: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1507C: 394BAC68  addi r10, r11, -0x5398
	ctx.r[10].s64 = ctx.r[11].s64 + -21400;
	// 82C15080: 915B0028  stw r10, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82C15084: 48038B1D  bl 0x82c4dba0
	ctx.lr = 0x82C15088;
	sub_82C4DBA0(ctx, base);
	// 82C15088: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C1508C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C15090: 38E9A37C  addi r7, r9, -0x5c84
	ctx.r[7].s64 = ctx.r[9].s64 + -23684;
	// 82C15094: 38C8A3B0  addi r6, r8, -0x5c50
	ctx.r[6].s64 = ctx.r[8].s64 + -23632;
	// 82C15098: 90FB0004  stw r7, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C1509C: 90DB0000  stw r6, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82C150A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C150A4: 480943AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C150A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C150A8 size=376
    let mut pc: u32 = 0x82C150A8;
    'dispatch: loop {
        match pc {
            0x82C150A8 => {
    //   block [0x82C150A8..0x82C15220)
	// 82C150A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C150AC: 48094359  bl 0x82ca9404
	ctx.lr = 0x82C150B0;
	sub_82CA93D0(ctx, base);
	// 82C150B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C150B4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C150B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C150BC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C150C0: 394BAD84  addi r10, r11, -0x527c
	ctx.r[10].s64 = ctx.r[11].s64 + -21116;
	// 82C150C4: 83FB0038  lwz r31, 0x38(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C150C8: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C150CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C150D0: 419A002C  beq cr6, 0x82c150fc
	if ctx.cr[6].eq {
	pc = 0x82C150FC; continue 'dispatch;
	}
	// 82C150D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C150D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C150DC: 419A0014  beq cr6, 0x82c150f0
	if ctx.cr[6].eq {
	pc = 0x82C150F0; continue 'dispatch;
	}
	// 82C150E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C150E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C150E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C150EC: 4E800421  bctrl
	ctx.lr = 0x82C150F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C150F0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C150F4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C150F8: 939B0038  stw r28, 0x38(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(56 as u32), ctx.r[28].u32 ) };
	// 82C150FC: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C15100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C15104: 419A001C  beq cr6, 0x82c15120
	if ctx.cr[6].eq {
	pc = 0x82C15120; continue 'dispatch;
	}
	// 82C15108: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C1510C: 386B003C  addi r3, r11, 0x3c
	ctx.r[3].s64 = ctx.r[11].s64 + 60;
	// 82C15110: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C15114: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C15118: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82C1511C: 4BFFCF7D  bl 0x82c12098
	ctx.lr = 0x82C15120;
	sub_82C12098(ctx, base);
	// 82C15120: 83BB001C  lwz r29, 0x1c(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C15124: 3BFB001C  addi r31, r27, 0x1c
	ctx.r[31].s64 = ctx.r[27].s64 + 28;
	// 82C15128: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C1512C: 419A00AC  beq cr6, 0x82c151d8
	if ctx.cr[6].eq {
	pc = 0x82C151D8; continue 'dispatch;
	}
	// 82C15130: 807D0038  lwz r3, 0x38(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C15134: 3BDD0034  addi r30, r29, 0x34
	ctx.r[30].s64 = ctx.r[29].s64 + 52;
	// 82C15138: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1513C: 419A0014  beq cr6, 0x82c15150
	if ctx.cr[6].eq {
	pc = 0x82C15150; continue 'dispatch;
	}
	// 82C15140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15144: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15148: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1514C: 4E800421  bctrl
	ctx.lr = 0x82C15150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15150: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C15154: 393D0010  addi r9, r29, 0x10
	ctx.r[9].s64 = ctx.r[29].s64 + 16;
	// 82C15158: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1515C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C15160: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C15164: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C15168: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1516C: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C15170: 409A005C  bne cr6, 0x82c151cc
	if !ctx.cr[6].eq {
	pc = 0x82C151CC; continue 'dispatch;
	}
	// 82C15174: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15178: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1517C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C15180: 409A0024  bne cr6, 0x82c151a4
	if !ctx.cr[6].eq {
	pc = 0x82C151A4; continue 'dispatch;
	}
	// 82C15184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C15188: 409A0010  bne cr6, 0x82c15198
	if !ctx.cr[6].eq {
	pc = 0x82C15198; continue 'dispatch;
	}
	// 82C1518C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C15190: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C15194: 4800002C  b 0x82c151c0
	pc = 0x82C151C0; continue 'dispatch;
	// 82C15198: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1519C: 938B0014  stw r28, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82C151A0: 48000020  b 0x82c151c0
	pc = 0x82C151C0; continue 'dispatch;
	// 82C151A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C151A8: 409A0010  bne cr6, 0x82c151b8
	if !ctx.cr[6].eq {
	pc = 0x82C151B8; continue 'dispatch;
	}
	// 82C151AC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C151B0: 938A0010  stw r28, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82C151B4: 4800000C  b 0x82c151c0
	pc = 0x82C151C0; continue 'dispatch;
	// 82C151B8: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C151BC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82C151C0: 93890004  stw r28, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C151C4: 93890000  stw r28, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C151C8: 93890008  stw r28, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82C151CC: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C151D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C151D4: 409AFF5C  bne cr6, 0x82c15130
	if !ctx.cr[6].eq {
	pc = 0x82C15130; continue 'dispatch;
	}
	// 82C151D8: 387B003C  addi r3, r27, 0x3c
	ctx.r[3].s64 = ctx.r[27].s64 + 60;
	// 82C151DC: 4B5FFBFD  bl 0x82214dd8
	ctx.lr = 0x82C151E0;
	sub_82214DD8(ctx, base);
	// 82C151E0: 3BFB0028  addi r31, r27, 0x28
	ctx.r[31].s64 = ctx.r[27].s64 + 40;
	// 82C151E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C151E8: 4BFFE2F9  bl 0x82c134e0
	ctx.lr = 0x82C151EC;
	sub_82C134E0(ctx, base);
	// 82C151EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C151F0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C151F4: 394BAC68  addi r10, r11, -0x5398
	ctx.r[10].s64 = ctx.r[11].s64 + -21400;
	// 82C151F8: 915B0028  stw r10, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82C151FC: 480389A5  bl 0x82c4dba0
	ctx.lr = 0x82C15200;
	sub_82C4DBA0(ctx, base);
	// 82C15200: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C15204: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C15208: 38E9A37C  addi r7, r9, -0x5c84
	ctx.r[7].s64 = ctx.r[9].s64 + -23684;
	// 82C1520C: 38C8C928  addi r6, r8, -0x36d8
	ctx.r[6].s64 = ctx.r[8].s64 + -14040;
	// 82C15210: 90FB0004  stw r7, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C15214: 90DB0000  stw r6, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82C15218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1521C: 48094238  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15220 size=80
    let mut pc: u32 = 0x82C15220;
    'dispatch: loop {
        match pc {
            0x82C15220 => {
    //   block [0x82C15220..0x82C15270)
	// 82C15220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1522C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15238: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1523C: 4BFFEAF5  bl 0x82c13d30
	ctx.lr = 0x82C15240;
	sub_82C13D30(ctx, base);
	// 82C15240: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C15244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1524C: 419A000C  beq cr6, 0x82c15258
	if ctx.cr[6].eq {
	pc = 0x82C15258; continue 'dispatch;
	}
	// 82C15250: 4BC30561  bl 0x828457b0
	ctx.lr = 0x82C15254;
	sub_828457B0(ctx, base);
	// 82C15254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1525C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1526C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15270 size=80
    let mut pc: u32 = 0x82C15270;
    'dispatch: loop {
        match pc {
            0x82C15270 => {
    //   block [0x82C15270..0x82C152C0)
	// 82C15270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1527C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15288: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1528C: 4BFFEC05  bl 0x82c13e90
	ctx.lr = 0x82C15290;
	sub_82C13E90(ctx, base);
	// 82C15290: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C15294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1529C: 419A000C  beq cr6, 0x82c152a8
	if ctx.cr[6].eq {
	pc = 0x82C152A8; continue 'dispatch;
	}
	// 82C152A0: 4BC30511  bl 0x828457b0
	ctx.lr = 0x82C152A4;
	sub_828457B0(ctx, base);
	// 82C152A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C152A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C152AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C152B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C152B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C152B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C152BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C152C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C152C0 size=80
    let mut pc: u32 = 0x82C152C0;
    'dispatch: loop {
        match pc {
            0x82C152C0 => {
    //   block [0x82C152C0..0x82C15310)
	// 82C152C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C152C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C152C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C152CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C152D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C152D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C152D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C152DC: 4BFFF09D  bl 0x82c14378
	ctx.lr = 0x82C152E0;
	sub_82C14378(ctx, base);
	// 82C152E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C152E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C152E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C152EC: 419A000C  beq cr6, 0x82c152f8
	if ctx.cr[6].eq {
	pc = 0x82C152F8; continue 'dispatch;
	}
	// 82C152F0: 4BC304C1  bl 0x828457b0
	ctx.lr = 0x82C152F4;
	sub_828457B0(ctx, base);
	// 82C152F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C152F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C152FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15304: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1530C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15310 size=720
    let mut pc: u32 = 0x82C15310;
    'dispatch: loop {
        match pc {
            0x82C15310 => {
    //   block [0x82C15310..0x82C155E0)
	// 82C15310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15314: 480940E1  bl 0x82ca93f4
	ctx.lr = 0x82C15318;
	sub_82CA93D0(ctx, base);
	// 82C15318: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82C1531C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15320: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C15324: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82C15328: 38790060  addi r3, r25, 0x60
	ctx.r[3].s64 = ctx.r[25].s64 + 96;
	// 82C1532C: 80BB0008  lwz r5, 8(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C15330: 80990288  lwz r4, 0x288(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C15334: 4803AA55  bl 0x82c4fd88
	ctx.lr = 0x82C15338;
	sub_82C4FD88(ctx, base);
	// 82C15338: 839B0014  lwz r28, 0x14(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1533C: 3B5B0014  addi r26, r27, 0x14
	ctx.r[26].s64 = ctx.r[27].s64 + 20;
	// 82C15340: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C15344: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C15348: 419A028C  beq cr6, 0x82c155d4
	if ctx.cr[6].eq {
	pc = 0x82C155D4; continue 'dispatch;
	}
	// 82C1534C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15350: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C15354: 3B0BAD10  addi r24, r11, -0x52f0
	ctx.r[24].s64 = ctx.r[11].s64 + -21232;
	// 82C15358: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C1535C: 38BC0038  addi r5, r28, 0x38
	ctx.r[5].s64 = ctx.r[28].s64 + 56;
	// 82C15360: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C15364: 419A000C  beq cr6, 0x82c15370
	if ctx.cr[6].eq {
	pc = 0x82C15370; continue 'dispatch;
	}
	// 82C15368: 7FB7EB78  mr r23, r29
	ctx.r[23].u64 = ctx.r[29].u64;
	// 82C1536C: 48000024  b 0x82c15390
	pc = 0x82C15390; continue 'dispatch;
	// 82C15370: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C15378: 419A0014  beq cr6, 0x82c1538c
	if ctx.cr[6].eq {
	pc = 0x82C1538C; continue 'dispatch;
	}
	// 82C1537C: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C15380: 7FB7EB78  mr r23, r29
	ctx.r[23].u64 = ctx.r[29].u64;
	// 82C15384: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C15388: 409A0008  bne cr6, 0x82c15390
	if !ctx.cr[6].eq {
	pc = 0x82C15390; continue 'dispatch;
	}
	// 82C1538C: 7D775B78  mr r23, r11
	ctx.r[23].u64 = ctx.r[11].u64;
	// 82C15390: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C15394: 7D6A58F8  nor r10, r11, r11
	ctx.r[10].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82C15398: 5549F7FE  rlwinm r9, r10, 0x1e, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82C1539C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C153A0: 419A00C8  beq cr6, 0x82c15468
	if ctx.cr[6].eq {
	pc = 0x82C15468; continue 'dispatch;
	}
	// 82C153A4: 83DC0048  lwz r30, 0x48(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C153A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C153AC: 419A00BC  beq cr6, 0x82c15468
	if ctx.cr[6].eq {
	pc = 0x82C15468; continue 'dispatch;
	}
	// 82C153B0: 37FE006C  addic. r31, r30, 0x6c
	ctx.xer.ca = (ctx.r[30].u32 > (!(108 as u32)));
	ctx.r[31].s64 = ctx.r[30].s64 + 108;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82C153B4: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82C153B8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82C153BC: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82C153C0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82C153C4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82C153C8: 41820034  beq 0x82c153fc
	if ctx.cr[0].eq {
	pc = 0x82C153FC; continue 'dispatch;
	}
	// 82C153CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C153D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C153D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C153D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C153DC: 4E800421  bctrl
	ctx.lr = 0x82C153E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C153E0: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C153E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C153E8: 419A0014  beq cr6, 0x82c153fc
	if ctx.cr[6].eq {
	pc = 0x82C153FC; continue 'dispatch;
	}
	// 82C153EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C153F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C153F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C153F8: 4E800421  bctrl
	ctx.lr = 0x82C153FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C153FC: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82C15400: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C15404: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82C15408: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C1540C: 807C001C  lwz r3, 0x1c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C15410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15414: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C15418: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1541C: 4E800421  bctrl
	ctx.lr = 0x82C15420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15420: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82C15424: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82C15428: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C1542C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C15430: 4BFFEC59  bl 0x82c14088
	ctx.lr = 0x82C15434;
	sub_82C14088(ctx, base);
	// 82C15434: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C15438: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82C1543C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15440: 419A0014  beq cr6, 0x82c15454
	if ctx.cr[6].eq {
	pc = 0x82C15454; continue 'dispatch;
	}
	// 82C15444: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15448: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1544C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C15450: 4E800421  bctrl
	ctx.lr = 0x82C15454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15454: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82C15458: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1545C: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82C15460: 4BFF0CA1  bl 0x82c06100
	ctx.lr = 0x82C15464;
	sub_82C06100(ctx, base);
	// 82C15464: 48000164  b 0x82c155c8
	pc = 0x82C155C8; continue 'dispatch;
	// 82C15468: 811C0024  lwz r8, 0x24(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C1546C: 38DC0024  addi r6, r28, 0x24
	ctx.r[6].s64 = ctx.r[28].s64 + 36;
	// 82C15470: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15474: 419A00D8  beq cr6, 0x82c1554c
	if ctx.cr[6].eq {
	pc = 0x82C1554C; continue 'dispatch;
	}
	// 82C15478: 38FB000C  addi r7, r27, 0xc
	ctx.r[7].s64 = ctx.r[27].s64 + 12;
	// 82C1547C: 8148000C  lwz r10, 0xc(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C15480: 39680004  addi r11, r8, 4
	ctx.r[11].s64 = ctx.r[8].s64 + 4;
	// 82C15484: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C15488: 409A005C  bne cr6, 0x82c154e4
	if !ctx.cr[6].eq {
	pc = 0x82C154E4; continue 'dispatch;
	}
	// 82C1548C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15490: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15494: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C15498: 409A0024  bne cr6, 0x82c154bc
	if !ctx.cr[6].eq {
	pc = 0x82C154BC; continue 'dispatch;
	}
	// 82C1549C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C154A0: 409A0010  bne cr6, 0x82c154b0
	if !ctx.cr[6].eq {
	pc = 0x82C154B0; continue 'dispatch;
	}
	// 82C154A4: 93A60000  stw r29, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C154A8: 93A60004  stw r29, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C154AC: 4800002C  b 0x82c154d8
	pc = 0x82C154D8; continue 'dispatch;
	// 82C154B0: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C154B4: 93AA0008  stw r29, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C154B8: 48000020  b 0x82c154d8
	pc = 0x82C154D8; continue 'dispatch;
	// 82C154BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C154C0: 409A0010  bne cr6, 0x82c154d0
	if !ctx.cr[6].eq {
	pc = 0x82C154D0; continue 'dispatch;
	}
	// 82C154C4: 91260004  stw r9, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C154C8: 93A90004  stw r29, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C154CC: 4800000C  b 0x82c154d8
	pc = 0x82C154D8; continue 'dispatch;
	// 82C154D0: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C154D4: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C154D8: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C154DC: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C154E0: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C154E4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C154E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C154EC: 409A0054  bne cr6, 0x82c15540
	if !ctx.cr[6].eq {
	pc = 0x82C15540; continue 'dispatch;
	}
	// 82C154F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C154F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C154F8: 409A0048  bne cr6, 0x82c15540
	if !ctx.cr[6].eq {
	pc = 0x82C15540; continue 'dispatch;
	}
	// 82C154FC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15500: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C15504: 409A003C  bne cr6, 0x82c15540
	if !ctx.cr[6].eq {
	pc = 0x82C15540; continue 'dispatch;
	}
	// 82C15508: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C1550C: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15510: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C15514: 409A0014  bne cr6, 0x82c15528
	if !ctx.cr[6].eq {
	pc = 0x82C15528; continue 'dispatch;
	}
	// 82C15518: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C1551C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C15520: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C15524: 48000018  b 0x82c1553c
	pc = 0x82C1553C; continue 'dispatch;
	// 82C15528: 81470004  lwz r10, 4(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1552C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C15530: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C15534: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15538: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C1553C: 91070004  stw r8, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C15540: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15544: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15548: 409AFF34  bne cr6, 0x82c1547c
	if !ctx.cr[6].eq {
	pc = 0x82C1547C; continue 'dispatch;
	}
	// 82C1554C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C15550: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C15554: 409A005C  bne cr6, 0x82c155b0
	if !ctx.cr[6].eq {
	pc = 0x82C155B0; continue 'dispatch;
	}
	// 82C15558: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1555C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15560: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C15564: 409A0024  bne cr6, 0x82c15588
	if !ctx.cr[6].eq {
	pc = 0x82C15588; continue 'dispatch;
	}
	// 82C15568: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1556C: 409A0010  bne cr6, 0x82c1557c
	if !ctx.cr[6].eq {
	pc = 0x82C1557C; continue 'dispatch;
	}
	// 82C15570: 93BA0000  stw r29, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C15574: 93BA0004  stw r29, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C15578: 4800002C  b 0x82c155a4
	pc = 0x82C155A4; continue 'dispatch;
	// 82C1557C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C15580: 93AB003C  stw r29, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82C15584: 48000020  b 0x82c155a4
	pc = 0x82C155A4; continue 'dispatch;
	// 82C15588: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1558C: 409A0010  bne cr6, 0x82c1559c
	if !ctx.cr[6].eq {
	pc = 0x82C1559C; continue 'dispatch;
	}
	// 82C15590: 915A0004  stw r10, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C15594: 93AA0038  stw r29, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82C15598: 4800000C  b 0x82c155a4
	pc = 0x82C155A4; continue 'dispatch;
	// 82C1559C: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82C155A0: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82C155A4: 93A50004  stw r29, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C155A8: 93A50000  stw r29, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C155AC: 93A50008  stw r29, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C155B0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C155B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C155B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C155BC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C155C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C155C4: 4E800421  bctrl
	ctx.lr = 0x82C155C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C155C8: 7EFCBB78  mr r28, r23
	ctx.r[28].u64 = ctx.r[23].u64;
	// 82C155CC: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82C155D0: 409AFD88  bne cr6, 0x82c15358
	if !ctx.cr[6].eq {
	pc = 0x82C15358; continue 'dispatch;
	}
	// 82C155D4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C155D8: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82C155DC: 48093E68  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C155E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C155E0 size=744
    let mut pc: u32 = 0x82C155E0;
    'dispatch: loop {
        match pc {
            0x82C155E0 => {
    //   block [0x82C155E0..0x82C158C8)
	// 82C155E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C155E4: 48093E09  bl 0x82ca93ec
	ctx.lr = 0x82C155E8;
	sub_82CA93D0(ctx, base);
	// 82C155E8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C155EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C155F0: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82C155F4: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82C155F8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82C155FC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82C15600: 7D164378  mr r22, r8
	ctx.r[22].u64 = ctx.r[8].u64;
	// 82C15604: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82C15608: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C1560C: 409A0018  bne cr6, 0x82c15624
	if !ctx.cr[6].eq {
	pc = 0x82C15624; continue 'dispatch;
	}
	// 82C15610: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C15614: 93F50000  stw r31, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C15618: 93F50004  stw r31, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C1561C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C15620: 48093E1C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82C15624: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82C15628: 4B609C31  bl 0x8221f258
	ctx.lr = 0x82C1562C;
	sub_8221F258(ctx, base);
	// 82C1562C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C15630: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15634: 419A0010  beq cr6, 0x82c15644
	if ctx.cr[6].eq {
	pc = 0x82C15644; continue 'dispatch;
	}
	// 82C15638: 4BFFE7B9  bl 0x82c13df0
	ctx.lr = 0x82C1563C;
	sub_82C13DF0(ctx, base);
	// 82C1563C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82C15640: 48000008  b 0x82c15648
	pc = 0x82C15648; continue 'dispatch;
	// 82C15644: 7FF8FB78  mr r24, r31
	ctx.r[24].u64 = ctx.r[31].u64;
	// 82C15648: 3779006C  addic. r27, r25, 0x6c
	ctx.xer.ca = (ctx.r[25].u32 > (!(108 as u32)));
	ctx.r[27].s64 = ctx.r[25].s64 + 108;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82C1564C: 3BB80048  addi r29, r24, 0x48
	ctx.r[29].s64 = ctx.r[24].s64 + 72;
	// 82C15650: 41820018  beq 0x82c15668
	if ctx.cr[0].eq {
	pc = 0x82C15668; continue 'dispatch;
	}
	// 82C15654: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15658: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C1565C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15660: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C15664: 4E800421  bctrl
	ctx.lr = 0x82C15668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15668: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1566C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15670: 419A0014  beq cr6, 0x82c15684
	if ctx.cr[6].eq {
	pc = 0x82C15684; continue 'dispatch;
	}
	// 82C15674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15678: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1567C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C15680: 4E800421  bctrl
	ctx.lr = 0x82C15684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15684: 937D0004  stw r27, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C15688: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82C1568C: 933D0000  stw r25, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C15690: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82C15694: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C15698: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82C1569C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82C156A0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82C156A4: 4BFFEEBD  bl 0x82c14560
	ctx.lr = 0x82C156A8;
	sub_82C14560(ctx, base);
	// 82C156A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C156AC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C156B0: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C156B4: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C156B8: 4BFFE8B9  bl 0x82c13f70
	ctx.lr = 0x82C156BC;
	sub_82C13F70(ctx, base);
	// 82C156BC: 387E0060  addi r3, r30, 0x60
	ctx.r[3].s64 = ctx.r[30].s64 + 96;
	// 82C156C0: 809E0288  lwz r4, 0x288(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C156C4: 80B70008  lwz r5, 8(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C156C8: 4803A6C1  bl 0x82c4fd88
	ctx.lr = 0x82C156CC;
	sub_82C4FD88(ctx, base);
	// 82C156CC: 8078001C  lwz r3, 0x1c(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C156D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C156D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C156D8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C156DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C156E0: 4E800421  bctrl
	ctx.lr = 0x82C156E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C156E4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82C156E8: 38780044  addi r3, r24, 0x44
	ctx.r[3].s64 = ctx.r[24].s64 + 68;
	// 82C156EC: 4B64FAB5  bl 0x822651a0
	ctx.lr = 0x82C156F0;
	sub_822651A0(ctx, base);
	// 82C156F0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82C156F4: 38770014  addi r3, r23, 0x14
	ctx.r[3].s64 = ctx.r[23].s64 + 20;
	// 82C156F8: 4BFFC6F9  bl 0x82c11df0
	ctx.lr = 0x82C156FC;
	sub_82C11DF0(ctx, base);
	// 82C156FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15700: 92E1006C  stw r23, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[23].u32 ) };
	// 82C15704: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82C15708: 3B2BAB10  addi r25, r11, -0x54f0
	ctx.r[25].s64 = ctx.r[11].s64 + -21744;
	// 82C1570C: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82C15710: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82C15714: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C15718: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82C1571C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82C15720: 80770004  lwz r3, 4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15724: 480086C5  bl 0x82c1dde8
	ctx.lr = 0x82C15728;
	sub_82C1DDE8(ctx, base);
	// 82C15728: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C1572C: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82C15730: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82C15734: 419A0134  beq cr6, 0x82c15868
	if ctx.cr[6].eq {
	pc = 0x82C15868; continue 'dispatch;
	}
	// 82C15738: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1573C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C15740: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C15744: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82C15748: 3BCB0040  addi r30, r11, 0x40
	ctx.r[30].s64 = ctx.r[11].s64 + 64;
	// 82C1574C: 7D5B5214  add r10, r27, r10
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[10].u64;
	// 82C15750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C15754: 812B008C  lwz r9, 0x8c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C15758: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C1575C: 7CE8482E  lwzx r7, r8, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82C15760: 83870004  lwz r28, 4(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15764: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82C15768: 4BFFCB39  bl 0x82c122a0
	ctx.lr = 0x82C1576C;
	sub_82C122A0(ctx, base);
	// 82C1576C: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C15770: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C15774: 409A000C  bne cr6, 0x82c15780
	if !ctx.cr[6].eq {
	pc = 0x82C15780; continue 'dispatch;
	}
	// 82C15778: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82C1577C: 48000010  b 0x82c1578c
	pc = 0x82C1578C; continue 'dispatch;
	// 82C15780: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15784: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C15788: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C1578C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15790: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C15794: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C15798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1579C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C157A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C157A4: 4E800421  bctrl
	ctx.lr = 0x82C157A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C157A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C157AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C157B0: 419A00AC  beq cr6, 0x82c1585c
	if ctx.cr[6].eq {
	pc = 0x82C1585C; continue 'dispatch;
	}
	// 82C157B4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C157B8: 4B609AA1  bl 0x8221f258
	ctx.lr = 0x82C157BC;
	sub_8221F258(ctx, base);
	// 82C157BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C157C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C157C4: 419A0020  beq cr6, 0x82c157e4
	if ctx.cr[6].eq {
	pc = 0x82C157E4; continue 'dispatch;
	}
	// 82C157C8: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C157CC: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C157D0: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C157D4: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82C157D8: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82C157DC: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82C157E0: 48000008  b 0x82c157e8
	pc = 0x82C157E8; continue 'dispatch;
	// 82C157E4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C157E8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C157EC: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82C157F0: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C157F4: 39380024  addi r9, r24, 0x24
	ctx.r[9].s64 = ctx.r[24].s64 + 36;
	// 82C157F8: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82C157FC: 938B0014  stw r28, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82C15800: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C15804: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C15808: 409A0054  bne cr6, 0x82c1585c
	if !ctx.cr[6].eq {
	pc = 0x82C1585C; continue 'dispatch;
	}
	// 82C1580C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15810: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15814: 409A0048  bne cr6, 0x82c1585c
	if !ctx.cr[6].eq {
	pc = 0x82C1585C; continue 'dispatch;
	}
	// 82C15818: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1581C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15820: 409A003C  bne cr6, 0x82c1585c
	if !ctx.cr[6].eq {
	pc = 0x82C1585C; continue 'dispatch;
	}
	// 82C15824: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C15828: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1582C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15830: 409A0014  bne cr6, 0x82c15844
	if !ctx.cr[6].eq {
	pc = 0x82C15844; continue 'dispatch;
	}
	// 82C15834: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C15838: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C1583C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C15840: 48000018  b 0x82c15858
	pc = 0x82C15858; continue 'dispatch;
	// 82C15844: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15848: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C1584C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C15850: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15854: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C15858: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1585C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82C15860: 7F1BD040  cmplw cr6, r27, r26
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C15864: 4198FED4  blt cr6, 0x82c15738
	if ctx.cr[6].lt {
	pc = 0x82C15738; continue 'dispatch;
	}
	// 82C15868: 37D8002C  addic. r30, r24, 0x2c
	ctx.xer.ca = (ctx.r[24].u32 > (!(44 as u32)));
	ctx.r[30].s64 = ctx.r[24].s64 + 44;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C1586C: 93F50000  stw r31, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C15870: 93F50004  stw r31, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C15874: 41820018  beq 0x82c1588c
	if ctx.cr[0].eq {
	pc = 0x82C1588C; continue 'dispatch;
	}
	// 82C15878: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1587C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C15880: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15884: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C15888: 4E800421  bctrl
	ctx.lr = 0x82C1588C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1588C: 80750004  lwz r3, 4(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15890: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15894: 419A0014  beq cr6, 0x82c158a8
	if ctx.cr[6].eq {
	pc = 0x82C158A8; continue 'dispatch;
	}
	// 82C15898: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1589C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C158A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C158A4: 4E800421  bctrl
	ctx.lr = 0x82C158A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C158A8: 93D50004  stw r30, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C158AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C158B0: 93150000  stw r24, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82C158B4: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82C158B8: 4BFF0849  bl 0x82c06100
	ctx.lr = 0x82C158BC;
	sub_82C06100(ctx, base);
	// 82C158BC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82C158C0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C158C4: 48093B78  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C158C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C158C8 size=80
    let mut pc: u32 = 0x82C158C8;
    'dispatch: loop {
        match pc {
            0x82C158C8 => {
    //   block [0x82C158C8..0x82C15918)
	// 82C158C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C158CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C158D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C158D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C158D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C158DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C158E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C158E4: 4BFFEE6D  bl 0x82c14750
	ctx.lr = 0x82C158E8;
	sub_82C14750(ctx, base);
	// 82C158E8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C158EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C158F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C158F4: 419A000C  beq cr6, 0x82c15900
	if ctx.cr[6].eq {
	pc = 0x82C15900; continue 'dispatch;
	}
	// 82C158F8: 4BC2FEB9  bl 0x828457b0
	ctx.lr = 0x82C158FC;
	sub_828457B0(ctx, base);
	// 82C158FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15900: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1590C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15918 size=56
    let mut pc: u32 = 0x82C15918;
    'dispatch: loop {
        match pc {
            0x82C15918 => {
    //   block [0x82C15918..0x82C15950)
	// 82C15918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1591C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15928: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1592C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15930: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15934: 4BFFFCAD  bl 0x82c155e0
	ctx.lr = 0x82C15938;
	sub_82C155E0(ctx, base);
	// 82C15938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1593C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C15940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1594C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15950 size=92
    let mut pc: u32 = 0x82C15950;
    'dispatch: loop {
        match pc {
            0x82C15950 => {
    //   block [0x82C15950..0x82C159AC)
	// 82C15950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1595C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15964: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15968: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C1596C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C15970: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C15974: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C15978: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1597C: 4E800421  bctrl
	ctx.lr = 0x82C15980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15984: 419A0010  beq cr6, 0x82c15994
	if ctx.cr[6].eq {
	pc = 0x82C15994; continue 'dispatch;
	}
	// 82C15988: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C1598C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C15990: 4BFFF2D1  bl 0x82c14c60
	ctx.lr = 0x82C15994;
	sub_82C14C60(ctx, base);
	// 82C15994: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C159A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C159A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C159A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C159B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C159B0 size=92
    let mut pc: u32 = 0x82C159B0;
    'dispatch: loop {
        match pc {
            0x82C159B0 => {
    //   block [0x82C159B0..0x82C15A0C)
	// 82C159B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C159B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C159B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C159BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C159C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C159C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C159C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C159CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C159D0: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C159D4: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C159D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C159DC: 4E800421  bctrl
	ctx.lr = 0x82C159E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C159E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C159E4: 419A0010  beq cr6, 0x82c159f4
	if ctx.cr[6].eq {
	pc = 0x82C159F4; continue 'dispatch;
	}
	// 82C159E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C159EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C159F0: 4BFFF301  bl 0x82c14cf0
	ctx.lr = 0x82C159F4;
	sub_82C14CF0(ctx, base);
	// 82C159F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C159F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C159FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15A00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C15A10 size=8
    let mut pc: u32 = 0x82C15A10;
    'dispatch: loop {
        match pc {
            0x82C15A10 => {
    //   block [0x82C15A10..0x82C15A18)
	// 82C15A10: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C15A14: 4BFFEF2C  b 0x82c14940
	sub_82C14940(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15A18 size=96
    let mut pc: u32 = 0x82C15A18;
    'dispatch: loop {
        match pc {
            0x82C15A18 => {
    //   block [0x82C15A18..0x82C15A78)
	// 82C15A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15A20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C15A24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15A28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15A2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15A30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15A34: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C15A38: 394BAC40  addi r10, r11, -0x53c0
	ctx.r[10].s64 = ctx.r[11].s64 + -21440;
	// 82C15A3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C15A40: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C15A44: 4803815D  bl 0x82c4dba0
	ctx.lr = 0x82C15A48;
	sub_82C4DBA0(ctx, base);
	// 82C15A48: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C15A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15A50: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C15A54: 419A000C  beq cr6, 0x82c15a60
	if ctx.cr[6].eq {
	pc = 0x82C15A60; continue 'dispatch;
	}
	// 82C15A58: 4BC2FD59  bl 0x828457b0
	ctx.lr = 0x82C15A5C;
	sub_828457B0(ctx, base);
	// 82C15A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15A60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15A6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C15A78 size=152
    let mut pc: u32 = 0x82C15A78;
    'dispatch: loop {
        match pc {
            0x82C15A78 => {
    //   block [0x82C15A78..0x82C15B10)
	// 82C15A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15A7C: 48093991  bl 0x82ca940c
	ctx.lr = 0x82C15A80;
	sub_82CA93D0(ctx, base);
	// 82C15A80: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15B10 size=332
    let mut pc: u32 = 0x82C15B10;
    'dispatch: loop {
        match pc {
            0x82C15B10 => {
    //   block [0x82C15B10..0x82C15C5C)
	// 82C15B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15B14: 480938ED  bl 0x82ca9400
	ctx.lr = 0x82C15B18;
	sub_82CA93D0(ctx, base);
	// 82C15B18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15B1C: 3BA40028  addi r29, r4, 0x28
	ctx.r[29].s64 = ctx.r[4].s64 + 40;
	// 82C15B20: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C15B24: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C15B28: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C15B2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C15B30: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82C15B34: 480376A5  bl 0x82c4d1d8
	ctx.lr = 0x82C15B38;
	sub_82C4D1D8(ctx, base);
	// 82C15B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15B3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C15B40: 409A00DC  bne cr6, 0x82c15c1c
	if !ctx.cr[6].eq {
	pc = 0x82C15C1C; continue 'dispatch;
	}
	// 82C15B44: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82C15B48: 4B609711  bl 0x8221f258
	ctx.lr = 0x82C15B4C;
	sub_8221F258(ctx, base);
	// 82C15B4C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C15B50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15B54: 419A0010  beq cr6, 0x82c15b64
	if ctx.cr[6].eq {
	pc = 0x82C15B64; continue 'dispatch;
	}
	// 82C15B58: 4BFFD859  bl 0x82c133b0
	ctx.lr = 0x82C15B5C;
	sub_82C133B0(ctx, base);
	// 82C15B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15B60: 48000008  b 0x82c15b68
	pc = 0x82C15B68; continue 'dispatch;
	// 82C15B64: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82C15B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C15B6C: 3B9F001C  addi r28, r31, 0x1c
	ctx.r[28].s64 = ctx.r[31].s64 + 28;
	// 82C15B70: 4BFF0431  bl 0x82c05fa0
	ctx.lr = 0x82C15B74;
	sub_82C05FA0(ctx, base);
	// 82C15B74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C15B78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C15B7C: 4BFFC14D  bl 0x82c11cc8
	ctx.lr = 0x82C15B80;
	sub_82C11CC8(ctx, base);
	// 82C15B80: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C15B84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15B88: 419A0014  beq cr6, 0x82c15b9c
	if ctx.cr[6].eq {
	pc = 0x82C15B9C; continue 'dispatch;
	}
	// 82C15B8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15B90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15B94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C15B98: 4E800421  bctrl
	ctx.lr = 0x82C15B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15B9C: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82C15BA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C15BA4: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82C15BA8: 4BFEFB89  bl 0x82c05730
	ctx.lr = 0x82C15BAC;
	sub_82C05730(ctx, base);
	// 82C15BAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15BB0: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82C15BB4: 388BAD9C  addi r4, r11, -0x5264
	ctx.r[4].s64 = ctx.r[11].s64 + -21092;
	// 82C15BB8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C15BBC: 3B6A4A08  addi r27, r10, 0x4a08
	ctx.r[27].s64 = ctx.r[10].s64 + 18952;
	// 82C15BC0: 4BFEFC39  bl 0x82c057f8
	ctx.lr = 0x82C15BC4;
	sub_82C057F8(ctx, base);
	// 82C15BC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C15BC8: 4BFEFBF9  bl 0x82c057c0
	ctx.lr = 0x82C15BCC;
	sub_82C057C0(ctx, base);
	// 82C15BCC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C15BD0: 4BFEFC29  bl 0x82c057f8
	ctx.lr = 0x82C15BD4;
	sub_82C057F8(ctx, base);
	// 82C15BD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C15BD8: 839C0000  lwz r28, 0(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15BDC: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15BE0: 4BC366A1  bl 0x8284c280
	ctx.lr = 0x82C15BE4;
	sub_8284C280(ctx, base);
	// 82C15BE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C15BE8: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15BEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C15BF0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C15BF4: 4E800421  bctrl
	ctx.lr = 0x82C15BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15BF8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C15BFC: 4BFEFB75  bl 0x82c05770
	ctx.lr = 0x82C15C00;
	sub_82C05770(ctx, base);
	// 82C15C00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C15C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15C08: 4B64F599  bl 0x822651a0
	ctx.lr = 0x82C15C0C;
	sub_822651A0(ctx, base);
	// 82C15C0C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C15C10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C15C14: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82C15C18: 480376A9  bl 0x82c4d2c0
	ctx.lr = 0x82C15C1C;
	sub_82C4D2C0(ctx, base);
	// 82C15C1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15C20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C15C24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C15C28: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C15C2C: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C15C30: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C15C34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C15C38: 907A0004  stw r3, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C15C3C: 419A0014  beq cr6, 0x82c15c50
	if ctx.cr[6].eq {
	pc = 0x82C15C50; continue 'dispatch;
	}
	// 82C15C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15C44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15C48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C15C4C: 4E800421  bctrl
	ctx.lr = 0x82C15C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C15C50: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C15C54: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C15C58: 480937F8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C15C60 size=48
    let mut pc: u32 = 0x82C15C60;
    'dispatch: loop {
        match pc {
            0x82C15C60 => {
    //   block [0x82C15C60..0x82C15C90)
	// 82C15C60: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15C64: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15C68: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C15C6C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C15C70: 409A0030  bne cr6, 0x82c15ca0
	if !ctx.cr[6].eq {
		sub_82C15C90(ctx, base);
		return;
	}
	// 82C15C74: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15C78: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C15C7C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C15C80: 40980010  bge cr6, 0x82c15c90
	if !ctx.cr[6].lt {
		sub_82C15C90(ctx, base);
		return;
	}
	// 82C15C84: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C15C88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15C8C: 48000008  b 0x82c15c94
	sub_82C15C90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C15C90 size=72
    let mut pc: u32 = 0x82C15C90;
    'dispatch: loop {
        match pc {
            0x82C15C90 => {
    //   block [0x82C15C90..0x82C15CD8)
	// 82C15C90: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C15C94: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C15C98: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15C9C: 419AFFDC  beq cr6, 0x82c15c78
	if ctx.cr[6].eq {
		sub_82C15C60(ctx, base);
		return;
	}
	// 82C15CA0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15CA4: 9141FFF4  stw r10, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 82C15CA8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C15CAC: 9081FFF0  stw r4, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u32 ) };
	// 82C15CB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15CB4: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C15CB8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C15CBC: 409A0030  bne cr6, 0x82c15cec
	if !ctx.cr[6].eq {
		sub_82C15CD8(ctx, base);
		return;
	}
	// 82C15CC0: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15CC4: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C15CC8: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C15CCC: 4098000C  bge cr6, 0x82c15cd8
	if !ctx.cr[6].lt {
		sub_82C15CD8(ctx, base);
		return;
	}
	// 82C15CD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C15CD4: 4800000C  b 0x82c15ce0
	sub_82C15CD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C15CD8 size=48
    let mut pc: u32 = 0x82C15CD8;
    'dispatch: loop {
        match pc {
            0x82C15CD8 => {
    //   block [0x82C15CD8..0x82C15D08)
	// 82C15CD8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C15CDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C15CE0: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C15CE4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C15CE8: 419AFFDC  beq cr6, 0x82c15cc4
	if ctx.cr[6].eq {
		sub_82C15C90(ctx, base);
		return;
	}
	// 82C15CEC: E961FFF0  ld r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15CF0: 9141FFF4  stw r10, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 82C15CF4: 9081FFF0  stw r4, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u32 ) };
	// 82C15CF8: E941FFF0  ld r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15CFC: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82C15D00: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82C15D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15D08 size=84
    let mut pc: u32 = 0x82C15D08;
    'dispatch: loop {
        match pc {
            0x82C15D08 => {
    //   block [0x82C15D08..0x82C15D5C)
	// 82C15D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15D18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15D1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15D20: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15D24: 4BFFEE0D  bl 0x82c14b30
	ctx.lr = 0x82C15D28;
	sub_82C14B30(ctx, base);
	// 82C15D28: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15D2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C15D30: 91290004  stw r9, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C15D34: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15D38: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C15D3C: 91080000  stw r8, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C15D40: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C15D44: 90E70008  stw r7, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C15D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C15D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15D54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15D60 size=80
    let mut pc: u32 = 0x82C15D60;
    'dispatch: loop {
        match pc {
            0x82C15D60 => {
    //   block [0x82C15D60..0x82C15DB0)
	// 82C15D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C15D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15D78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C15D7C: 4BFFF085  bl 0x82c14e00
	ctx.lr = 0x82C15D80;
	sub_82C14E00(ctx, base);
	// 82C15D80: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C15D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15D88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C15D8C: 419A000C  beq cr6, 0x82c15d98
	if ctx.cr[6].eq {
	pc = 0x82C15D98; continue 'dispatch;
	}
	// 82C15D90: 4BC2FA21  bl 0x828457b0
	ctx.lr = 0x82C15D94;
	sub_828457B0(ctx, base);
	// 82C15D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15D98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15DA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15DA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C15DB0 size=252
    let mut pc: u32 = 0x82C15DB0;
    'dispatch: loop {
        match pc {
            0x82C15DB0 => {
    //   block [0x82C15DB0..0x82C15EAC)
	// 82C15DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15DB4: 48093659  bl 0x82ca940c
	ctx.lr = 0x82C15DB8;
	sub_82CA93D0(ctx, base);
	// 82C15DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15DBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15DC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15DC4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C15DC8: 392BAD68  addi r9, r11, -0x5298
	ctx.r[9].s64 = ctx.r[11].s64 + -21144;
	// 82C15DCC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C15DD0: 390A11BC  addi r8, r10, 0x11bc
	ctx.r[8].s64 = ctx.r[10].s64 + 4540;
	// 82C15DD4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C15DD8: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C15DDC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C15DE0: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C15DE4: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 82C15DE8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C15DEC: 38C7AC68  addi r6, r7, -0x5398
	ctx.r[6].s64 = ctx.r[7].s64 + -21400;
	// 82C15DF0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C15DF4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C15DF8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82C15DFC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C15E00: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C15E04: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82C15E08: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82C15E0C: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82C15E10: 48037C99  bl 0x82c4daa8
	ctx.lr = 0x82C15E14;
	sub_82C4DAA8(ctx, base);
	// 82C15E14: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C15E18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C15E1C: 3865AC68  addi r3, r5, -0x5398
	ctx.r[3].s64 = ctx.r[5].s64 + -21400;
	// 82C15E20: 3BBF0060  addi r29, r31, 0x60
	ctx.r[29].s64 = ctx.r[31].s64 + 96;
	// 82C15E24: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82C15E28: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82C15E2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C15E30: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C15E34: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82C15E38: 909F0040  stw r4, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 82C15E3C: 4B5E19C5  bl 0x821f7800
	ctx.lr = 0x82C15E40;
	sub_821F7800(ctx, base);
	// 82C15E40: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82C15E44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C15E48: 388B0D8C  addi r4, r11, 0xd8c
	ctx.r[4].s64 = ctx.r[11].s64 + 3468;
	// 82C15E4C: 4B65F5ED  bl 0x82275438
	ctx.lr = 0x82C15E50;
	sub_82275438(ctx, base);
	// 82C15E50: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C15E54: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82C15E58: 93FF0008  stw r31, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C15E5C: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82C15E60: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82C15E64: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C15E68: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C15E6C: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82C15E70: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C15E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15E78: 13E04C07  vcmpneb. (lvlx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C15E7C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C15E80: 13C04407  vcmpneb. (lvlx128) v30, v0, v8
	tmp.u32 = ctx.r[8].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C15E84: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C15E88: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15EB0 size=80
    let mut pc: u32 = 0x82C15EB0;
    'dispatch: loop {
        match pc {
            0x82C15EB0 => {
    //   block [0x82C15EB0..0x82C15F00)
	// 82C15EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C15EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15EC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C15ECC: 4BFFF03D  bl 0x82c14f08
	ctx.lr = 0x82C15ED0;
	sub_82C14F08(ctx, base);
	// 82C15ED0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C15ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15ED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C15EDC: 419A000C  beq cr6, 0x82c15ee8
	if ctx.cr[6].eq {
	pc = 0x82C15EE8; continue 'dispatch;
	}
	// 82C15EE0: 4BC2F8D1  bl 0x828457b0
	ctx.lr = 0x82C15EE4;
	sub_828457B0(ctx, base);
	// 82C15EE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15F00 size=164
    let mut pc: u32 = 0x82C15F00;
    'dispatch: loop {
        match pc {
            0x82C15F00 => {
    //   block [0x82C15F00..0x82C15FA4)
	// 82C15F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15F04: 48093509  bl 0x82ca940c
	ctx.lr = 0x82C15F08;
	sub_82CA93D0(ctx, base);
	// 82C15F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15F0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15F10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C15F14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C15F18: 392BAD84  addi r9, r11, -0x527c
	ctx.r[9].s64 = ctx.r[11].s64 + -21116;
	// 82C15F1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C15F20: 390AA3EC  addi r8, r10, -0x5c14
	ctx.r[8].s64 = ctx.r[10].s64 + -23572;
	// 82C15F24: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C15F28: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C15F2C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C15F30: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C15F34: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 82C15F38: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C15F3C: 38C7AC68  addi r6, r7, -0x5398
	ctx.r[6].s64 = ctx.r[7].s64 + -21400;
	// 82C15F40: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C15F44: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C15F48: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82C15F4C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C15F50: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C15F54: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82C15F58: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82C15F5C: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82C15F60: 48037B49  bl 0x82c4daa8
	ctx.lr = 0x82C15F64;
	sub_82C4DAA8(ctx, base);
	// 82C15F64: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C15F68: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	// 82C15F6C: 3885AC68  addi r4, r5, -0x5398
	ctx.r[4].s64 = ctx.r[5].s64 + -21400;
	// 82C15F70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C15F74: 909F0028  stw r4, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82C15F78: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82C15F7C: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C15F80: 4B5E1881  bl 0x821f7800
	ctx.lr = 0x82C15F84;
	sub_821F7800(ctx, base);
	// 82C15F84: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82C15F88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C15F8C: 388B0D8C  addi r4, r11, 0xd8c
	ctx.r[4].s64 = ctx.r[11].s64 + 3468;
	// 82C15F90: 4B65F4A9  bl 0x82275438
	ctx.lr = 0x82C15F94;
	sub_82275438(ctx, base);
	// 82C15F94: 93FF0008  stw r31, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C15F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15F9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15FA0: 480934BC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15FA8 size=80
    let mut pc: u32 = 0x82C15FA8;
    'dispatch: loop {
        match pc {
            0x82C15FA8 => {
    //   block [0x82C15FA8..0x82C15FF8)
	// 82C15FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C15FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C15FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C15FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C15FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C15FC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C15FC4: 4BFFF0E5  bl 0x82c150a8
	ctx.lr = 0x82C15FC8;
	sub_82C150A8(ctx, base);
	// 82C15FC8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C15FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15FD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C15FD4: 419A000C  beq cr6, 0x82c15fe0
	if ctx.cr[6].eq {
	pc = 0x82C15FE0; continue 'dispatch;
	}
	// 82C15FD8: 4BC2F7D9  bl 0x828457b0
	ctx.lr = 0x82C15FDC;
	sub_828457B0(ctx, base);
	// 82C15FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C15FE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C15FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C15FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C15FEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C15FF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C15FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C15FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C15FF8 size=184
    let mut pc: u32 = 0x82C15FF8;
    'dispatch: loop {
        match pc {
            0x82C15FF8 => {
    //   block [0x82C15FF8..0x82C160B0)
	// 82C15FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C15FFC: 48093409  bl 0x82ca9404
	ctx.lr = 0x82C16000;
	sub_82CA93D0(ctx, base);
	// 82C16000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16004: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C16008: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82C1600C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C16010: 4B609249  bl 0x8221f258
	ctx.lr = 0x82C16014;
	sub_8221F258(ctx, base);
	// 82C16014: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C16018: 419A0014  beq cr6, 0x82c1602c
	if ctx.cr[6].eq {
	pc = 0x82C1602C; continue 'dispatch;
	}
	// 82C1601C: 4BFFFD95  bl 0x82c15db0
	ctx.lr = 0x82C16020;
	sub_82C15DB0(ctx, base);
	// 82C16020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16024: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C16028: 409A001C  bne cr6, 0x82c16044
	if !ctx.cr[6].eq {
	pc = 0x82C16044; continue 'dispatch;
	}
	// 82C1602C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C16030: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C16034: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16038: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1603C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C16040: 48093414  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C16044: 37BC0004  addic. r29, r28, 4
	ctx.xer.ca = (ctx.r[28].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C16048: 3BDF0034  addi r30, r31, 0x34
	ctx.r[30].s64 = ctx.r[31].s64 + 52;
	// 82C1604C: 41820018  beq 0x82c16064
	if ctx.cr[0].eq {
	pc = 0x82C16064; continue 'dispatch;
	}
	// 82C16050: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16054: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C16058: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1605C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C16060: 4E800421  bctrl
	ctx.lr = 0x82C16064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16064: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16068: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1606C: 419A0014  beq cr6, 0x82c16080
	if ctx.cr[6].eq {
	pc = 0x82C16080; continue 'dispatch;
	}
	// 82C16070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16074: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16078: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1607C: 4E800421  bctrl
	ctx.lr = 0x82C16080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16080: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C16084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C16088: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1608C: 387C001C  addi r3, r28, 0x1c
	ctx.r[3].s64 = ctx.r[28].s64 + 28;
	// 82C16090: 4BFFEA29  bl 0x82c14ab8
	ctx.lr = 0x82C16094;
	sub_82C14AB8(ctx, base);
	// 82C16094: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82C16098: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1609C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C160A0: 4B5BF141  bl 0x821d51e0
	ctx.lr = 0x82C160A4;
	sub_821D51E0(ctx, base);
	// 82C160A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C160A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C160AC: 480933A8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C160B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C160B0 size=312
    let mut pc: u32 = 0x82C160B0;
    'dispatch: loop {
        match pc {
            0x82C160B0 => {
    //   block [0x82C160B0..0x82C161E8)
	// 82C160B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C160B4: 48093349  bl 0x82ca93fc
	ctx.lr = 0x82C160B8;
	sub_82CA93D0(ctx, base);
	// 82C160B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C160BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C160C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C160C4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C160C8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C160CC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82C160D0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C160D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C160D8: 419A00F8  beq cr6, 0x82c161d0
	if ctx.cr[6].eq {
	pc = 0x82C161D0; continue 'dispatch;
	}
	// 82C160DC: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C160E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C160E4: 419A00EC  beq cr6, 0x82c161d0
	if ctx.cr[6].eq {
	pc = 0x82C161D0; continue 'dispatch;
	}
	// 82C160E8: 833D0010  lwz r25, 0x10(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C160EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C160F0: 386B0034  addi r3, r11, 0x34
	ctx.r[3].s64 = ctx.r[11].s64 + 52;
	// 82C160F4: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 82C160F8: 4BFFD259  bl 0x82c13350
	ctx.lr = 0x82C160FC;
	sub_82C13350(ctx, base);
	// 82C160FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C16100: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C16104: 419A00CC  beq cr6, 0x82c161d0
	if ctx.cr[6].eq {
	pc = 0x82C161D0; continue 'dispatch;
	}
	// 82C16108: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82C1610C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C16110: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C16114: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C16118: 4BFFE5B1  bl 0x82c146c8
	ctx.lr = 0x82C1611C;
	sub_82C146C8(ctx, base);
	// 82C1611C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16120: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C16124: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C16128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1612C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C16130: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C16134: 4E800421  bctrl
	ctx.lr = 0x82C16138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16138: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C1613C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C16140: 409A0024  bne cr6, 0x82c16164
	if !ctx.cr[6].eq {
	pc = 0x82C16164; continue 'dispatch;
	}
	// 82C16144: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C16148: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C1614C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16150: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C16154: 4BFFE5FD  bl 0x82c14750
	ctx.lr = 0x82C16158;
	sub_82C14750(ctx, base);
	// 82C16158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1615C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C16160: 480932EC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C16164: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C16168: 4BFEF5C9  bl 0x82c05730
	ctx.lr = 0x82C1616C;
	sub_82C05730(ctx, base);
	// 82C1616C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16174: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16178: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1617C: 4E800421  bctrl
	ctx.lr = 0x82C16180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16180: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C16184: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C16188: 4BFEF639  bl 0x82c057c0
	ctx.lr = 0x82C1618C;
	sub_82C057C0(ctx, base);
	// 82C1618C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C16190: 4BC360F1  bl 0x8284c280
	ctx.lr = 0x82C16194;
	sub_8284C280(ctx, base);
	// 82C16194: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82C16198: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C1619C: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C161A0: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82C161A4: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C161A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C161AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C161B0: 4BFFF431  bl 0x82c155e0
	ctx.lr = 0x82C161B4;
	sub_82C155E0(ctx, base);
	// 82C161B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C161B8: 4BFEF5B9  bl 0x82c05770
	ctx.lr = 0x82C161BC;
	sub_82C05770(ctx, base);
	// 82C161BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C161C0: 4BFFE591  bl 0x82c14750
	ctx.lr = 0x82C161C4;
	sub_82C14750(ctx, base);
	// 82C161C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C161C8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C161CC: 48093280  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C161D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C161D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C161D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C161DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C161E0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C161E4: 48093268  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C161E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C161E8 size=164
    let mut pc: u32 = 0x82C161E8;
    'dispatch: loop {
        match pc {
            0x82C161E8 => {
    //   block [0x82C161E8..0x82C1628C)
	// 82C161E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C161EC: 48093221  bl 0x82ca940c
	ctx.lr = 0x82C161F0;
	sub_82CA93D0(ctx, base);
	// 82C161F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C161F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C161F8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82C161FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16200: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C16204: 388100A4  addi r4, r1, 0xa4
	ctx.r[4].s64 = ctx.r[1].s64 + 164;
	// 82C16208: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1620C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16210: 3BAB005C  addi r29, r11, 0x5c
	ctx.r[29].s64 = ctx.r[11].s64 + 92;
	// 82C16214: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C16218: 4BFFC089  bl 0x82c122a0
	ctx.lr = 0x82C1621C;
	sub_82C122A0(ctx, base);
	// 82C1621C: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16220: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C16224: 409A000C  bne cr6, 0x82c16230
	if !ctx.cr[6].eq {
	pc = 0x82C16230; continue 'dispatch;
	}
	// 82C16228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C1622C: 48000010  b 0x82c1623c
	pc = 0x82C1623C; continue 'dispatch;
	// 82C16230: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16234: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C16238: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C1623C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C16240: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 82C16244: 388B0028  addi r4, r11, 0x28
	ctx.r[4].s64 = ctx.r[11].s64 + 40;
	// 82C16248: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82C1624C: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C16250: 48036F89  bl 0x82c4d1d8
	ctx.lr = 0x82C16254;
	sub_82C4D1D8(ctx, base);
	// 82C16254: 3963001C  addi r11, r3, 0x1c
	ctx.r[11].s64 = ctx.r[3].s64 + 28;
	// 82C16258: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1625C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C16260: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C16268: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C1626C: 419A0014  beq cr6, 0x82c16280
	if ctx.cr[6].eq {
	pc = 0x82C16280; continue 'dispatch;
	}
	// 82C16270: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16274: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16278: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1627C: 4E800421  bctrl
	ctx.lr = 0x82C16280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C16284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C16288: 480931D4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C16290 size=452
    let mut pc: u32 = 0x82C16290;
    'dispatch: loop {
        match pc {
            0x82C16290 => {
    //   block [0x82C16290..0x82C16454)
	// 82C16290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16294: 48093171  bl 0x82ca9404
	ctx.lr = 0x82C16298;
	sub_82CA93D0(ctx, base);
	// 82C16298: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1629C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C162A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C162A4: 997B0311  stb r11, 0x311(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(785 as u32), ctx.r[11].u8 ) };
	// 82C162A8: 4BFFE731  bl 0x82c149d8
	ctx.lr = 0x82C162AC;
	sub_82C149D8(ctx, base);
	// 82C162AC: 3BBB0034  addi r29, r27, 0x34
	ctx.r[29].s64 = ctx.r[27].s64 + 52;
	// 82C162B0: 83DB0034  lwz r30, 0x34(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C162B4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C162B8: 419A0084  beq cr6, 0x82c1633c
	if ctx.cr[6].eq {
	pc = 0x82C1633C; continue 'dispatch;
	}
	// 82C162BC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C162C0: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C162C4: 3BFE004C  addi r31, r30, 0x4c
	ctx.r[31].s64 = ctx.r[30].s64 + 76;
	// 82C162C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C162CC: 419A0040  beq cr6, 0x82c1630c
	if ctx.cr[6].eq {
	pc = 0x82C1630C; continue 'dispatch;
	}
	// 82C162D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C162D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C162D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C162DC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C162E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C162E4: 4E800421  bctrl
	ctx.lr = 0x82C162E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C162E8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C162EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C162F0: 419A0014  beq cr6, 0x82c16304
	if ctx.cr[6].eq {
	pc = 0x82C16304; continue 'dispatch;
	}
	// 82C162F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C162F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C162FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C16300: 4E800421  bctrl
	ctx.lr = 0x82C16304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16304: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C16308: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1630C: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C16310: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C16314: 409A0028  bne cr6, 0x82c1633c
	if !ctx.cr[6].eq {
	pc = 0x82C1633C; continue 'dispatch;
	}
	// 82C16318: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1631C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16320: 419A0010  beq cr6, 0x82c16330
	if ctx.cr[6].eq {
	pc = 0x82C16330; continue 'dispatch;
	}
	// 82C16324: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C16328: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C1632C: 409A0010  bne cr6, 0x82c1633c
	if !ctx.cr[6].eq {
	pc = 0x82C1633C; continue 'dispatch;
	}
	// 82C16330: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82C16334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16338: 409AFF88  bne cr6, 0x82c162c0
	if !ctx.cr[6].eq {
	pc = 0x82C162C0; continue 'dispatch;
	}
	// 82C1633C: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16340: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82C16344: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C16348: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1634C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C16350: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16358: 419A000C  beq cr6, 0x82c16364
	if ctx.cr[6].eq {
	pc = 0x82C16364; continue 'dispatch;
	}
	// 82C1635C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C16360: 419A0008  beq cr6, 0x82c16368
	if ctx.cr[6].eq {
	pc = 0x82C16368; continue 'dispatch;
	}
	// 82C16364: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16368: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1636C: 419A00E0  beq cr6, 0x82c1644c
	if ctx.cr[6].eq {
	pc = 0x82C1644C; continue 'dispatch;
	}
	// 82C16370: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16374: 409A0008  bne cr6, 0x82c1637c
	if !ctx.cr[6].eq {
	pc = 0x82C1637C; continue 'dispatch;
	}
	// 82C16378: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1637C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16380: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16384: 409A0008  bne cr6, 0x82c1638c
	if !ctx.cr[6].eq {
	pc = 0x82C1638C; continue 'dispatch;
	}
	// 82C16388: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1638C: 83AA0010  lwz r29, 0x10(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C16390: 3BDD0014  addi r30, r29, 0x14
	ctx.r[30].s64 = ctx.r[29].s64 + 20;
	// 82C16394: 83FD0014  lwz r31, 0x14(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C16398: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1639C: 419A006C  beq cr6, 0x82c16408
	if ctx.cr[6].eq {
	pc = 0x82C16408; continue 'dispatch;
	}
	// 82C163A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C163A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C163A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C163AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C163B0: 4E800421  bctrl
	ctx.lr = 0x82C163B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C163B4: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C163B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C163BC: 409A0020  bne cr6, 0x82c163dc
	if !ctx.cr[6].eq {
	pc = 0x82C163DC; continue 'dispatch;
	}
	// 82C163C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C163C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C163C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C163CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C163D0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C163D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C163D8: 4E800421  bctrl
	ctx.lr = 0x82C163DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C163DC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C163E0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C163E4: 409A0024  bne cr6, 0x82c16408
	if !ctx.cr[6].eq {
	pc = 0x82C16408; continue 'dispatch;
	}
	// 82C163E8: 83FF0038  lwz r31, 0x38(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C163EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C163F0: 419A0018  beq cr6, 0x82c16408
	if ctx.cr[6].eq {
	pc = 0x82C16408; continue 'dispatch;
	}
	// 82C163F4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C163F8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C163FC: 409A000C  bne cr6, 0x82c16408
	if !ctx.cr[6].eq {
	pc = 0x82C16408; continue 'dispatch;
	}
	// 82C16400: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C16404: 409AFF9C  bne cr6, 0x82c163a0
	if !ctx.cr[6].eq {
	pc = 0x82C163A0; continue 'dispatch;
	}
	// 82C16408: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1640C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C16410: 419A0028  beq cr6, 0x82c16438
	if ctx.cr[6].eq {
	pc = 0x82C16438; continue 'dispatch;
	}
	// 82C16414: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C16418: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1641C: 4BFFEEF5  bl 0x82c15310
	ctx.lr = 0x82C16420;
	sub_82C15310(ctx, base);
	// 82C16420: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C16424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C16428: 4BFFDFD9  bl 0x82c14400
	ctx.lr = 0x82C1642C;
	sub_82C14400(ctx, base);
	// 82C1642C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16430: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C16434: 409AFFE0  bne cr6, 0x82c16414
	if !ctx.cr[6].eq {
	pc = 0x82C16414; continue 'dispatch;
	}
	// 82C16438: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1643C: 4B94041D  bl 0x82556858
	ctx.lr = 0x82C16440;
	sub_82556858(ctx, base);
	// 82C16440: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C16444: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16448: 4BFFFF08  b 0x82c16350
	pc = 0x82C16350; continue 'dispatch;
	// 82C1644C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C16450: 48093004  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C16458 size=696
    let mut pc: u32 = 0x82C16458;
    'dispatch: loop {
        match pc {
            0x82C16458 => {
    //   block [0x82C16458..0x82C16710)
	// 82C16458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1645C: 48092FA5  bl 0x82ca9400
	ctx.lr = 0x82C16460;
	sub_82CA93D0(ctx, base);
	// 82C16460: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16464: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C16468: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C1646C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C16470: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C16474: 419A0290  beq cr6, 0x82c16704
	if ctx.cr[6].eq {
	pc = 0x82C16704; continue 'dispatch;
	}
	// 82C16478: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82C1647C: 419A0288  beq cr6, 0x82c16704
	if ctx.cr[6].eq {
	pc = 0x82C16704; continue 'dispatch;
	}
	// 82C16480: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C16484: 393E0034  addi r9, r30, 0x34
	ctx.r[9].s64 = ctx.r[30].s64 + 52;
	// 82C16488: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C1648C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16490: 419A0048  beq cr6, 0x82c164d8
	if ctx.cr[6].eq {
	pc = 0x82C164D8; continue 'dispatch;
	}
	// 82C16494: 816A0068  lwz r11, 0x68(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C16498: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1649C: 409A001C  bne cr6, 0x82c164b8
	if !ctx.cr[6].eq {
	pc = 0x82C164B8; continue 'dispatch;
	}
	// 82C164A0: 816A0060  lwz r11, 0x60(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C164A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C164A8: 419A0014  beq cr6, 0x82c164bc
	if ctx.cr[6].eq {
	pc = 0x82C164BC; continue 'dispatch;
	}
	// 82C164AC: 810B0068  lwz r8, 0x68(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C164B0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C164B4: 419A0008  beq cr6, 0x82c164bc
	if ctx.cr[6].eq {
	pc = 0x82C164BC; continue 'dispatch;
	}
	// 82C164B8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82C164BC: 810A0080  lwz r8, 0x80(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C164C0: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C164C4: 409A0008  bne cr6, 0x82c164cc
	if !ctx.cr[6].eq {
	pc = 0x82C164CC; continue 'dispatch;
	}
	// 82C164C8: 934A0080  stw r26, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[26].u32 ) };
	// 82C164CC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C164D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C164D4: 409AFFC0  bne cr6, 0x82c16494
	if !ctx.cr[6].eq {
	pc = 0x82C16494; continue 'dispatch;
	}
	// 82C164D8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C164DC: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82C164E0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C164E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C164E8: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C164EC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C164F0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C164F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C164F8: 419A000C  beq cr6, 0x82c16504
	if ctx.cr[6].eq {
	pc = 0x82C16504; continue 'dispatch;
	}
	// 82C164FC: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C16500: 419A0008  beq cr6, 0x82c16508
	if ctx.cr[6].eq {
	pc = 0x82C16508; continue 'dispatch;
	}
	// 82C16504: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16508: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1650C: 419A0050  beq cr6, 0x82c1655c
	if ctx.cr[6].eq {
	pc = 0x82C1655C; continue 'dispatch;
	}
	// 82C16510: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16514: 409A0008  bne cr6, 0x82c1651c
	if !ctx.cr[6].eq {
	pc = 0x82C1651C; continue 'dispatch;
	}
	// 82C16518: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1651C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16520: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16524: 409A0008  bne cr6, 0x82c1652c
	if !ctx.cr[6].eq {
	pc = 0x82C1652C; continue 'dispatch;
	}
	// 82C16528: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1652C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C16530: 7F09D840  cmplw cr6, r9, r27
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C16534: 409A0014  bne cr6, 0x82c16548
	if !ctx.cr[6].eq {
	pc = 0x82C16548; continue 'dispatch;
	}
	// 82C16538: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C1653C: 409A0008  bne cr6, 0x82c16544
	if !ctx.cr[6].eq {
	pc = 0x82C16544; continue 'dispatch;
	}
	// 82C16540: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16544: 934A0010  stw r26, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82C16548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1654C: 4B94030D  bl 0x82556858
	ctx.lr = 0x82C16550;
	sub_82556858(ctx, base);
	// 82C16550: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C16554: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16558: 4BFFFF98  b 0x82c164f0
	pc = 0x82C164F0; continue 'dispatch;
	// 82C1655C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C16560: 3BFE0018  addi r31, r30, 0x18
	ctx.r[31].s64 = ctx.r[30].s64 + 24;
	// 82C16564: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C16568: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C1656C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16570: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C16574: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16578: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1657C: 419A000C  beq cr6, 0x82c16588
	if ctx.cr[6].eq {
	pc = 0x82C16588; continue 'dispatch;
	}
	// 82C16580: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C16584: 419A0008  beq cr6, 0x82c1658c
	if ctx.cr[6].eq {
	pc = 0x82C1658C; continue 'dispatch;
	}
	// 82C16588: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1658C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C16590: 419A0050  beq cr6, 0x82c165e0
	if ctx.cr[6].eq {
	pc = 0x82C165E0; continue 'dispatch;
	}
	// 82C16594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16598: 409A0008  bne cr6, 0x82c165a0
	if !ctx.cr[6].eq {
	pc = 0x82C165A0; continue 'dispatch;
	}
	// 82C1659C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C165A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C165A4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C165A8: 409A0008  bne cr6, 0x82c165b0
	if !ctx.cr[6].eq {
	pc = 0x82C165B0; continue 'dispatch;
	}
	// 82C165AC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C165B0: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C165B4: 7F09D840  cmplw cr6, r9, r27
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C165B8: 409A0014  bne cr6, 0x82c165cc
	if !ctx.cr[6].eq {
	pc = 0x82C165CC; continue 'dispatch;
	}
	// 82C165BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C165C0: 409A0008  bne cr6, 0x82c165c8
	if !ctx.cr[6].eq {
	pc = 0x82C165C8; continue 'dispatch;
	}
	// 82C165C4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C165C8: 934A0010  stw r26, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82C165CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C165D0: 4B940289  bl 0x82556858
	ctx.lr = 0x82C165D4;
	sub_82556858(ctx, base);
	// 82C165D4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C165D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C165DC: 4BFFFF98  b 0x82c16574
	pc = 0x82C16574; continue 'dispatch;
	// 82C165E0: 83FB0014  lwz r31, 0x14(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C165E4: 3BBB0014  addi r29, r27, 0x14
	ctx.r[29].s64 = ctx.r[27].s64 + 20;
	// 82C165E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C165EC: 419A0080  beq cr6, 0x82c1666c
	if ctx.cr[6].eq {
	pc = 0x82C1666C; continue 'dispatch;
	}
	// 82C165F0: 3BDA0014  addi r30, r26, 0x14
	ctx.r[30].s64 = ctx.r[26].s64 + 20;
	// 82C165F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C165F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C165FC: 4BFFB865  bl 0x82c11e60
	ctx.lr = 0x82C16600;
	sub_82C11E60(ctx, base);
	// 82C16600: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C16604: 397F0038  addi r11, r31, 0x38
	ctx.r[11].s64 = ctx.r[31].s64 + 56;
	// 82C16608: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1660C: 409A0054  bne cr6, 0x82c16660
	if !ctx.cr[6].eq {
	pc = 0x82C16660; continue 'dispatch;
	}
	// 82C16610: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16614: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16618: 409A0048  bne cr6, 0x82c16660
	if !ctx.cr[6].eq {
	pc = 0x82C16660; continue 'dispatch;
	}
	// 82C1661C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16620: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16624: 409A003C  bne cr6, 0x82c16660
	if !ctx.cr[6].eq {
	pc = 0x82C16660; continue 'dispatch;
	}
	// 82C16628: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C1662C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16630: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16634: 409A0014  bne cr6, 0x82c16648
	if !ctx.cr[6].eq {
	pc = 0x82C16648; continue 'dispatch;
	}
	// 82C16638: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C1663C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C16640: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C16644: 48000018  b 0x82c1665c
	pc = 0x82C1665C; continue 'dispatch;
	// 82C16648: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1664C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C16650: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C16654: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16658: 93E90038  stw r31, 0x38(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 82C1665C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C16660: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16664: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C16668: 409AFF8C  bne cr6, 0x82c165f4
	if !ctx.cr[6].eq {
	pc = 0x82C165F4; continue 'dispatch;
	}
	// 82C1666C: 83FB000C  lwz r31, 0xc(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C16670: 3BBB000C  addi r29, r27, 0xc
	ctx.r[29].s64 = ctx.r[27].s64 + 12;
	// 82C16674: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C16678: 419A0080  beq cr6, 0x82c166f8
	if ctx.cr[6].eq {
	pc = 0x82C166F8; continue 'dispatch;
	}
	// 82C1667C: 3BDA000C  addi r30, r26, 0xc
	ctx.r[30].s64 = ctx.r[26].s64 + 12;
	// 82C16680: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C16684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C16688: 4BFFB6D1  bl 0x82c11d58
	ctx.lr = 0x82C1668C;
	sub_82C11D58(ctx, base);
	// 82C1668C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C16690: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82C16694: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16698: 409A0054  bne cr6, 0x82c166ec
	if !ctx.cr[6].eq {
	pc = 0x82C166EC; continue 'dispatch;
	}
	// 82C1669C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C166A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C166A4: 409A0048  bne cr6, 0x82c166ec
	if !ctx.cr[6].eq {
	pc = 0x82C166EC; continue 'dispatch;
	}
	// 82C166A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C166AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C166B0: 409A003C  bne cr6, 0x82c166ec
	if !ctx.cr[6].eq {
	pc = 0x82C166EC; continue 'dispatch;
	}
	// 82C166B4: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C166B8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C166BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C166C0: 409A0014  bne cr6, 0x82c166d4
	if !ctx.cr[6].eq {
	pc = 0x82C166D4; continue 'dispatch;
	}
	// 82C166C4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C166C8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C166CC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C166D0: 48000018  b 0x82c166e8
	pc = 0x82C166E8; continue 'dispatch;
	// 82C166D4: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C166D8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C166DC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C166E0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C166E4: 93E90004  stw r31, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C166E8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C166EC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C166F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C166F4: 409AFF8C  bne cr6, 0x82c16680
	if !ctx.cr[6].eq {
	pc = 0x82C16680; continue 'dispatch;
	}
	// 82C166F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C166FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C16700: 48092D50  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C16704: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C16708: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1670C: 48092D44  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C16710 size=724
    let mut pc: u32 = 0x82C16710;
    'dispatch: loop {
        match pc {
            0x82C16710 => {
    //   block [0x82C16710..0x82C169E4)
	// 82C16710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16714: 48092CED  bl 0x82ca9400
	ctx.lr = 0x82C16718;
	sub_82CA93D0(ctx, base);
	// 82C16718: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1671C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C16720: 897C0311  lbz r11, 0x311(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(785 as u32) ) } as u64;
	// 82C16724: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16728: 409A02B4  bne cr6, 0x82c169dc
	if !ctx.cr[6].eq {
	pc = 0x82C169DC; continue 'dispatch;
	}
	// 82C1672C: 39600270  li r11, 0x270
	ctx.r[11].s64 = 624;
	// 82C16730: C03C0280  lfs f1, 0x280(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(640 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C16734: 39400260  li r10, 0x260
	ctx.r[10].s64 = 608;
	// 82C16738: 39200250  li r9, 0x250
	ctx.r[9].s64 = 592;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C169E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C169E8 size=156
    let mut pc: u32 = 0x82C169E8;
    'dispatch: loop {
        match pc {
            0x82C169E8 => {
    //   block [0x82C169E8..0x82C16A84)
	// 82C169E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C169EC: 48092A21  bl 0x82ca940c
	ctx.lr = 0x82C169F0;
	sub_82CA93D0(ctx, base);
	// 82C169F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C169F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C169F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C169FC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C16A00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C16A04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C16A08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16A0C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16A10: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C16A14: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16A18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16A1C: 419A000C  beq cr6, 0x82c16a28
	if ctx.cr[6].eq {
	pc = 0x82C16A28; continue 'dispatch;
	}
	// 82C16A20: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C16A24: 419A0008  beq cr6, 0x82c16a2c
	if ctx.cr[6].eq {
	pc = 0x82C16A2C; continue 'dispatch;
	}
	// 82C16A28: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16A2C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C16A30: 419A0044  beq cr6, 0x82c16a74
	if ctx.cr[6].eq {
	pc = 0x82C16A74; continue 'dispatch;
	}
	// 82C16A34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16A38: 409A0008  bne cr6, 0x82c16a40
	if !ctx.cr[6].eq {
	pc = 0x82C16A40; continue 'dispatch;
	}
	// 82C16A3C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16A40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16A44: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16A48: 409A0008  bne cr6, 0x82c16a50
	if !ctx.cr[6].eq {
	pc = 0x82C16A50; continue 'dispatch;
	}
	// 82C16A4C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16A50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C16A54: 806A0010  lwz r3, 0x10(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C16A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C16A5C: 4BFFC555  bl 0x82c12fb0
	ctx.lr = 0x82C16A60;
	sub_82C12FB0(ctx, base);
	// 82C16A60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C16A64: 4B93FDF5  bl 0x82556858
	ctx.lr = 0x82C16A68;
	sub_82556858(ctx, base);
	// 82C16A68: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C16A6C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16A70: 4BFFFFA4  b 0x82c16a14
	pc = 0x82C16A14; continue 'dispatch;
	// 82C16A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C16A78: 4BFFFC99  bl 0x82c16710
	ctx.lr = 0x82C16A7C;
	sub_82C16710(ctx, base);
	// 82C16A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C16A80: 480929DC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C16A88 size=120
    let mut pc: u32 = 0x82C16A88;
    'dispatch: loop {
        match pc {
            0x82C16A88 => {
    //   block [0x82C16A88..0x82C16B00)
	// 82C16A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16A8C: 48092981  bl 0x82ca940c
	ctx.lr = 0x82C16A90;
	sub_82CA93D0(ctx, base);
	// 82C16A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16A94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C16A98: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82C16A9C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C16AA0: 4B6087B9  bl 0x8221f258
	ctx.lr = 0x82C16AA4;
	sub_8221F258(ctx, base);
	// 82C16AA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C16AA8: 419A0014  beq cr6, 0x82c16abc
	if ctx.cr[6].eq {
	pc = 0x82C16ABC; continue 'dispatch;
	}
	// 82C16AAC: 4BFFF455  bl 0x82c15f00
	ctx.lr = 0x82C16AB0;
	sub_82C15F00(ctx, base);
	// 82C16AB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16AB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C16AB8: 409A001C  bne cr6, 0x82c16ad4
	if !ctx.cr[6].eq {
	pc = 0x82C16AD4; continue 'dispatch;
	}
	// 82C16ABC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C16AC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C16AC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16AC8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C16ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C16AD0: 4809298C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C16AD4: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82C16AD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C16ADC: 387D003C  addi r3, r29, 0x3c
	ctx.r[3].s64 = ctx.r[29].s64 + 60;
	// 82C16AE0: 4BFFDFD9  bl 0x82c14ab8
	ctx.lr = 0x82C16AE4;
	sub_82C14AB8(ctx, base);
	// 82C16AE4: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82C16AE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C16AEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C16AF0: 4B5BE6F1  bl 0x821d51e0
	ctx.lr = 0x82C16AF4;
	sub_821D51E0(ctx, base);
	// 82C16AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C16AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C16AFC: 48092960  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C16B00 size=172
    let mut pc: u32 = 0x82C16B00;
    'dispatch: loop {
        match pc {
            0x82C16B00 => {
    //   block [0x82C16B00..0x82C16BAC)
	// 82C16B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C16B08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C16B0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C16B10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16B14: 3BC30028  addi r30, r3, 0x28
	ctx.r[30].s64 = ctx.r[3].s64 + 40;
	// 82C16B18: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C16B1C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C16B20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C16B24: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C16B28: 480366B1  bl 0x82c4d1d8
	ctx.lr = 0x82C16B2C;
	sub_82C4D1D8(ctx, base);
	// 82C16B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16B30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16B34: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C16B38: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C16B3C: 40820058  bne 0x82c16b94
	if !ctx.cr[0].eq {
	pc = 0x82C16B94; continue 'dispatch;
	}
	// 82C16B40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C16B44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C16B48: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C16B4C: 48036EED  bl 0x82c4da38
	ctx.lr = 0x82C16B50;
	sub_82C4DA38(ctx, base);
	// 82C16B50: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C16B54: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82C16B58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C16B5C: 419A0014  beq cr6, 0x82c16b70
	if ctx.cr[6].eq {
	pc = 0x82C16B70; continue 'dispatch;
	}
	// 82C16B60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16B64: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16B68: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C16B6C: 4E800421  bctrl
	ctx.lr = 0x82C16B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C16B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C16B74: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C16B78: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C16B7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16B80: 4BF802F9  bl 0x82b96e78
	ctx.lr = 0x82C16B84;
	sub_82B96E78(ctx, base);
	// 82C16B84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C16B88: 4B5FE251  bl 0x82214dd8
	ctx.lr = 0x82C16B8C;
	sub_82214DD8(ctx, base);
	// 82C16B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C16B90: 4BC2EC21  bl 0x828457b0
	ctx.lr = 0x82C16B94;
	sub_828457B0(ctx, base);
	// 82C16B94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C16B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C16B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C16BA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C16BA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C16BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C16BB0 size=368
    let mut pc: u32 = 0x82C16BB0;
    'dispatch: loop {
        match pc {
            0x82C16BB0 => {
    //   block [0x82C16BB0..0x82C16D20)
	// 82C16BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16BB4: 4809284D  bl 0x82ca9400
	ctx.lr = 0x82C16BB8;
	sub_82CA93D0(ctx, base);
	// 82C16BB8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16BBC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C16BC0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C16BC4: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82C16BC8: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82C16BCC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16BD0: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C16BD4: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16BD8: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16BDC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C16BE0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82C16BE4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16BE8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C16BEC: 419A000C  beq cr6, 0x82c16bf8
	if ctx.cr[6].eq {
	pc = 0x82C16BF8; continue 'dispatch;
	}
	// 82C16BF0: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C16BF4: 419A0008  beq cr6, 0x82c16bfc
	if ctx.cr[6].eq {
	pc = 0x82C16BFC; continue 'dispatch;
	}
	// 82C16BF8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16BFC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16C00: 419A0114  beq cr6, 0x82c16d14
	if ctx.cr[6].eq {
	pc = 0x82C16D14; continue 'dispatch;
	}
	// 82C16C04: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C16C08: 409A0008  bne cr6, 0x82c16c10
	if !ctx.cr[6].eq {
	pc = 0x82C16C10; continue 'dispatch;
	}
	// 82C16C0C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16C10: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16C14: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16C18: 409A0008  bne cr6, 0x82c16c20
	if !ctx.cr[6].eq {
	pc = 0x82C16C20; continue 'dispatch;
	}
	// 82C16C1C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16C20: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C16C24: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C16C28: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82C16C2C: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C16C34: 4BFFB66D  bl 0x82c122a0
	ctx.lr = 0x82C16C38;
	sub_82C122A0(ctx, base);
	// 82C16C38: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16C3C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C16C40: 419A0018  beq cr6, 0x82c16c58
	if ctx.cr[6].eq {
	pc = 0x82C16C58; continue 'dispatch;
	}
	// 82C16C44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16C48: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C16C4C: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C16C50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C16C54: 409A0018  bne cr6, 0x82c16c6c
	if !ctx.cr[6].eq {
	pc = 0x82C16C6C; continue 'dispatch;
	}
	// 82C16C58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C16C5C: 4B93FBFD  bl 0x82556858
	ctx.lr = 0x82C16C60;
	sub_82556858(ctx, base);
	// 82C16C60: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C16C64: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C16C68: 4BFFFF7C  b 0x82c16be4
	pc = 0x82C16BE4; continue 'dispatch;
	// 82C16C6C: 897F000F  lbz r11, 0xf(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(15 as u32) ) } as u64;
	// 82C16C70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16C74: 419A00A0  beq cr6, 0x82c16d14
	if ctx.cr[6].eq {
	pc = 0x82C16D14; continue 'dispatch;
	}
	// 82C16C78: 897F000B  lbz r11, 0xb(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(11 as u32) ) } as u64;
	// 82C16C7C: 895F000A  lbz r10, 0xa(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82C16C80: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C16C84: 40980090  bge cr6, 0x82c16d14
	if !ctx.cr[6].lt {
	pc = 0x82C16D14; continue 'dispatch;
	}
	// 82C16C88: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16C8C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16C90: 409A0008  bne cr6, 0x82c16c98
	if !ctx.cr[6].eq {
	pc = 0x82C16C98; continue 'dispatch;
	}
	// 82C16C94: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16C98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C16C9C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16CA0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C16CA4: 4BFEBC2D  bl 0x82c028d0
	ctx.lr = 0x82C16CA8;
	sub_82C028D0(ctx, base);
	// 82C16CA8: 574B0738  rlwinm r11, r26, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82C16CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16CB0: 409A0010  bne cr6, 0x82c16cc0
	if !ctx.cr[6].eq {
	pc = 0x82C16CC0; continue 'dispatch;
	}
	// 82C16CB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C16CB8: 889F000C  lbz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C16CBC: 4BFEBC2D  bl 0x82c028e8
	ctx.lr = 0x82C16CC0;
	sub_82C028E8(ctx, base);
	// 82C16CC0: 574B05AC  rlwinm r11, r26, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82C16CC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16CC8: 409A0010  bne cr6, 0x82c16cd8
	if !ctx.cr[6].eq {
	pc = 0x82C16CD8; continue 'dispatch;
	}
	// 82C16CCC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C16CD0: 889F000D  lbz r4, 0xd(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 82C16CD4: 4BFEBCA5  bl 0x82c02978
	ctx.lr = 0x82C16CD8;
	sub_82C02978(ctx, base);
	// 82C16CD8: 574B056A  rlwinm r11, r26, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82C16CDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16CE0: 409A0010  bne cr6, 0x82c16cf0
	if !ctx.cr[6].eq {
	pc = 0x82C16CF0; continue 'dispatch;
	}
	// 82C16CE4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C16CE8: 889F000E  lbz r4, 0xe(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 82C16CEC: 4BFEBCA5  bl 0x82c02990
	ctx.lr = 0x82C16CF0;
	sub_82C02990(ctx, base);
	// 82C16CF0: 574B04E6  rlwinm r11, r26, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82C16CF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16CF8: 409A0010  bne cr6, 0x82c16d08
	if !ctx.cr[6].eq {
	pc = 0x82C16D08; continue 'dispatch;
	}
	// 82C16CFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C16D00: A09F0008  lhz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C16D04: 4BFEBCA5  bl 0x82c029a8
	ctx.lr = 0x82C16D08;
	sub_82C029A8(ctx, base);
	// 82C16D08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C16D0C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C16D10: 48092740  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C16D14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C16D18: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C16D1C: 48092734  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C16D20 size=44
    let mut pc: u32 = 0x82C16D20;
    'dispatch: loop {
        match pc {
            0x82C16D20 => {
    //   block [0x82C16D20..0x82C16D4C)
	// 82C16D20: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16D24: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16D28: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C16D2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C16D30: 409A0030  bne cr6, 0x82c16d60
	if !ctx.cr[6].eq {
		sub_82C16D4C(ctx, base);
		return;
	}
	// 82C16D34: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16D38: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C16D3C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C16D40: 4098000C  bge cr6, 0x82c16d4c
	if !ctx.cr[6].lt {
		sub_82C16D4C(ctx, base);
		return;
	}
	// 82C16D44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C16D48: 4800000C  b 0x82c16d54
	sub_82C16D4C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16D4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C16D4C size=80
    let mut pc: u32 = 0x82C16D4C;
    'dispatch: loop {
        match pc {
            0x82C16D4C => {
    //   block [0x82C16D4C..0x82C16D9C)
	// 82C16D4C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C16D50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16D54: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C16D58: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C16D5C: 419AFFDC  beq cr6, 0x82c16d38
	if ctx.cr[6].eq {
		sub_82C16D20(ctx, base);
		return;
	}
	// 82C16D60: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16D64: 9141FFF4  stw r10, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 82C16D68: 9081FFF0  stw r4, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u32 ) };
	// 82C16D6C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16D70: 419A002C  beq cr6, 0x82c16d9c
	if ctx.cr[6].eq {
		sub_82C16D9C(ctx, base);
		return;
	}
	// 82C16D74: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16D78: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C16D7C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C16D80: 4198001C  blt cr6, 0x82c16d9c
	if ctx.cr[6].lt {
		sub_82C16D9C(ctx, base);
		return;
	}
	// 82C16D84: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82C16D88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16D8C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16D90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C16D94: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C16D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16D9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C16D9C size=32
    let mut pc: u32 = 0x82C16D9C;
    'dispatch: loop {
        match pc {
            0x82C16D9C => {
    //   block [0x82C16D9C..0x82C16DBC)
	// 82C16D9C: 9161FFFC  stw r11, -4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82C16DA0: 3961FFF8  addi r11, r1, -8
	ctx.r[11].s64 = ctx.r[1].s64 + -8;
	// 82C16DA4: 9081FFF8  stw r4, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[4].u32 ) };
	// 82C16DA8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16DAC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C16DB0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C16DB4: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C16DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C16DC0 size=136
    let mut pc: u32 = 0x82C16DC0;
    'dispatch: loop {
        match pc {
            0x82C16DC0 => {
    //   block [0x82C16DC0..0x82C16E48)
	// 82C16DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C16DC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C16DCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16DD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16DD4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82C16DD8: 4B608481  bl 0x8221f258
	ctx.lr = 0x82C16DDC;
	sub_8221F258(ctx, base);
	// 82C16DDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C16DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C16DE4: 419A0008  beq cr6, 0x82c16dec
	if ctx.cr[6].eq {
	pc = 0x82C16DEC; continue 'dispatch;
	}
	// 82C16DE8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16DEC: 35430004  addic. r10, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C16DF0: 41820008  beq 0x82c16df8
	if ctx.cr[0].eq {
	pc = 0x82C16DF8; continue 'dispatch;
	}
	// 82C16DF4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16DF8: 35430008  addic. r10, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C16DFC: 41820008  beq 0x82c16e04
	if ctx.cr[0].eq {
	pc = 0x82C16E04; continue 'dispatch;
	}
	// 82C16E00: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C16E04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C16E08: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82C16E0C: 99430028  stb r10, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82C16E10: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C16E14: 99430029  stb r10, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 82C16E18: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16E1C: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C16E20: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16E24: 91290000  stw r9, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C16E28: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16E2C: 91080008  stw r8, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C16E30: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C16E34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C16E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C16E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C16E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C16E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C16E48 size=196
    let mut pc: u32 = 0x82C16E48;
    'dispatch: loop {
        match pc {
            0x82C16E48 => {
    //   block [0x82C16E48..0x82C16F0C)
	// 82C16E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16E4C: 480925C1  bl 0x82ca940c
	ctx.lr = 0x82C16E50;
	sub_82CA93D0(ctx, base);
	// 82C16E50: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C16E54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16E58: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C16E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16E60: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82C16E64: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C16E68: 3BDF0054  addi r30, r31, 0x54
	ctx.r[30].s64 = ctx.r[31].s64 + 84;
	// 82C16E6C: D3E100A4  stfs f31, 0xa4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82C16E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C16E74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C16E78: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C16E7C: 4BFFFEA5  bl 0x82c16d20
	ctx.lr = 0x82C16E80;
	sub_82C16D20(ctx, base);
	// 82C16E80: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16E84: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C16E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16E8C: 419A000C  beq cr6, 0x82c16e98
	if ctx.cr[6].eq {
	pc = 0x82C16E98; continue 'dispatch;
	}
	// 82C16E90: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C16E94: 419A0008  beq cr6, 0x82c16e9c
	if ctx.cr[6].eq {
	pc = 0x82C16E9C; continue 'dispatch;
	}
	// 82C16E98: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16E9C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C16EA0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C16EA4: 419A005C  beq cr6, 0x82c16f00
	if ctx.cr[6].eq {
	pc = 0x82C16F00; continue 'dispatch;
	}
	// 82C16EA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C16EAC: 409A0008  bne cr6, 0x82c16eb4
	if !ctx.cr[6].eq {
	pc = 0x82C16EB4; continue 'dispatch;
	}
	// 82C16EB0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16EB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16EB8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C16EBC: 409A0008  bne cr6, 0x82c16ec4
	if !ctx.cr[6].eq {
	pc = 0x82C16EC4; continue 'dispatch;
	}
	// 82C16EC0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16EC4: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C16EC8: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
	// 82C16ECC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82C16ED0: 4199000C  bgt cr6, 0x82c16edc
	if ctx.cr[6].gt {
	pc = 0x82C16EDC; continue 'dispatch;
	}
	// 82C16ED4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82C16ED8: 48000018  b 0x82c16ef0
	pc = 0x82C16EF0; continue 'dispatch;
	// 82C16EDC: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C16EE0: 38AB0004  addi r5, r11, 4
	ctx.r[5].s64 = ctx.r[11].s64 + 4;
	// 82C16EE4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82C16EE8: 40980008  bge cr6, 0x82c16ef0
	if !ctx.cr[6].lt {
	pc = 0x82C16EF0; continue 'dispatch;
	}
	// 82C16EEC: 38A100A4  addi r5, r1, 0xa4
	ctx.r[5].s64 = ctx.r[1].s64 + 164;
	// 82C16EF0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C16EF4: 809F0288  lwz r4, 0x288(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C16EF8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82C16EFC: 4BFFC295  bl 0x82c13190
	ctx.lr = 0x82C16F00;
	sub_82C13190(ctx, base);
	// 82C16F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C16F04: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C16F08: 48092554  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C16F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C16F10 size=244
    let mut pc: u32 = 0x82C16F10;
    'dispatch: loop {
        match pc {
            0x82C16F10 => {
    //   block [0x82C16F10..0x82C17004)
	// 82C16F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C16F14: 480924F9  bl 0x82ca940c
	ctx.lr = 0x82C16F18;
	sub_82CA93D0(ctx, base);
	// 82C16F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C16F1C: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C16F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C16F24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C16F28: 3BDF0054  addi r30, r31, 0x54
	ctx.r[30].s64 = ctx.r[31].s64 + 84;
	// 82C16F2C: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82C16F30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C16F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C16F38: 4BFFFDE9  bl 0x82c16d20
	ctx.lr = 0x82C16F3C;
	sub_82C16D20(ctx, base);
	// 82C16F3C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C16F40: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C16F44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16F48: 419A000C  beq cr6, 0x82c16f54
	if ctx.cr[6].eq {
	pc = 0x82C16F54; continue 'dispatch;
	}
	// 82C16F4C: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C16F50: 419A0008  beq cr6, 0x82c16f58
	if ctx.cr[6].eq {
	pc = 0x82C16F58; continue 'dispatch;
	}
	// 82C16F54: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16F58: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C16F5C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C16F60: 409A0010  bne cr6, 0x82c16f70
	if !ctx.cr[6].eq {
	pc = 0x82C16F70; continue 'dispatch;
	}
	// 82C16F64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C16F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C16F6C: 480924F0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C16F70: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C16F74: 409A0008  bne cr6, 0x82c16f7c
	if !ctx.cr[6].eq {
	pc = 0x82C16F7C; continue 'dispatch;
	}
	// 82C16F78: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16F7C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C16F80: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C16F84: 409A0008  bne cr6, 0x82c16f8c
	if !ctx.cr[6].eq {
	pc = 0x82C16F8C; continue 'dispatch;
	}
	// 82C16F88: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C16F8C: 813F0288  lwz r9, 0x288(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C16F90: 810B0018  lwz r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C16F94: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C16F98: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82C16F9C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C16FA0: 41980018  blt cr6, 0x82c16fb8
	if ctx.cr[6].lt {
	pc = 0x82C16FB8; continue 'dispatch;
	}
	// 82C16FA4: C00B0024  lfs f0, 0x24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C16FA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C16FAC: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C16FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C16FB4: 480924A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C16FB8: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C16FBC: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C16FC0: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C16FC4: C1AB0024  lfs f13, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C16FC8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82C16FCC: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C16FD0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82C16FD4: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C16FD8: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 82C16FDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C16FE0: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C16FE4: FCC04818  frsp f6, f9
	ctx.f[6].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C16FE8: ECED0028  fsubs f7, f13, f0
	ctx.f[7].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C16FEC: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C16FF0: ECA83024  fdivs f5, f8, f6
	ctx.f[5].f64 = ((ctx.f[8].f64 / ctx.f[6].f64) as f32) as f64;
	// 82C16FF4: EC0501FA  fmadds f0, f5, f7, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C16FF8: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C16FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C17000: 4809245C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C17008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C17008 size=156
    let mut pc: u32 = 0x82C17008;
    'dispatch: loop {
        match pc {
            0x82C17008 => {
    //   block [0x82C17008..0x82C170A4)
	// 82C17008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1700C: 480923FD  bl 0x82ca9408
	ctx.lr = 0x82C17010;
	sub_82CA93D0(ctx, base);
	// 82C17010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C17014: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C17018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1701C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C17020: 3BDF0054  addi r30, r31, 0x54
	ctx.r[30].s64 = ctx.r[31].s64 + 84;
	// 82C17024: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82C17028: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1702C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C17030: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C17034: 4BFFFCED  bl 0x82c16d20
	ctx.lr = 0x82C17038;
	sub_82C16D20(ctx, base);
	// 82C17038: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C1703C: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C17040: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17044: 419A000C  beq cr6, 0x82c17050
	if ctx.cr[6].eq {
	pc = 0x82C17050; continue 'dispatch;
	}
	// 82C17048: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C1704C: 419A0008  beq cr6, 0x82c17054
	if ctx.cr[6].eq {
	pc = 0x82C17054; continue 'dispatch;
	}
	// 82C17050: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17054: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C17058: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1705C: 409A0010  bne cr6, 0x82c1706c
	if !ctx.cr[6].eq {
	pc = 0x82C1706C; continue 'dispatch;
	}
	// 82C17060: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C17064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C17068: 480923F0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C1706C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17070: 409A0008  bne cr6, 0x82c17078
	if !ctx.cr[6].eq {
	pc = 0x82C17078; continue 'dispatch;
	}
	// 82C17074: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17078: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1707C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C17080: 409A0008  bne cr6, 0x82c17088
	if !ctx.cr[6].eq {
	pc = 0x82C17088; continue 'dispatch;
	}
	// 82C17084: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17088: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1708C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C17090: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C17094: C1AB0014  lfs f13, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C17098: D1BC0000  stfs f13, 0(r28)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C1709C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C170A0: 480923B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C170A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C170A8 size=264
    let mut pc: u32 = 0x82C170A8;
    'dispatch: loop {
        match pc {
            0x82C170A8 => {
    //   block [0x82C170A8..0x82C171B0)
	// 82C170A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C170AC: 48092359  bl 0x82ca9404
	ctx.lr = 0x82C170B0;
	sub_82CA93D0(ctx, base);
	// 82C170B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C170B4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C170B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C170BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C170C0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C170C4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82C170C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C170CC: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C170D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C170D4: 409A001C  bne cr6, 0x82c170f0
	if !ctx.cr[6].eq {
	pc = 0x82C170F0; continue 'dispatch;
	}
	// 82C170D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C170DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C170E0: 4BFFFAD1  bl 0x82c16bb0
	ctx.lr = 0x82C170E4;
	sub_82C16BB0(ctx, base);
	// 82C170E4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C170E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C170EC: 419A00AC  beq cr6, 0x82c17198
	if ctx.cr[6].eq {
	pc = 0x82C17198; continue 'dispatch;
	}
	// 82C170F0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C170F4: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C170F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C170FC: 419A009C  beq cr6, 0x82c17198
	if ctx.cr[6].eq {
	pc = 0x82C17198; continue 'dispatch;
	}
	// 82C17100: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C17104: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C17108: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1710C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C17110: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C17114: 4B75E36D  bl 0x82375480
	ctx.lr = 0x82C17118;
	sub_82375480(ctx, base);
	// 82C17118: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C1711C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17120: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17124: 419A000C  beq cr6, 0x82c17130
	if ctx.cr[6].eq {
	pc = 0x82C17130; continue 'dispatch;
	}
	// 82C17128: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C1712C: 419A0008  beq cr6, 0x82c17134
	if ctx.cr[6].eq {
	pc = 0x82C17134; continue 'dispatch;
	}
	// 82C17130: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17134: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C17138: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1713C: 419A005C  beq cr6, 0x82c17198
	if ctx.cr[6].eq {
	pc = 0x82C17198; continue 'dispatch;
	}
	// 82C17140: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17144: 409A0008  bne cr6, 0x82c1714c
	if !ctx.cr[6].eq {
	pc = 0x82C1714C; continue 'dispatch;
	}
	// 82C17148: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1714C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17150: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C17154: 409A0008  bne cr6, 0x82c1715c
	if !ctx.cr[6].eq {
	pc = 0x82C1715C; continue 'dispatch;
	}
	// 82C17158: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1715C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C17160: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C17164: 419A0034  beq cr6, 0x82c17198
	if ctx.cr[6].eq {
	pc = 0x82C17198; continue 'dispatch;
	}
	// 82C17168: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C1716C: 409A0008  bne cr6, 0x82c17174
	if !ctx.cr[6].eq {
	pc = 0x82C17174; continue 'dispatch;
	}
	// 82C17170: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17174: 909B0000  stw r4, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C17178: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C1717C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C17180: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C17184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C17188: 4BFFEF29  bl 0x82c160b0
	ctx.lr = 0x82C1718C;
	sub_82C160B0(ctx, base);
	// 82C1718C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C17190: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C17194: 480922C0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C17198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1719C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C171A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C171A4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C171A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C171AC: 480922A8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C171B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C171B0 size=204
    let mut pc: u32 = 0x82C171B0;
    'dispatch: loop {
        match pc {
            0x82C171B0 => {
    //   block [0x82C171B0..0x82C1727C)
	// 82C171B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C171B4: 48092251  bl 0x82ca9404
	ctx.lr = 0x82C171B8;
	sub_82CA93D0(ctx, base);
	// 82C171B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C171BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C171C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C171C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C171C8: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82C171CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C171D0: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C171D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C171D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C171DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C171E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C171E4: 4B75E29D  bl 0x82375480
	ctx.lr = 0x82C171E8;
	sub_82375480(ctx, base);
	// 82C171E8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C171EC: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C171F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C171F4: 419A000C  beq cr6, 0x82c17200
	if ctx.cr[6].eq {
	pc = 0x82C17200; continue 'dispatch;
	}
	// 82C171F8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C171FC: 419A0008  beq cr6, 0x82c17204
	if ctx.cr[6].eq {
	pc = 0x82C17204; continue 'dispatch;
	}
	// 82C17200: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17204: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C17208: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1720C: 419A0058  beq cr6, 0x82c17264
	if ctx.cr[6].eq {
	pc = 0x82C17264; continue 'dispatch;
	}
	// 82C17210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17214: 409A0008  bne cr6, 0x82c1721c
	if !ctx.cr[6].eq {
	pc = 0x82C1721C; continue 'dispatch;
	}
	// 82C17218: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1721C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17220: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C17224: 409A0008  bne cr6, 0x82c1722c
	if !ctx.cr[6].eq {
	pc = 0x82C1722C; continue 'dispatch;
	}
	// 82C17228: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1722C: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C17230: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C17234: 419A0030  beq cr6, 0x82c17264
	if ctx.cr[6].eq {
	pc = 0x82C17264; continue 'dispatch;
	}
	// 82C17238: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C1723C: 409A0008  bne cr6, 0x82c17244
	if !ctx.cr[6].eq {
	pc = 0x82C17244; continue 'dispatch;
	}
	// 82C17240: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C17244: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82C17248: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82C1724C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C17250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C17254: 4BFFEE5D  bl 0x82c160b0
	ctx.lr = 0x82C17258;
	sub_82C160B0(ctx, base);
	// 82C17258: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1725C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C17260: 480921F4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C17264: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C17268: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1726C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C17270: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C17274: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C17278: 480921DC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C17280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C17280 size=156
    let mut pc: u32 = 0x82C17280;
    'dispatch: loop {
        match pc {
            0x82C17280 => {
    //   block [0x82C17280..0x82C1731C)
	// 82C17280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C17284: 48092189  bl 0x82ca940c
	ctx.lr = 0x82C17288;
	sub_82CA93D0(ctx, base);
	// 82C17288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1728C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C17290: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C17294: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C17298: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1729C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C172A0: 409A0018  bne cr6, 0x82c172b8
	if !ctx.cr[6].eq {
	pc = 0x82C172B8; continue 'dispatch;
	}
	// 82C172A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C172A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C172AC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C172B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C172B4: 480921A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C172B8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C172BC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C172C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C172C4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C172C8: 4BFFEDE9  bl 0x82c160b0
	ctx.lr = 0x82C172CC;
	sub_82C160B0(ctx, base);
	// 82C172CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C172D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C172D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C172D8: 419A0020  beq cr6, 0x82c172f8
	if ctx.cr[6].eq {
	pc = 0x82C172F8; continue 'dispatch;
	}
	// 82C172DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C172E0: 4B81BFF1  bl 0x824332d0
	ctx.lr = 0x82C172E4;
	sub_824332D0(ctx, base);
	// 82C172E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C172E8: 4BFEEC11  bl 0x82c05ef8
	ctx.lr = 0x82C172EC;
	sub_82C05EF8(ctx, base);
	// 82C172EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C172F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C172F4: 48092168  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C172F8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C172FC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C17300: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17304: 4BFFFEAD  bl 0x82c171b0
	ctx.lr = 0x82C17308;
	sub_82C171B0(ctx, base);
	// 82C17308: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1730C: 4BFEEBED  bl 0x82c05ef8
	ctx.lr = 0x82C17310;
	sub_82C05EF8(ctx, base);
	// 82C17310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C17314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C17318: 48092144  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C17320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C17320 size=8
    let mut pc: u32 = 0x82C17320;
    'dispatch: loop {
        match pc {
            0x82C17320 => {
    //   block [0x82C17320..0x82C17328)
	// 82C17320: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17324: 4BFFFBEC  b 0x82c16f10
	sub_82C16F10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C17328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C17328 size=8
    let mut pc: u32 = 0x82C17328;
    'dispatch: loop {
        match pc {
            0x82C17328 => {
    //   block [0x82C17328..0x82C17330)
	// 82C17328: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1732C: 4BFFFCDC  b 0x82c17008
	sub_82C17008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C17330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C17330 size=368
    let mut pc: u32 = 0x82C17330;
    'dispatch: loop {
        match pc {
            0x82C17330 => {
    //   block [0x82C17330..0x82C174A0)
	// 82C17330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C17334: 480920CD  bl 0x82ca9400
	ctx.lr = 0x82C17338;
	sub_82CA93D0(ctx, base);
	// 82C17338: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1733C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C17340: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C17344: 3B5B0034  addi r26, r27, 0x34
	ctx.r[26].s64 = ctx.r[27].s64 + 52;
	// 82C17348: 83BB0034  lwz r29, 0x34(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C1734C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C17350: 419A0138  beq cr6, 0x82c17488
	if ctx.cr[6].eq {
	pc = 0x82C17488; continue 'dispatch;
	}
	// 82C17354: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82C17358: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82C1735C: 38DD0004  addi r6, r29, 4
	ctx.r[6].s64 = ctx.r[29].s64 + 4;
	// 82C17360: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C17364: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C17368: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C1736C: 4BFFFD3D  bl 0x82c170a8
	ctx.lr = 0x82C17370;
	sub_82C170A8(ctx, base);
	// 82C17370: 3BFD004C  addi r31, r29, 0x4c
	ctx.r[31].s64 = ctx.r[29].s64 + 76;
	// 82C17374: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C17378: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C1737C: 419A0050  beq cr6, 0x82c173cc
	if ctx.cr[6].eq {
	pc = 0x82C173CC; continue 'dispatch;
	}
	// 82C17380: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17388: 419A0018  beq cr6, 0x82c173a0
	if ctx.cr[6].eq {
	pc = 0x82C173A0; continue 'dispatch;
	}
	// 82C1738C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C17390: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17394: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17398: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1739C: 4E800421  bctrl
	ctx.lr = 0x82C173A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C173A0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C173A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C173A8: 419A0014  beq cr6, 0x82c173bc
	if ctx.cr[6].eq {
	pc = 0x82C173BC; continue 'dispatch;
	}
	// 82C173AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C173B0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C173B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C173B8: 4E800421  bctrl
	ctx.lr = 0x82C173BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C173BC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C173C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C173C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C173C8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C173CC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C173D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C173D4: 419A0014  beq cr6, 0x82c173e8
	if ctx.cr[6].eq {
	pc = 0x82C173E8; continue 'dispatch;
	}
	// 82C173D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C173DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C173E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C173E4: 4E800421  bctrl
	ctx.lr = 0x82C173E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C173E8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82C173EC: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82C173F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C173F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C173F8: 419A0060  beq cr6, 0x82c17458
	if ctx.cr[6].eq {
	pc = 0x82C17458; continue 'dispatch;
	}
	// 82C173FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C17400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17404: 419A0054  beq cr6, 0x82c17458
	if ctx.cr[6].eq {
	pc = 0x82C17458; continue 'dispatch;
	}
	// 82C17408: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1740C: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17410: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C17414: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C17418: 419A0040  beq cr6, 0x82c17458
	if ctx.cr[6].eq {
	pc = 0x82C17458; continue 'dispatch;
	}
	// 82C1741C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C17420: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82C17424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C17428: 4BFFAE79  bl 0x82c122a0
	ctx.lr = 0x82C1742C;
	sub_82C122A0(ctx, base);
	// 82C1742C: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C17430: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17434: 419A0024  beq cr6, 0x82c17458
	if ctx.cr[6].eq {
	pc = 0x82C17458; continue 'dispatch;
	}
	// 82C17438: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1743C: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C17440: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C17444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17448: 419A0010  beq cr6, 0x82c17458
	if ctx.cr[6].eq {
	pc = 0x82C17458; continue 'dispatch;
	}
	// 82C1744C: 894B000B  lbz r10, 0xb(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82C17450: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C17454: 994B000B  stb r10, 0xb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11 as u32), ctx.r[10].u8 ) };
	// 82C17458: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C1745C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17460: 409A0028  bne cr6, 0x82c17488
	if !ctx.cr[6].eq {
	pc = 0x82C17488; continue 'dispatch;
	}
	// 82C17464: 817D0060  lwz r11, 0x60(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C17468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1746C: 419A0010  beq cr6, 0x82c1747c
	if ctx.cr[6].eq {
	pc = 0x82C1747C; continue 'dispatch;
	}
	// 82C17470: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C17474: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17478: 409A0010  bne cr6, 0x82c17488
	if !ctx.cr[6].eq {
	pc = 0x82C17488; continue 'dispatch;
	}
	// 82C1747C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82C17480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17484: 409AFED0  bne cr6, 0x82c17354
	if !ctx.cr[6].eq {
	pc = 0x82C17354; continue 'dispatch;
	}
	// 82C17488: 9B9B0311  stb r28, 0x311(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(785 as u32), ctx.r[28].u8 ) };
	// 82C1748C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C17490: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C17494: 480AB9B5  bl 0x82cc2e48
	ctx.lr = 0x82C17498;
	sub_82CC2E48(ctx, base);
	// 82C17498: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C1749C: 48091FB4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C174A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C174A0 size=536
    let mut pc: u32 = 0x82C174A0;
    'dispatch: loop {
        match pc {
            0x82C174A0 => {
    //   block [0x82C174A0..0x82C176B8)
	// 82C174A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C174A4: 48091F51  bl 0x82ca93f4
	ctx.lr = 0x82C174A8;
	sub_82CA93D0(ctx, base);
	// 82C174A8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C174AC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82C174B0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82C174B4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C174B8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82C174BC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82C174C0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C174C4: 835E0000  lwz r26, 0(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C174C8: 36FF0004  addic. r23, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[23].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82C174CC: 41820018  beq 0x82c174e4
	if ctx.cr[0].eq {
	pc = 0x82C174E4; continue 'dispatch;
	}
	// 82C174D0: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C174D4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82C174D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C174DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C174E0: 4E800421  bctrl
	ctx.lr = 0x82C174E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C174E4: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82C174E8: 92E1005C  stw r23, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[23].u32 ) };
	// 82C174EC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82C174F0: 4B607D69  bl 0x8221f258
	ctx.lr = 0x82C174F4;
	sub_8221F258(ctx, base);
	// 82C174F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C174F8: 419A0010  beq cr6, 0x82c17508
	if ctx.cr[6].eq {
	pc = 0x82C17508; continue 'dispatch;
	}
	// 82C174FC: 4BFFC085  bl 0x82c13580
	ctx.lr = 0x82C17500;
	sub_82C13580(ctx, base);
	// 82C17500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C17504: 48000008  b 0x82c1750c
	pc = 0x82C1750C; continue 'dispatch;
	// 82C17508: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82C1750C: 3BBF0078  addi r29, r31, 0x78
	ctx.r[29].s64 = ctx.r[31].s64 + 120;
	// 82C17510: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82C17514: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C17518: 4BFFA7B1  bl 0x82c11cc8
	ctx.lr = 0x82C1751C;
	sub_82C11CC8(ctx, base);
	// 82C1751C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17520: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C17524: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C17528: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1752C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82C17530: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82C17534: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C17538: 4BFFFB71  bl 0x82c170a8
	ctx.lr = 0x82C1753C;
	sub_82C170A8(ctx, base);
	// 82C1753C: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C17540: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C17544: 409A0048  bne cr6, 0x82c1758c
	if !ctx.cr[6].eq {
	pc = 0x82C1758C; continue 'dispatch;
	}
	// 82C17548: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1754C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C17550: 419A0014  beq cr6, 0x82c17564
	if ctx.cr[6].eq {
	pc = 0x82C17564; continue 'dispatch;
	}
	// 82C17554: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17558: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1755C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C17560: 4E800421  bctrl
	ctx.lr = 0x82C17564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C17564: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C17568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1756C: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C17570: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17574: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17578: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1757C: 4E800421  bctrl
	ctx.lr = 0x82C17580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C17580: 93980000  stw r28, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C17584: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C17588: 480000EC  b 0x82c17674
	pc = 0x82C17674; continue 'dispatch;
	// 82C1758C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C17590: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C17594: 4B5BDD55  bl 0x821d52e8
	ctx.lr = 0x82C17598;
	sub_821D52E8(ctx, base);
	// 82C17598: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C1759C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82C175A0: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 82C175A4: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82C175A8: 4BFFA721  bl 0x82c11cc8
	ctx.lr = 0x82C175AC;
	sub_82C11CC8(ctx, base);
	// 82C175AC: 574B0672  rlwinm r11, r26, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82C175B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C175B4: 419A0020  beq cr6, 0x82c175d4
	if ctx.cr[6].eq {
	pc = 0x82C175D4; continue 'dispatch;
	}
	// 82C175B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C175BC: C03E0030  lfs f1, 0x30(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C175C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C175C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C175C8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C175CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C175D0: 4E800421  bctrl
	ctx.lr = 0x82C175D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C175D4: 574B0630  rlwinm r11, r26, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82C175D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C175DC: 419A0020  beq cr6, 0x82c175fc
	if ctx.cr[6].eq {
	pc = 0x82C175FC; continue 'dispatch;
	}
	// 82C175E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C175E4: C03E0034  lfs f1, 0x34(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C175E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C175EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C175F0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C175F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C175F8: 4E800421  bctrl
	ctx.lr = 0x82C175FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C175FC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C17600: 419A0010  beq cr6, 0x82c17610
	if ctx.cr[6].eq {
	pc = 0x82C17610; continue 'dispatch;
	}
	// 82C17604: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C17608: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1760C: 4BFFCFD5  bl 0x82c145e0
	ctx.lr = 0x82C17610;
	sub_82C145E0(ctx, base);
	// 82C17610: 3BDF006C  addi r30, r31, 0x6c
	ctx.r[30].s64 = ctx.r[31].s64 + 108;
	// 82C17614: 3BBB0048  addi r29, r27, 0x48
	ctx.r[29].s64 = ctx.r[27].s64 + 72;
	// 82C17618: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C1761C: 419A0018  beq cr6, 0x82c17634
	if ctx.cr[6].eq {
	pc = 0x82C17634; continue 'dispatch;
	}
	// 82C17620: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C17628: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1762C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C17630: 4E800421  bctrl
	ctx.lr = 0x82C17634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C17634: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17638: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1763C: 419A0014  beq cr6, 0x82c17650
	if ctx.cr[6].eq {
	pc = 0x82C17650; continue 'dispatch;
	}
	// 82C17640: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17644: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17648: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1764C: 4E800421  bctrl
	ctx.lr = 0x82C17650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C17650: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C17654: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17658: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C1765C: 38790034  addi r3, r25, 0x34
	ctx.r[3].s64 = ctx.r[25].s64 + 52;
	// 82C17660: 4BFFA899  bl 0x82c11ef8
	ctx.lr = 0x82C17664;
	sub_82C11EF8(ctx, base);
	// 82C17664: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C17668: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1766C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C17670: 4B5BDB71  bl 0x821d51e0
	ctx.lr = 0x82C17674;
	sub_821D51E0(ctx, base);
	// 82C17674: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C17678: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1767C: 419A0014  beq cr6, 0x82c17690
	if ctx.cr[6].eq {
	pc = 0x82C17690; continue 'dispatch;
	}
	// 82C17680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17684: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17688: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1768C: 4E800421  bctrl
	ctx.lr = 0x82C17690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C17690: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82C17694: 419A0018  beq cr6, 0x82c176ac
	if ctx.cr[6].eq {
	pc = 0x82C176AC; continue 'dispatch;
	}
	// 82C17698: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1769C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82C176A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C176A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C176A8: 4E800421  bctrl
	ctx.lr = 0x82C176AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C176AC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C176B0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C176B4: 48091D90  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C176B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C176B8 size=236
    let mut pc: u32 = 0x82C176B8;
    'dispatch: loop {
        match pc {
            0x82C176B8 => {
    //   block [0x82C176B8..0x82C177A4)
	// 82C176B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C176BC: 48091D4D  bl 0x82ca9408
	ctx.lr = 0x82C176C0;
	sub_82CA93D0(ctx, base);
	// 82C176C0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C176C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C176C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C176CC: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C176D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C176D4: 409A001C  bne cr6, 0x82c176f0
	if !ctx.cr[6].eq {
	pc = 0x82C176F0; continue 'dispatch;
	}
	// 82C176D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C176DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C176E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C176E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C176E8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C176EC: 48091D6C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C176F0: 838B0034  lwz r28, 0x34(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C176F4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C176F8: 419AFFE0  beq cr6, 0x82c176d8
	if ctx.cr[6].eq {
	pc = 0x82C176D8; continue 'dispatch;
	}
	// 82C176FC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C17700: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C17704: 4BFFA0E5  bl 0x82c117e8
	ctx.lr = 0x82C17708;
	sub_82C117E8(ctx, base);
	// 82C17708: 3BBE0050  addi r29, r30, 0x50
	ctx.r[29].s64 = ctx.r[30].s64 + 80;
	// 82C1770C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C177A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C177A8 size=1068
    let mut pc: u32 = 0x82C177A8;
    'dispatch: loop {
        match pc {
            0x82C177A8 => {
    //   block [0x82C177A8..0x82C17BD4)
	// 82C177A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C177AC: 48091C51  bl 0x82ca93fc
	ctx.lr = 0x82C177B0;
	sub_82CA93D0(ctx, base);
	// 82C177B0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C177B4: F8A10100  std r5, 0x100(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[5].u64 ) };
	// 82C177B8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82C177BC: 83E10104  lwz r31, 0x104(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82C177C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C177C4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C177C8: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C177CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C177D0: 419A005C  beq cr6, 0x82c1782c
	if ctx.cr[6].eq {
	pc = 0x82C1782C; continue 'dispatch;
	}
	// 82C177D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C177D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C177DC: 388B16C0  addi r4, r11, 0x16c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5824;
	// 82C177E0: 4B6DA761  bl 0x822f1f40
	ctx.lr = 0x82C177E4;
	sub_822F1F40(ctx, base);
	// 82C177E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C177E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C177EC: 4B6DA585  bl 0x822f1d70
	ctx.lr = 0x82C177F0;
	sub_822F1D70(ctx, base);
	// 82C177F0: 4B6DA631  bl 0x822f1e20
	ctx.lr = 0x82C177F4;
	sub_822F1E20(ctx, base);
	// 82C177F4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82C177F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C177FC: 392A1720  addi r9, r10, 0x1720
	ctx.r[9].s64 = ctx.r[10].s64 + 5920;
	// 82C17800: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82C17804: 4BA63FDD  bl 0x8267b7e0
	ctx.lr = 0x82C17808;
	sub_8267B7E0(ctx, base);
	// 82C17808: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C1780C: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82C17810: 4198000C  blt cr6, 0x82c1781c
	if ctx.cr[6].lt {
	pc = 0x82C1781C; continue 'dispatch;
	}
	// 82C17814: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C17818: 4BC2DF99  bl 0x828457b0
	ctx.lr = 0x82C1781C;
	sub_828457B0(ctx, base);
	// 82C1781C: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82C17820: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82C17824: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82C17828: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82C1782C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82C17830: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82C17834: 4B93F025  bl 0x82556858
	ctx.lr = 0x82C17838;
	sub_82556858(ctx, base);
	// 82C17838: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1783C: 896A0015  lbz r11, 0x15(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17844: 419A000C  beq cr6, 0x82c17850
	if ctx.cr[6].eq {
	pc = 0x82C17850; continue 'dispatch;
	}
	// 82C17848: 83BA0008  lwz r29, 8(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1784C: 4800002C  b 0x82c17878
	pc = 0x82C17878; continue 'dispatch;
	// 82C17850: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17854: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17858: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1785C: 419A000C  beq cr6, 0x82c17868
	if ctx.cr[6].eq {
	pc = 0x82C17868; continue 'dispatch;
	}
	// 82C17860: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82C17864: 48000014  b 0x82c17878
	pc = 0x82C17878; continue 'dispatch;
	// 82C17868: 81610104  lwz r11, 0x104(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82C1786C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17870: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17874: 409A00F0  bne cr6, 0x82c17964
	if !ctx.cr[6].eq {
	pc = 0x82C17964; continue 'dispatch;
	}
	// 82C17878: 897D0015  lbz r11, 0x15(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C1787C: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17884: 409A0008  bne cr6, 0x82c1788c
	if !ctx.cr[6].eq {
	pc = 0x82C1788C; continue 'dispatch;
	}
	// 82C17888: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C1788C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17890: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17894: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17898: 409A000C  bne cr6, 0x82c178a4
	if !ctx.cr[6].eq {
	pc = 0x82C178A4; continue 'dispatch;
	}
	// 82C1789C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C178A0: 4800001C  b 0x82c178bc
	pc = 0x82C178BC; continue 'dispatch;
	// 82C178A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C178A8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C178AC: 409A000C  bne cr6, 0x82c178b8
	if !ctx.cr[6].eq {
	pc = 0x82C178B8; continue 'dispatch;
	}
	// 82C178B0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C178B4: 48000008  b 0x82c178bc
	pc = 0x82C178BC; continue 'dispatch;
	// 82C178B8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C178BC: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C178C0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C178C4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C178C8: 409A0044  bne cr6, 0x82c1790c
	if !ctx.cr[6].eq {
	pc = 0x82C1790C; continue 'dispatch;
	}
	// 82C178CC: 897D0015  lbz r11, 0x15(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C178D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C178D4: 419A000C  beq cr6, 0x82c178e0
	if ctx.cr[6].eq {
	pc = 0x82C178E0; continue 'dispatch;
	}
	// 82C178D8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82C178DC: 4800002C  b 0x82c17908
	pc = 0x82C17908; continue 'dispatch;
	// 82C178E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C178E4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82C178E8: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C178EC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C178F0: 409A0018  bne cr6, 0x82c17908
	if !ctx.cr[6].eq {
	pc = 0x82C17908; continue 'dispatch;
	}
	// 82C178F4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C178F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C178FC: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17900: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17904: 419AFFF0  beq cr6, 0x82c178f4
	if ctx.cr[6].eq {
	pc = 0x82C178F4; continue 'dispatch;
	}
	// 82C17908: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1790C: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17910: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17914: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17918: 409A00E0  bne cr6, 0x82c179f8
	if !ctx.cr[6].eq {
	pc = 0x82C179F8; continue 'dispatch;
	}
	// 82C1791C: 897D0015  lbz r11, 0x15(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17920: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17924: 419A0010  beq cr6, 0x82c17934
	if ctx.cr[6].eq {
	pc = 0x82C17934; continue 'dispatch;
	}
	// 82C17928: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82C1792C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C17930: 480000C8  b 0x82c179f8
	pc = 0x82C179F8; continue 'dispatch;
	// 82C17934: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17938: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82C1793C: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17940: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17944: 409A0018  bne cr6, 0x82c1795c
	if !ctx.cr[6].eq {
	pc = 0x82C1795C; continue 'dispatch;
	}
	// 82C17948: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C1794C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17950: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17954: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17958: 419AFFF0  beq cr6, 0x82c17948
	if ctx.cr[6].eq {
	pc = 0x82C17948; continue 'dispatch;
	}
	// 82C1795C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C17960: 48000098  b 0x82c179f8
	pc = 0x82C179F8; continue 'dispatch;
	// 82C17964: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C17968: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1796C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C17970: 813A0008  lwz r9, 8(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17974: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C17978: 409A000C  bne cr6, 0x82c17984
	if !ctx.cr[6].eq {
	pc = 0x82C17984; continue 'dispatch;
	}
	// 82C1797C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C17980: 4800002C  b 0x82c179ac
	pc = 0x82C179AC; continue 'dispatch;
	// 82C17984: 895D0015  lbz r10, 0x15(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17988: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1798C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17990: 409A0008  bne cr6, 0x82c17998
	if !ctx.cr[6].eq {
	pc = 0x82C17998; continue 'dispatch;
	}
	// 82C17994: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C17998: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C1799C: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C179A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C179A4: 813A0008  lwz r9, 8(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C179A8: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C179AC: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C179B0: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C179B4: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C179B8: 409A000C  bne cr6, 0x82c179c4
	if !ctx.cr[6].eq {
	pc = 0x82C179C4; continue 'dispatch;
	}
	// 82C179BC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C179C0: 48000020  b 0x82c179e0
	pc = 0x82C179E0; continue 'dispatch;
	// 82C179C4: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C179C8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C179CC: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C179D0: 409A000C  bne cr6, 0x82c179dc
	if !ctx.cr[6].eq {
	pc = 0x82C179DC; continue 'dispatch;
	}
	// 82C179D4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C179D8: 48000008  b 0x82c179e0
	pc = 0x82C179E0; continue 'dispatch;
	// 82C179DC: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C179E0: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C179E4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C179E8: 890B0014  lbz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C179EC: 893A0014  lbz r9, 0x14(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C179F0: 992B0014  stb r9, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 82C179F4: 991A0014  stb r8, 0x14(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82C179F8: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C179FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82C17A00: 409A0194  bne cr6, 0x82c17b94
	if !ctx.cr[6].eq {
	pc = 0x82C17B94; continue 'dispatch;
	}
	// 82C17A04: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17A08: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C17A0C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17A10: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C17A14: 419A017C  beq cr6, 0x82c17b90
	if ctx.cr[6].eq {
	pc = 0x82C17B90; continue 'dispatch;
	}
	// 82C17A18: 897D0014  lbz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17A1C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82C17A20: 409A0170  bne cr6, 0x82c17b90
	if !ctx.cr[6].eq {
	pc = 0x82C17B90; continue 'dispatch;
	}
	// 82C17A24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17A28: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C17A2C: 409A00A8  bne cr6, 0x82c17ad4
	if !ctx.cr[6].eq {
	pc = 0x82C17AD4; continue 'dispatch;
	}
	// 82C17A30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17A34: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17A38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17A3C: 409A001C  bne cr6, 0x82c17a58
	if !ctx.cr[6].eq {
	pc = 0x82C17A58; continue 'dispatch;
	}
	// 82C17A40: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17A44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17A48: 9B7F0014  stb r27, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82C17A4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17A50: 4BDA0B11  bl 0x829b8560
	ctx.lr = 0x82C17A54;
	sub_829B8560(ctx, base);
	// 82C17A54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17A58: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17A5C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17A60: 409A00C8  bne cr6, 0x82c17b28
	if !ctx.cr[6].eq {
	pc = 0x82C17B28; continue 'dispatch;
	}
	// 82C17A64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17A68: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17A6C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17A70: 409A0014  bne cr6, 0x82c17a84
	if !ctx.cr[6].eq {
	pc = 0x82C17A84; continue 'dispatch;
	}
	// 82C17A74: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17A78: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17A7C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17A80: 419A00A4  beq cr6, 0x82c17b24
	if ctx.cr[6].eq {
	pc = 0x82C17B24; continue 'dispatch;
	}
	// 82C17A84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17A88: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17A8C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17A90: 409A0020  bne cr6, 0x82c17ab0
	if !ctx.cr[6].eq {
	pc = 0x82C17AB0; continue 'dispatch;
	}
	// 82C17A94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17A98: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C17A9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17AA0: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17AA4: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82C17AA8: 4B6F1361  bl 0x82308e08
	ctx.lr = 0x82C17AAC;
	sub_82308E08(ctx, base);
	// 82C17AAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17AB0: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17AB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17AB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17ABC: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82C17AC0: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17AC4: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17AC8: 9BC90014  stb r30, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17ACC: 4BDA0A95  bl 0x829b8560
	ctx.lr = 0x82C17AD0;
	sub_829B8560(ctx, base);
	// 82C17AD0: 480000C0  b 0x82c17b90
	pc = 0x82C17B90; continue 'dispatch;
	// 82C17AD4: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17AD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17ADC: 409A001C  bne cr6, 0x82c17af8
	if !ctx.cr[6].eq {
	pc = 0x82C17AF8; continue 'dispatch;
	}
	// 82C17AE0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17AE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17AE8: 9B7F0014  stb r27, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82C17AEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17AF0: 4B6F1319  bl 0x82308e08
	ctx.lr = 0x82C17AF4;
	sub_82308E08(ctx, base);
	// 82C17AF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17AF8: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C17AFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17B00: 409A0028  bne cr6, 0x82c17b28
	if !ctx.cr[6].eq {
	pc = 0x82C17B28; continue 'dispatch;
	}
	// 82C17B04: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17B08: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17B0C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17B10: 409A0034  bne cr6, 0x82c17b44
	if !ctx.cr[6].eq {
	pc = 0x82C17B44; continue 'dispatch;
	}
	// 82C17B14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17B18: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17B1C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17B20: 409A0024  bne cr6, 0x82c17b44
	if !ctx.cr[6].eq {
	pc = 0x82C17B44; continue 'dispatch;
	}
	// 82C17B24: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82C17B28: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17B2C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82C17B30: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17B34: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17B38: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C17B3C: 409AFEDC  bne cr6, 0x82c17a18
	if !ctx.cr[6].eq {
	pc = 0x82C17A18; continue 'dispatch;
	}
	// 82C17B40: 48000050  b 0x82c17b90
	pc = 0x82C17B90; continue 'dispatch;
	// 82C17B44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17B48: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17B4C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17B50: 409A0020  bne cr6, 0x82c17b70
	if !ctx.cr[6].eq {
	pc = 0x82C17B70; continue 'dispatch;
	}
	// 82C17B54: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17B58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C17B5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17B60: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17B64: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82C17B68: 4BDA09F9  bl 0x829b8560
	ctx.lr = 0x82C17B6C;
	sub_829B8560(ctx, base);
	// 82C17B6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17B70: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C17B74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17B78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17B7C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82C17B80: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17B84: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17B88: 9BC90014  stb r30, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17B8C: 4B6F127D  bl 0x82308e08
	ctx.lr = 0x82C17B90;
	sub_82308E08(ctx, base);
	// 82C17B90: 9BDD0014  stb r30, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82C17B94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C17B98: 4BC2DC19  bl 0x828457b0
	ctx.lr = 0x82C17B9C;
	sub_828457B0(ctx, base);
	// 82C17B9C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17BA0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C17BA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17BA8: 419A001C  beq cr6, 0x82c17bc4
	if ctx.cr[6].eq {
	pc = 0x82C17BC4; continue 'dispatch;
	}
	// 82C17BAC: E9410100  ld r10, 0x100(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) };
	// 82C17BB0: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82C17BB4: 913C0008  stw r9, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C17BB8: F9590000  std r10, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82C17BBC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C17BC0: 4809188C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C17BC4: E9610100  ld r11, 0x100(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) };
	// 82C17BC8: F9790000  std r11, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C17BCC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C17BD0: 4809187C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C17BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C17BD8 size=1068
    let mut pc: u32 = 0x82C17BD8;
    'dispatch: loop {
        match pc {
            0x82C17BD8 => {
    //   block [0x82C17BD8..0x82C18004)
	// 82C17BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C17BDC: 48091821  bl 0x82ca93fc
	ctx.lr = 0x82C17BE0;
	sub_82CA93D0(ctx, base);
	// 82C17BE0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C17BE4: F8A10100  std r5, 0x100(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[5].u64 ) };
	// 82C17BE8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82C17BEC: 83E10104  lwz r31, 0x104(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82C17BF0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C17BF4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C17BF8: 897F0029  lbz r11, 0x29(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17BFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17C00: 419A005C  beq cr6, 0x82c17c5c
	if ctx.cr[6].eq {
	pc = 0x82C17C5C; continue 'dispatch;
	}
	// 82C17C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C17C08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C17C0C: 388B16C0  addi r4, r11, 0x16c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5824;
	// 82C17C10: 4B6DA331  bl 0x822f1f40
	ctx.lr = 0x82C17C14;
	sub_822F1F40(ctx, base);
	// 82C17C14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C17C18: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C17C1C: 4B6DA155  bl 0x822f1d70
	ctx.lr = 0x82C17C20;
	sub_822F1D70(ctx, base);
	// 82C17C20: 4B6DA201  bl 0x822f1e20
	ctx.lr = 0x82C17C24;
	sub_822F1E20(ctx, base);
	// 82C17C24: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82C17C28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C17C2C: 392A1720  addi r9, r10, 0x1720
	ctx.r[9].s64 = ctx.r[10].s64 + 5920;
	// 82C17C30: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82C17C34: 4BA63BAD  bl 0x8267b7e0
	ctx.lr = 0x82C17C38;
	sub_8267B7E0(ctx, base);
	// 82C17C38: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C17C3C: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82C17C40: 4198000C  blt cr6, 0x82c17c4c
	if ctx.cr[6].lt {
	pc = 0x82C17C4C; continue 'dispatch;
	}
	// 82C17C44: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C17C48: 4BC2DB69  bl 0x828457b0
	ctx.lr = 0x82C17C4C;
	sub_828457B0(ctx, base);
	// 82C17C4C: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82C17C50: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82C17C54: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82C17C58: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82C17C5C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82C17C60: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82C17C64: 4BFFA595  bl 0x82c121f8
	ctx.lr = 0x82C17C68;
	sub_82C121F8(ctx, base);
	// 82C17C68: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17C6C: 896A0029  lbz r11, 0x29(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17C70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17C74: 419A000C  beq cr6, 0x82c17c80
	if ctx.cr[6].eq {
	pc = 0x82C17C80; continue 'dispatch;
	}
	// 82C17C78: 83BA0008  lwz r29, 8(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17C7C: 4800002C  b 0x82c17ca8
	pc = 0x82C17CA8; continue 'dispatch;
	// 82C17C80: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17C84: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17C88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C17C8C: 419A000C  beq cr6, 0x82c17c98
	if ctx.cr[6].eq {
	pc = 0x82C17C98; continue 'dispatch;
	}
	// 82C17C90: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82C17C94: 48000014  b 0x82c17ca8
	pc = 0x82C17CA8; continue 'dispatch;
	// 82C17C98: 81610104  lwz r11, 0x104(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82C17C9C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17CA0: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17CA4: 409A00F0  bne cr6, 0x82c17d94
	if !ctx.cr[6].eq {
	pc = 0x82C17D94; continue 'dispatch;
	}
	// 82C17CA8: 897D0029  lbz r11, 0x29(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17CAC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17CB4: 409A0008  bne cr6, 0x82c17cbc
	if !ctx.cr[6].eq {
	pc = 0x82C17CBC; continue 'dispatch;
	}
	// 82C17CB8: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C17CBC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17CC0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17CC4: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17CC8: 409A000C  bne cr6, 0x82c17cd4
	if !ctx.cr[6].eq {
	pc = 0x82C17CD4; continue 'dispatch;
	}
	// 82C17CCC: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C17CD0: 4800001C  b 0x82c17cec
	pc = 0x82C17CEC; continue 'dispatch;
	// 82C17CD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17CD8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17CDC: 409A000C  bne cr6, 0x82c17ce8
	if !ctx.cr[6].eq {
	pc = 0x82C17CE8; continue 'dispatch;
	}
	// 82C17CE0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C17CE4: 48000008  b 0x82c17cec
	pc = 0x82C17CEC; continue 'dispatch;
	// 82C17CE8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C17CEC: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17CF0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17CF4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17CF8: 409A0044  bne cr6, 0x82c17d3c
	if !ctx.cr[6].eq {
	pc = 0x82C17D3C; continue 'dispatch;
	}
	// 82C17CFC: 897D0029  lbz r11, 0x29(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17D00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17D04: 419A000C  beq cr6, 0x82c17d10
	if ctx.cr[6].eq {
	pc = 0x82C17D10; continue 'dispatch;
	}
	// 82C17D08: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82C17D0C: 4800002C  b 0x82c17d38
	pc = 0x82C17D38; continue 'dispatch;
	// 82C17D10: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17D14: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82C17D18: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17D1C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17D20: 409A0018  bne cr6, 0x82c17d38
	if !ctx.cr[6].eq {
	pc = 0x82C17D38; continue 'dispatch;
	}
	// 82C17D24: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C17D28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17D2C: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17D30: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17D34: 419AFFF0  beq cr6, 0x82c17d24
	if ctx.cr[6].eq {
	pc = 0x82C17D24; continue 'dispatch;
	}
	// 82C17D38: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C17D3C: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17D40: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17D44: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17D48: 409A00E0  bne cr6, 0x82c17e28
	if !ctx.cr[6].eq {
	pc = 0x82C17E28; continue 'dispatch;
	}
	// 82C17D4C: 897D0029  lbz r11, 0x29(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17D50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17D54: 419A0010  beq cr6, 0x82c17d64
	if ctx.cr[6].eq {
	pc = 0x82C17D64; continue 'dispatch;
	}
	// 82C17D58: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82C17D5C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C17D60: 480000C8  b 0x82c17e28
	pc = 0x82C17E28; continue 'dispatch;
	// 82C17D64: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17D68: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82C17D6C: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17D70: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17D74: 409A0018  bne cr6, 0x82c17d8c
	if !ctx.cr[6].eq {
	pc = 0x82C17D8C; continue 'dispatch;
	}
	// 82C17D78: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C17D7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17D80: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17D84: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C17D88: 419AFFF0  beq cr6, 0x82c17d78
	if ctx.cr[6].eq {
	pc = 0x82C17D78; continue 'dispatch;
	}
	// 82C17D8C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C17D90: 48000098  b 0x82c17e28
	pc = 0x82C17E28; continue 'dispatch;
	// 82C17D94: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C17D98: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17D9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C17DA0: 813A0008  lwz r9, 8(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17DA4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C17DA8: 409A000C  bne cr6, 0x82c17db4
	if !ctx.cr[6].eq {
	pc = 0x82C17DB4; continue 'dispatch;
	}
	// 82C17DAC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C17DB0: 4800002C  b 0x82c17ddc
	pc = 0x82C17DDC; continue 'dispatch;
	// 82C17DB4: 895D0029  lbz r10, 0x29(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17DB8: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17DBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17DC0: 409A0008  bne cr6, 0x82c17dc8
	if !ctx.cr[6].eq {
	pc = 0x82C17DC8; continue 'dispatch;
	}
	// 82C17DC4: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C17DC8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C17DCC: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17DD0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C17DD4: 813A0008  lwz r9, 8(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17DD8: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C17DDC: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17DE0: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17DE4: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17DE8: 409A000C  bne cr6, 0x82c17df4
	if !ctx.cr[6].eq {
	pc = 0x82C17DF4; continue 'dispatch;
	}
	// 82C17DEC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C17DF0: 48000020  b 0x82c17e10
	pc = 0x82C17E10; continue 'dispatch;
	// 82C17DF4: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17DF8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17DFC: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C17E00: 409A000C  bne cr6, 0x82c17e0c
	if !ctx.cr[6].eq {
	pc = 0x82C17E0C; continue 'dispatch;
	}
	// 82C17E04: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C17E08: 48000008  b 0x82c17e10
	pc = 0x82C17E10; continue 'dispatch;
	// 82C17E0C: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C17E10: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17E14: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C17E18: 890B0028  lbz r8, 0x28(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17E1C: 893A0028  lbz r9, 0x28(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17E20: 992B0028  stb r9, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 82C17E24: 991A0028  stb r8, 0x28(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82C17E28: 897A0028  lbz r11, 0x28(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17E2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82C17E30: 409A0194  bne cr6, 0x82c17fc4
	if !ctx.cr[6].eq {
	pc = 0x82C17FC4; continue 'dispatch;
	}
	// 82C17E34: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17E38: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C17E3C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17E40: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C17E44: 419A017C  beq cr6, 0x82c17fc0
	if ctx.cr[6].eq {
	pc = 0x82C17FC0; continue 'dispatch;
	}
	// 82C17E48: 897D0028  lbz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17E4C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82C17E50: 409A0170  bne cr6, 0x82c17fc0
	if !ctx.cr[6].eq {
	pc = 0x82C17FC0; continue 'dispatch;
	}
	// 82C17E54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17E58: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C17E5C: 409A00A8  bne cr6, 0x82c17f04
	if !ctx.cr[6].eq {
	pc = 0x82C17F04; continue 'dispatch;
	}
	// 82C17E60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17E64: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17E68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17E6C: 409A001C  bne cr6, 0x82c17e88
	if !ctx.cr[6].eq {
	pc = 0x82C17E88; continue 'dispatch;
	}
	// 82C17E70: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17E74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17E78: 9B7F0028  stb r27, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82C17E7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17E80: 4BFFB459  bl 0x82c132d8
	ctx.lr = 0x82C17E84;
	sub_82C132D8(ctx, base);
	// 82C17E84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17E88: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17E8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17E90: 409A00C8  bne cr6, 0x82c17f58
	if !ctx.cr[6].eq {
	pc = 0x82C17F58; continue 'dispatch;
	}
	// 82C17E94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17E98: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17E9C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17EA0: 409A0014  bne cr6, 0x82c17eb4
	if !ctx.cr[6].eq {
	pc = 0x82C17EB4; continue 'dispatch;
	}
	// 82C17EA4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17EA8: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17EAC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17EB0: 419A00A4  beq cr6, 0x82c17f54
	if ctx.cr[6].eq {
	pc = 0x82C17F54; continue 'dispatch;
	}
	// 82C17EB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17EB8: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17EBC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17EC0: 409A0020  bne cr6, 0x82c17ee0
	if !ctx.cr[6].eq {
	pc = 0x82C17EE0; continue 'dispatch;
	}
	// 82C17EC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17EC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C17ECC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17ED0: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17ED4: 9B6B0028  stb r27, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82C17ED8: 480237E9  bl 0x82c3b6c0
	ctx.lr = 0x82C17EDC;
	sub_82C3B6C0(ctx, base);
	// 82C17EDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17EE0: 895F0028  lbz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17EE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17EE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17EEC: 994B0028  stb r10, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82C17EF0: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17EF4: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17EF8: 9BC90028  stb r30, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17EFC: 4BFFB3DD  bl 0x82c132d8
	ctx.lr = 0x82C17F00;
	sub_82C132D8(ctx, base);
	// 82C17F00: 480000C0  b 0x82c17fc0
	pc = 0x82C17FC0; continue 'dispatch;
	// 82C17F04: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17F08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17F0C: 409A001C  bne cr6, 0x82c17f28
	if !ctx.cr[6].eq {
	pc = 0x82C17F28; continue 'dispatch;
	}
	// 82C17F10: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17F14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17F18: 9B7F0028  stb r27, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82C17F1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17F20: 480237A1  bl 0x82c3b6c0
	ctx.lr = 0x82C17F24;
	sub_82C3B6C0(ctx, base);
	// 82C17F24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17F28: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C17F2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C17F30: 409A0028  bne cr6, 0x82c17f58
	if !ctx.cr[6].eq {
	pc = 0x82C17F58; continue 'dispatch;
	}
	// 82C17F34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17F38: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17F3C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17F40: 409A0034  bne cr6, 0x82c17f74
	if !ctx.cr[6].eq {
	pc = 0x82C17F74; continue 'dispatch;
	}
	// 82C17F44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17F48: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17F4C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17F50: 409A0024  bne cr6, 0x82c17f74
	if !ctx.cr[6].eq {
	pc = 0x82C17F74; continue 'dispatch;
	}
	// 82C17F54: 9B6B0028  stb r27, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82C17F58: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17F5C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82C17F60: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17F64: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C17F68: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C17F6C: 409AFEDC  bne cr6, 0x82c17e48
	if !ctx.cr[6].eq {
	pc = 0x82C17E48; continue 'dispatch;
	}
	// 82C17F70: 48000050  b 0x82c17fc0
	pc = 0x82C17FC0; continue 'dispatch;
	// 82C17F74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17F78: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17F7C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82C17F80: 409A0020  bne cr6, 0x82c17fa0
	if !ctx.cr[6].eq {
	pc = 0x82C17FA0; continue 'dispatch;
	}
	// 82C17F84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17F88: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C17F8C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17F90: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17F94: 9B6B0028  stb r27, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 82C17F98: 4BFFB341  bl 0x82c132d8
	ctx.lr = 0x82C17F9C;
	sub_82C132D8(ctx, base);
	// 82C17F9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17FA0: 895F0028  lbz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C17FA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C17FA8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C17FAC: 994B0028  stb r10, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82C17FB0: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17FB4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C17FB8: 9BC90028  stb r30, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17FBC: 48023705  bl 0x82c3b6c0
	ctx.lr = 0x82C17FC0;
	sub_82C3B6C0(ctx, base);
	// 82C17FC0: 9BDD0028  stb r30, 0x28(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C17FC4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C17FC8: 4BC2D7E9  bl 0x828457b0
	ctx.lr = 0x82C17FCC;
	sub_828457B0(ctx, base);
	// 82C17FCC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C17FD0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C17FD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C17FD8: 419A001C  beq cr6, 0x82c17ff4
	if ctx.cr[6].eq {
	pc = 0x82C17FF4; continue 'dispatch;
	}
	// 82C17FDC: E9410100  ld r10, 0x100(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) };
	// 82C17FE0: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82C17FE4: 913C0008  stw r9, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C17FE8: F9590000  std r10, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82C17FEC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C17FF0: 4809145C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C17FF4: E9610100  ld r11, 0x100(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) };
	// 82C17FF8: F9790000  std r11, 0(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C17FFC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C18000: 4809144C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C18008 size=852
    let mut pc: u32 = 0x82C18008;
    'dispatch: loop {
        match pc {
            0x82C18008 => {
    //   block [0x82C18008..0x82C1835C)
	// 82C18008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1800C: 480913ED  bl 0x82ca93f8
	ctx.lr = 0x82C18010;
	sub_82CA93D0(ctx, base);
	// 82C18010: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C18014: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C18018: 3D600924  lis r11, 0x924
	ctx.r[11].s64 = 153354240;
	// 82C1801C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82C18020: 61699248  ori r9, r11, 0x9248
	ctx.r[9].u64 = ctx.r[11].u64 | 37448;
	// 82C18024: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C18028: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1802C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82C18030: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82C18034: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82C18038: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1803C: 4198005C  blt cr6, 0x82c18098
	if ctx.cr[6].lt {
	pc = 0x82C18098; continue 'dispatch;
	}
	// 82C18040: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C18044: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18048: 388B1690  addi r4, r11, 0x1690
	ctx.r[4].s64 = ctx.r[11].s64 + 5776;
	// 82C1804C: 4B6D9EF5  bl 0x822f1f40
	ctx.lr = 0x82C18050;
	sub_822F1F40(ctx, base);
	// 82C18050: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C18054: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C18058: 4B6D9D19  bl 0x822f1d70
	ctx.lr = 0x82C1805C;
	sub_822F1D70(ctx, base);
	// 82C1805C: 4B6D9DC5  bl 0x822f1e20
	ctx.lr = 0x82C18060;
	sub_822F1E20(ctx, base);
	// 82C18060: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82C18064: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C18068: 392A1720  addi r9, r10, 0x1720
	ctx.r[9].s64 = ctx.r[10].s64 + 5920;
	// 82C1806C: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82C18070: 4BA63771  bl 0x8267b7e0
	ctx.lr = 0x82C18074;
	sub_8267B7E0(ctx, base);
	// 82C18074: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C18078: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82C1807C: 4198000C  blt cr6, 0x82c18088
	if ctx.cr[6].lt {
	pc = 0x82C18088; continue 'dispatch;
	}
	// 82C18080: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18084: 4BC2D72D  bl 0x828457b0
	ctx.lr = 0x82C18088;
	sub_828457B0(ctx, base);
	// 82C18088: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82C1808C: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82C18090: 9B210054  stb r25, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u8 ) };
	// 82C18094: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82C18098: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82C1809C: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C180A0: 4B6071B9  bl 0x8221f258
	ctx.lr = 0x82C180A4;
	sub_8221F258(ctx, base);
	// 82C180A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C180A8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82C180AC: 419A003C  beq cr6, 0x82c180e8
	if ctx.cr[6].eq {
	pc = 0x82C180E8; continue 'dispatch;
	}
	// 82C180B0: 395A000C  addi r10, r26, 0xc
	ctx.r[10].s64 = ctx.r[26].s64 + 12;
	// 82C180B4: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C180B8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82C180BC: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C180C0: 93DA0008  stw r30, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C180C4: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82C180C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C180CC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C180D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C180D4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C180D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C180DC: 4200FFF0  bdnz 0x82c180cc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C180CC; continue 'dispatch;
	}
	// 82C180E0: 9B3A0028  stb r25, 0x28(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(40 as u32), ctx.r[25].u8 ) };
	// 82C180E4: 9B3A0029  stb r25, 0x29(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(41 as u32), ctx.r[25].u8 ) };
	// 82C180E8: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C180EC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C180F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C180F4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C180F8: 915B0008  stw r10, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C180FC: 409A001C  bne cr6, 0x82c18118
	if !ctx.cr[6].eq {
	pc = 0x82C18118; continue 'dispatch;
	}
	// 82C18100: 934B0004  stw r26, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82C18104: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18108: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82C1810C: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18110: 934A0008  stw r26, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82C18114: 48000044  b 0x82c18158
	pc = 0x82C18158; continue 'dispatch;
	// 82C18118: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82C1811C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18120: 419A0020  beq cr6, 0x82c18140
	if ctx.cr[6].eq {
	pc = 0x82C18140; continue 'dispatch;
	}
	// 82C18124: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82C18128: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1812C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18130: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18134: 409A0024  bne cr6, 0x82c18158
	if !ctx.cr[6].eq {
	pc = 0x82C18158; continue 'dispatch;
	}
	// 82C18138: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82C1813C: 4800001C  b 0x82c18158
	pc = 0x82C18158; continue 'dispatch;
	// 82C18140: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82C18144: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18148: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1814C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18150: 409A0008  bne cr6, 0x82c18158
	if !ctx.cr[6].eq {
	pc = 0x82C18158; continue 'dispatch;
	}
	// 82C18154: 934B0008  stw r26, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82C18158: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1815C: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82C18160: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C18164: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82C18168: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C1816C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C18170: 409A01CC  bne cr6, 0x82c1833c
	if !ctx.cr[6].eq {
	pc = 0x82C1833C; continue 'dispatch;
	}
	// 82C18174: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18178: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1817C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18180: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C18184: 409A00D8  bne cr6, 0x82c1825c
	if !ctx.cr[6].eq {
	pc = 0x82C1825C; continue 'dispatch;
	}
	// 82C18188: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1818C: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C18190: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C18194: 409A0028  bne cr6, 0x82c181bc
	if !ctx.cr[6].eq {
	pc = 0x82C181BC; continue 'dispatch;
	}
	// 82C18198: 5489003E  slwi r9, r4, 0
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C1819C: 9BC90028  stb r30, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C181A0: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C181A4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C181A8: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181AC: 9B270028  stb r25, 0x28(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), ctx.r[25].u8 ) };
	// 82C181B0: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C181B4: 83E60004  lwz r31, 4(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181B8: 48000170  b 0x82c18328
	pc = 0x82C18328; continue 'dispatch;
	// 82C181BC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C181C0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C181C4: 409A0010  bne cr6, 0x82c181d4
	if !ctx.cr[6].eq {
	pc = 0x82C181D4; continue 'dispatch;
	}
	// 82C181C8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C181CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C181D0: 4BFFB109  bl 0x82c132d8
	ctx.lr = 0x82C181D4;
	sub_82C132D8(ctx, base);
	// 82C181D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181D8: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C181DC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181E0: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181E4: 9B290028  stb r25, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[25].u8 ) };
	// 82C181E8: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181EC: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C181F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C181F4: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C181F8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C181FC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C18200: 88C90029  lbz r6, 0x29(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C18204: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C18208: 409A0008  bne cr6, 0x82c18210
	if !ctx.cr[6].eq {
	pc = 0x82C18210; continue 'dispatch;
	}
	// 82C1820C: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C18210: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18214: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C18218: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1821C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18220: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C18224: 409A0010  bne cr6, 0x82c18234
	if !ctx.cr[6].eq {
	pc = 0x82C18234; continue 'dispatch;
	}
	// 82C18228: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C1822C: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C18230: 480000F4  b 0x82c18324
	pc = 0x82C18324; continue 'dispatch;
	// 82C18234: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18238: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1823C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C18240: 409A0010  bne cr6, 0x82c18250
	if !ctx.cr[6].eq {
	pc = 0x82C18250; continue 'dispatch;
	}
	// 82C18244: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C18248: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1824C: 480000D8  b 0x82c18324
	pc = 0x82C18324; continue 'dispatch;
	// 82C18250: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C18254: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C18258: 480000CC  b 0x82c18324
	pc = 0x82C18324; continue 'dispatch;
	// 82C1825C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18260: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C18264: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C18268: 409A0028  bne cr6, 0x82c18290
	if !ctx.cr[6].eq {
	pc = 0x82C18290; continue 'dispatch;
	}
	// 82C1826C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18270: 9BC90028  stb r30, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C18274: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C18278: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1827C: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18280: 9B270028  stb r25, 0x28(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), ctx.r[25].u8 ) };
	// 82C18284: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18288: 83E60004  lwz r31, 4(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1828C: 4800009C  b 0x82c18328
	pc = 0x82C18328; continue 'dispatch;
	// 82C18290: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18294: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C18298: 409A0010  bne cr6, 0x82c182a8
	if !ctx.cr[6].eq {
	pc = 0x82C182A8; continue 'dispatch;
	}
	// 82C1829C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C182A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C182A4: 4802341D  bl 0x82c3b6c0
	ctx.lr = 0x82C182A8;
	sub_82C3B6C0(ctx, base);
	// 82C182A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182AC: 9BCB0028  stb r30, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C182B0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182B4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182B8: 9B290028  stb r25, 0x28(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[25].u8 ) };
	// 82C182BC: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182C0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182C4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C182C8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C182CC: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C182D0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C182D4: 88C90029  lbz r6, 0x29(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C182D8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C182DC: 409A0008  bne cr6, 0x82c182e4
	if !ctx.cr[6].eq {
	pc = 0x82C182E4; continue 'dispatch;
	}
	// 82C182E0: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C182E4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182E8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C182EC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182F0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C182F4: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C182F8: 409A000C  bne cr6, 0x82c18304
	if !ctx.cr[6].eq {
	pc = 0x82C18304; continue 'dispatch;
	}
	// 82C182FC: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C18300: 48000020  b 0x82c18320
	pc = 0x82C18320; continue 'dispatch;
	// 82C18304: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18308: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1830C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C18310: 409A000C  bne cr6, 0x82c1831c
	if !ctx.cr[6].eq {
	pc = 0x82C1831C; continue 'dispatch;
	}
	// 82C18314: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C18318: 48000008  b 0x82c18320
	pc = 0x82C18320; continue 'dispatch;
	// 82C1831C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C18320: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C18324: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C18328: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1832C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82C18330: 892A0028  lbz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C18334: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C18338: 419AFE3C  beq cr6, 0x82c18174
	if ctx.cr[6].eq {
	pc = 0x82C18174; continue 'dispatch;
	}
	// 82C1833C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18340: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C18344: 93580004  stw r26, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82C18348: 93780000  stw r27, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C1834C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18350: 9BCA0028  stb r30, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82C18354: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C18358: 480910F0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C18360 size=232
    let mut pc: u32 = 0x82C18360;
    'dispatch: loop {
        match pc {
            0x82C18360 => {
    //   block [0x82C18360..0x82C18448)
	// 82C18360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C18364: 480910A1  bl 0x82ca9404
	ctx.lr = 0x82C18368;
	sub_82CA93D0(ctx, base);
	// 82C18368: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1836C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C18370: F8A100B0  std r5, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u64 ) };
	// 82C18374: 814100B0  lwz r10, 0xb0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82C18378: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C1837C: F8C100B8  std r6, 0xb8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[6].u64 ) };
	// 82C18380: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C18384: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18388: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1838C: 419A000C  beq cr6, 0x82c18398
	if ctx.cr[6].eq {
	pc = 0x82C18398; continue 'dispatch;
	}
	// 82C18390: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C18394: 419A0008  beq cr6, 0x82c1839c
	if ctx.cr[6].eq {
	pc = 0x82C1839C; continue 'dispatch;
	}
	// 82C18398: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1839C: 810100B4  lwz r8, 0xb4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82C183A0: 83A100BC  lwz r29, 0xbc(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 82C183A4: 83C100B8  lwz r30, 0xb8(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82C183A8: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C183AC: 409A0044  bne cr6, 0x82c183f0
	if !ctx.cr[6].eq {
	pc = 0x82C183F0; continue 'dispatch;
	}
	// 82C183B0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C183B4: 419A000C  beq cr6, 0x82c183c0
	if ctx.cr[6].eq {
	pc = 0x82C183C0; continue 'dispatch;
	}
	// 82C183B8: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C183BC: 419A0008  beq cr6, 0x82c183c4
	if ctx.cr[6].eq {
	pc = 0x82C183C4; continue 'dispatch;
	}
	// 82C183C0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C183C4: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C183C8: 409A0028  bne cr6, 0x82c183f0
	if !ctx.cr[6].eq {
	pc = 0x82C183F0; continue 'dispatch;
	}
	// 82C183CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C183D0: 4BFFD939  bl 0x82c15d08
	ctx.lr = 0x82C183D4;
	sub_82C15D08(ctx, base);
	// 82C183D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C183D8: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C183DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C183E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C183E4: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C183E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C183EC: 48091068  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C183F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C183F4: 419A000C  beq cr6, 0x82c18400
	if ctx.cr[6].eq {
	pc = 0x82C18400; continue 'dispatch;
	}
	// 82C183F8: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C183FC: 419A0008  beq cr6, 0x82c18404
	if ctx.cr[6].eq {
	pc = 0x82C18404; continue 'dispatch;
	}
	// 82C18400: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18404: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82C18408: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C1840C: 419A002C  beq cr6, 0x82c18438
	if ctx.cr[6].eq {
	pc = 0x82C18438; continue 'dispatch;
	}
	// 82C18410: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82C18414: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C18418: 4BFF9DE1  bl 0x82c121f8
	ctx.lr = 0x82C1841C;
	sub_82C121F8(ctx, base);
	// 82C1841C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C18420: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C18424: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18428: 4BFFF7B1  bl 0x82c17bd8
	ctx.lr = 0x82C1842C;
	sub_82C17BD8(ctx, base);
	// 82C1842C: E8A100B0  ld r5, 0xb0(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) };
	// 82C18430: 814100B0  lwz r10, 0xb0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82C18434: 4BFFFFBC  b 0x82c183f0
	pc = 0x82C183F0; continue 'dispatch;
	// 82C18438: F8BC0000  std r5, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[5].u64 ) };
	// 82C1843C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C18440: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18444: 48091010  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C18448 size=316
    let mut pc: u32 = 0x82C18448;
    'dispatch: loop {
        match pc {
            0x82C18448 => {
    //   block [0x82C18448..0x82C18584)
	// 82C18448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1844C: 48090FB5  bl 0x82ca9400
	ctx.lr = 0x82C18450;
	sub_82CA93D0(ctx, base);
	// 82C18450: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C18454: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C18458: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82C1845C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C18460: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C18464: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82C18468: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1846C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18470: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C18474: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C18478: 409A003C  bne cr6, 0x82c184b4
	if !ctx.cr[6].eq {
	pc = 0x82C184B4; continue 'dispatch;
	}
	// 82C1847C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18480: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18484: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C18488: 7D095010  subfc r8, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C1848C: 7CE84110  subfe r7, r8, r8
	let x = (!ctx.r[8].u32);
	let y = ctx.r[8].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[7].u32 = res;
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C18490: 54FD07FE  clrlwi r29, r7, 0x1f
	ctx.r[29].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 82C18494: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C18498: 419A000C  beq cr6, 0x82c184a4
	if ctx.cr[6].eq {
	pc = 0x82C184A4; continue 'dispatch;
	}
	// 82C1849C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C184A0: 48000008  b 0x82c184a8
	pc = 0x82C184A8; continue 'dispatch;
	// 82C184A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C184A8: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C184AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C184B0: 419AFFD0  beq cr6, 0x82c18480
	if ctx.cr[6].eq {
	pc = 0x82C18480; continue 'dispatch;
	}
	// 82C184B4: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82C184B8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C184BC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82C184C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C184C4: 419A0054  beq cr6, 0x82c18518
	if ctx.cr[6].eq {
	pc = 0x82C18518; continue 'dispatch;
	}
	// 82C184C8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C184CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C184D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C184D4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C184D8: 409A003C  bne cr6, 0x82c18514
	if !ctx.cr[6].eq {
	pc = 0x82C18514; continue 'dispatch;
	}
	// 82C184DC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C184E0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82C184E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C184E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C184EC: 4BFFFB1D  bl 0x82c18008
	ctx.lr = 0x82C184F0;
	sub_82C18008(ctx, base);
	// 82C184F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C184F4: 9B5E0008  stb r26, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 82C184F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C184FC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18500: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18504: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C18508: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C1850C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18510: 48090F40  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C18514: 4BFF9C1D  bl 0x82c12130
	ctx.lr = 0x82C18518;
	sub_82C12130(ctx, base);
	// 82C18518: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C1851C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18520: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18524: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18528: 40980040  bge cr6, 0x82c18568
	if !ctx.cr[6].lt {
	pc = 0x82C18568; continue 'dispatch;
	}
	// 82C1852C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C18530: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82C18534: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C18538: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C1853C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18540: 4BFFFAC9  bl 0x82c18008
	ctx.lr = 0x82C18544;
	sub_82C18008(ctx, base);
	// 82C18544: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C18548: 9B5E0008  stb r26, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 82C1854C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18550: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18554: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18558: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1855C: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C18560: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18564: 48090EEC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C18568: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C1856C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C18570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18574: 995E0008  stb r10, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82C18578: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C1857C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18580: 48090ED0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C18588 size=524
    let mut pc: u32 = 0x82C18588;
    'dispatch: loop {
        match pc {
            0x82C18588 => {
    //   block [0x82C18588..0x82C18794)
	// 82C18588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1858C: 48090E79  bl 0x82ca9404
	ctx.lr = 0x82C18590;
	sub_82CA93D0(ctx, base);
	// 82C18590: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C18594: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C18598: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C1859C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C185A0: FB6100B0  std r27, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[27].u64 ) };
	// 82C185A4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C185A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C185AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C185B0: 409A0020  bne cr6, 0x82c185d0
	if !ctx.cr[6].eq {
	pc = 0x82C185D0; continue 'dispatch;
	}
	// 82C185B4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C185B8: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C185BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C185C0: 4BFFFA49  bl 0x82c18008
	ctx.lr = 0x82C185C4;
	sub_82C18008(ctx, base);
	// 82C185C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C185C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C185CC: 48090E88  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C185D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C185D4: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82C185D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C185DC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C185E0: 419A000C  beq cr6, 0x82c185ec
	if ctx.cr[6].eq {
	pc = 0x82C185EC; continue 'dispatch;
	}
	// 82C185E4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C185E8: 419A0008  beq cr6, 0x82c185f0
	if ctx.cr[6].eq {
	pc = 0x82C185F0; continue 'dispatch;
	}
	// 82C185EC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C185F0: 838100B4  lwz r28, 0xb4(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82C185F4: 7F1C4840  cmplw cr6, r28, r9
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C185F8: 409A0038  bne cr6, 0x82c18630
	if !ctx.cr[6].eq {
	pc = 0x82C18630; continue 'dispatch;
	}
	// 82C185FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18600: 815C000C  lwz r10, 0xc(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18604: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18608: 4098015C  bge cr6, 0x82c18764
	if !ctx.cr[6].lt {
	pc = 0x82C18764; continue 'dispatch;
	}
	// 82C1860C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C18610: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C18614: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C18618: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1861C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18620: 4BFFF9E9  bl 0x82c18008
	ctx.lr = 0x82C18624;
	sub_82C18008(ctx, base);
	// 82C18624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18628: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1862C: 48090E28  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C18630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18634: 419A000C  beq cr6, 0x82c18640
	if ctx.cr[6].eq {
	pc = 0x82C18640; continue 'dispatch;
	}
	// 82C18638: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C1863C: 419A0008  beq cr6, 0x82c18644
	if ctx.cr[6].eq {
	pc = 0x82C18644; continue 'dispatch;
	}
	// 82C18640: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18644: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18648: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C1864C: 409A0034  bne cr6, 0x82c18680
	if !ctx.cr[6].eq {
	pc = 0x82C18680; continue 'dispatch;
	}
	// 82C18650: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C18654: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18658: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C1865C: 40980108  bge cr6, 0x82c18764
	if !ctx.cr[6].lt {
	pc = 0x82C18764; continue 'dispatch;
	}
	// 82C18660: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C18664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C18668: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1866C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18670: 4BFFF999  bl 0x82c18008
	ctx.lr = 0x82C18674;
	sub_82C18008(ctx, base);
	// 82C18674: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18678: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1867C: 48090DD8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C18680: 815C000C  lwz r10, 0xc(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18684: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18688: 40980058  bge cr6, 0x82c186e0
	if !ctx.cr[6].lt {
	pc = 0x82C186E0; continue 'dispatch;
	}
	// 82C1868C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18690: FB610050  std r27, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u64 ) };
	// 82C18694: 4BFF9A9D  bl 0x82c12130
	ctx.lr = 0x82C18698;
	sub_82C12130(ctx, base);
	// 82C18698: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C1869C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C186A0: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C186A4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C186A8: 40980038  bge cr6, 0x82c186e0
	if !ctx.cr[6].lt {
	pc = 0x82C186E0; continue 'dispatch;
	}
	// 82C186AC: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C186B0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C186B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C186B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C186BC: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C186C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C186C4: 409A008C  bne cr6, 0x82c18750
	if !ctx.cr[6].eq {
	pc = 0x82C18750; continue 'dispatch;
	}
	// 82C186C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C186CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C186D0: 4BFFF939  bl 0x82c18008
	ctx.lr = 0x82C186D4;
	sub_82C18008(ctx, base);
	// 82C186D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C186D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C186DC: 48090D78  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C186E0: 815C000C  lwz r10, 0xc(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C186E4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C186E8: 4098007C  bge cr6, 0x82c18764
	if !ctx.cr[6].lt {
	pc = 0x82C18764; continue 'dispatch;
	}
	// 82C186EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C186F0: FB610050  std r27, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u64 ) };
	// 82C186F4: 837F0004  lwz r27, 4(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C186F8: 4BFF9B01  bl 0x82c121f8
	ctx.lr = 0x82C186FC;
	sub_82C121F8(ctx, base);
	// 82C186FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C18700: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18704: 419A000C  beq cr6, 0x82c18710
	if ctx.cr[6].eq {
	pc = 0x82C18710; continue 'dispatch;
	}
	// 82C18708: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C1870C: 419A0008  beq cr6, 0x82c18714
	if ctx.cr[6].eq {
	pc = 0x82C18714; continue 'dispatch;
	}
	// 82C18710: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18714: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18718: 7F06D840  cmplw cr6, r6, r27
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C1871C: 419A0014  beq cr6, 0x82c18730
	if ctx.cr[6].eq {
	pc = 0x82C18730; continue 'dispatch;
	}
	// 82C18720: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18724: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18728: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C1872C: 40980038  bge cr6, 0x82c18764
	if !ctx.cr[6].lt {
	pc = 0x82C18764; continue 'dispatch;
	}
	// 82C18730: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C18734: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C18738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1873C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C18740: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C18744: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C18748: 419AFF84  beq cr6, 0x82c186cc
	if ctx.cr[6].eq {
	pc = 0x82C186CC; continue 'dispatch;
	}
	// 82C1874C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C18750: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C18754: 4BFFF8B5  bl 0x82c18008
	ctx.lr = 0x82C18758;
	sub_82C18008(ctx, base);
	// 82C18758: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1875C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18760: 48090CF4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C18764: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C18768: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1876C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18770: 4BFFFCD9  bl 0x82c18448
	ctx.lr = 0x82C18774;
	sub_82C18448(ctx, base);
	// 82C18774: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C18778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1877C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18780: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C18784: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18788: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C1878C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18790: 48090CC4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C18798 size=260
    let mut pc: u32 = 0x82C18798;
    'dispatch: loop {
        match pc {
            0x82C18798 => {
    //   block [0x82C18798..0x82C1889C)
	// 82C18798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1879C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C187A0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C187A4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C187A8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C187AC: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C187B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C187B4: 409A0030  bne cr6, 0x82c187e4
	if !ctx.cr[6].eq {
	pc = 0x82C187E4; continue 'dispatch;
	}
	// 82C187B8: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C187BC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C187C0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C187C4: 4098000C  bge cr6, 0x82c187d0
	if !ctx.cr[6].lt {
	pc = 0x82C187D0; continue 'dispatch;
	}
	// 82C187C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C187CC: 4800000C  b 0x82c187d8
	pc = 0x82C187D8; continue 'dispatch;
	// 82C187D0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C187D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C187D8: 890B0029  lbz r8, 0x29(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C187DC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C187E0: 419AFFDC  beq cr6, 0x82c187bc
	if ctx.cr[6].eq {
	pc = 0x82C187BC; continue 'dispatch;
	}
	// 82C187E4: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C187E8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82C187EC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C187F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C187F4: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C187F8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C187FC: 419A0014  beq cr6, 0x82c18810
	if ctx.cr[6].eq {
	pc = 0x82C18810; continue 'dispatch;
	}
	// 82C18800: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18804: 80EA000C  lwz r7, 0xc(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C18808: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C1880C: 40980060  bge cr6, 0x82c1886c
	if !ctx.cr[6].lt {
	pc = 0x82C1886C; continue 'dispatch;
	}
	// 82C18810: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18814: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C18818: 39410084  addi r10, r1, 0x84
	ctx.r[10].s64 = ctx.r[1].s64 + 132;
	// 82C1881C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82C18820: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82C18824: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82C18828: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82C1882C: 91010080  stw r8, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 82C18830: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C18834: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18838: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C1883C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C18840: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C18844: 4200FFF0  bdnz 0x82c18834
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C18834; continue 'dispatch;
	}
	// 82C18848: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1884C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C18850: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82C18854: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18858: 4BFFFD31  bl 0x82c18588
	ctx.lr = 0x82C1885C;
	sub_82C18588(ctx, base);
	// 82C1885C: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82C18860: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82C18864: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18868: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C1886C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C18870: 409A0008  bne cr6, 0x82c18878
	if !ctx.cr[6].eq {
	pc = 0x82C18878; continue 'dispatch;
	}
	// 82C18874: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18878: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1887C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18880: 409A0008  bne cr6, 0x82c18888
	if !ctx.cr[6].eq {
	pc = 0x82C18888; continue 'dispatch;
	}
	// 82C18884: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18888: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82C1888C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C18890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C18894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C18898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C188A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C188A0 size=252
    let mut pc: u32 = 0x82C188A0;
    'dispatch: loop {
        match pc {
            0x82C188A0 => {
    //   block [0x82C188A0..0x82C1899C)
	// 82C188A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C188A4: 48090B69  bl 0x82ca940c
	ctx.lr = 0x82C188A8;
	sub_82CA93D0(ctx, base);
	// 82C188A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C188AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C188B0: 4B895DB1  bl 0x824ae660
	ctx.lr = 0x82C188B4;
	sub_824AE660(ctx, base);
	// 82C188B4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C188B8: 4B895DA9  bl 0x824ae660
	ctx.lr = 0x82C188BC;
	sub_824AE660(ctx, base);
	// 82C188BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C188C0: 4B895DA1  bl 0x824ae660
	ctx.lr = 0x82C188C4;
	sub_824AE660(ctx, base);
	// 82C188C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C188C8: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 82C188CC: 392AAC40  addi r9, r10, -0x53c0
	ctx.r[9].s64 = ctx.r[10].s64 + -21440;
	// 82C188D0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C188D4: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82C188D8: 480351D1  bl 0x82c4daa8
	ctx.lr = 0x82C188DC;
	sub_82C4DAA8(ctx, base);
	// 82C188DC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C188E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C188E4: 38E8AC40  addi r7, r8, -0x53c0
	ctx.r[7].s64 = ctx.r[8].s64 + -21440;
	// 82C188E8: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82C188EC: 90FF0028  stw r7, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82C188F0: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82C188F4: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C188F8: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82C188FC: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82C18900: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82C18904: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82C18908: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82C1890C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82C18910: 4BFFE4B1  bl 0x82c16dc0
	ctx.lr = 0x82C18914;
	sub_82C16DC0(ctx, base);
	// 82C18914: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82C18918: 48037A89  bl 0x82c503a0
	ctx.lr = 0x82C1891C;
	sub_82C503A0(ctx, base);
	// 82C1891C: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C18920: 397F023C  addi r11, r31, 0x23c
	ctx.r[11].s64 = ctx.r[31].s64 + 572;
	// 82C18924: 38A6AC68  addi r5, r6, -0x5398
	ctx.r[5].s64 = ctx.r[6].s64 + -21400;
	// 82C18928: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1892C: 90BF023C  stw r5, 0x23c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(572 as u32), ctx.r[5].u32 ) };
	// 82C18930: 48035179  bl 0x82c4daa8
	ctx.lr = 0x82C18934;
	sub_82C4DAA8(ctx, base);
	// 82C18934: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82C18938: 3C608200  lis r3, -0x7e00
	ctx.r[3].s64 = -2113929216;
	// 82C1893C: 3964AC68  addi r11, r4, -0x5398
	ctx.r[11].s64 = ctx.r[4].s64 + -21400;
	// 82C18940: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82C18944: 917F023C  stw r11, 0x23c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(572 as u32), ctx.r[11].u32 ) };
	// 82C18948: 3BBF028C  addi r29, r31, 0x28c
	ctx.r[29].s64 = ctx.r[31].s64 + 652;
	// 82C1894C: 915F0284  stw r10, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 82C18950: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C18954: C0030C14  lfs f0, 0xc14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C18958: 93DF0288  stw r30, 0x288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), ctx.r[30].u32 ) };
	// 82C1895C: D01F0280  stfs f0, 0x280(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), tmp.u32 ) };
	// 82C18960: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C18964: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C18968: 4BF368D9  bl 0x82b4f240
	ctx.lr = 0x82C1896C;
	sub_82B4F240(ctx, base);
	// 82C1896C: 38DF0270  addi r6, r31, 0x270
	ctx.r[6].s64 = ctx.r[31].s64 + 624;
	// 82C18970: 38BF0260  addi r5, r31, 0x260
	ctx.r[5].s64 = ctx.r[31].s64 + 608;
	// 82C18974: 9BDF0310  stb r30, 0x310(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(784 as u32), ctx.r[30].u8 ) };
	// 82C18978: 389F0250  addi r4, r31, 0x250
	ctx.r[4].s64 = ctx.r[31].s64 + 592;
	// 82C1897C: 9BDF0311  stb r30, 0x311(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(785 as u32), ctx.r[30].u8 ) };
	// 82C18980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C18984: 4BFFA72D  bl 0x82c130b0
	ctx.lr = 0x82C18988;
	sub_82C130B0(ctx, base);
	// 82C18988: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1898C: 4BF36AFD  bl 0x82b4f488
	ctx.lr = 0x82C18990;
	sub_82B4F488(ctx, base);
	// 82C18990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C18994: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C18998: 48090AC4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C189A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C189A0 size=608
    let mut pc: u32 = 0x82C189A0;
    'dispatch: loop {
        match pc {
            0x82C189A0 => {
    //   block [0x82C189A0..0x82C18C00)
	// 82C189A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C189A4: 48090A55  bl 0x82ca93f8
	ctx.lr = 0x82C189A8;
	sub_82CA93D0(ctx, base);
	// 82C189A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C189AC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82C189B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C189B4: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82C189B8: 419A0240  beq cr6, 0x82c18bf8
	if ctx.cr[6].eq {
	pc = 0x82C18BF8; continue 'dispatch;
	}
	// 82C189BC: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C189C0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82C189C4: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82C189C8: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C189CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C189D0: 40990034  ble cr6, 0x82c18a04
	if !ctx.cr[6].gt {
	pc = 0x82C18A04; continue 'dispatch;
	}
	// 82C189D4: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82C189D8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C189DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C189E0: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82C189E4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82C189E8: 4BFFE119  bl 0x82c16b00
	ctx.lr = 0x82C189EC;
	sub_82C16B00(ctx, base);
	// 82C189EC: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C189F0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82C189F4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C189F8: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C189FC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18A00: 4198FFD8  blt cr6, 0x82c189d8
	if ctx.cr[6].lt {
	pc = 0x82C189D8; continue 'dispatch;
	}
	// 82C18A04: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C18A08: 3BFD000C  addi r31, r29, 0xc
	ctx.r[31].s64 = ctx.r[29].s64 + 12;
	// 82C18A0C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C18A10: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C18A14: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18A18: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82C18A1C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18A24: 419A000C  beq cr6, 0x82c18a30
	if ctx.cr[6].eq {
	pc = 0x82C18A30; continue 'dispatch;
	}
	// 82C18A28: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C18A2C: 419A0008  beq cr6, 0x82c18a34
	if ctx.cr[6].eq {
	pc = 0x82C18A34; continue 'dispatch;
	}
	// 82C18A30: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18A34: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18A38: 419A0064  beq cr6, 0x82c18a9c
	if ctx.cr[6].eq {
	pc = 0x82C18A9C; continue 'dispatch;
	}
	// 82C18A3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18A40: 409A0008  bne cr6, 0x82c18a48
	if !ctx.cr[6].eq {
	pc = 0x82C18A48; continue 'dispatch;
	}
	// 82C18A44: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18A48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18A4C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C18A50: 409A0008  bne cr6, 0x82c18a58
	if !ctx.cr[6].eq {
	pc = 0x82C18A58; continue 'dispatch;
	}
	// 82C18A54: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18A58: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C18A5C: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82C18A60: 409A0028  bne cr6, 0x82c18a88
	if !ctx.cr[6].eq {
	pc = 0x82C18A88; continue 'dispatch;
	}
	// 82C18A64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C18A68: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C18A6C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18A70: 4BFFED39  bl 0x82c177a8
	ctx.lr = 0x82C18A74;
	sub_82C177A8(ctx, base);
	// 82C18A74: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82C18A78: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C18A7C: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C18A80: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C18A84: 4BFFFF98  b 0x82c18a1c
	pc = 0x82C18A1C; continue 'dispatch;
	// 82C18A88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C18A8C: 4B93DDCD  bl 0x82556858
	ctx.lr = 0x82C18A90;
	sub_82556858(ctx, base);
	// 82C18A90: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C18A94: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C18A98: 4BFFFF84  b 0x82c18a1c
	pc = 0x82C18A1C; continue 'dispatch;
	// 82C18A9C: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C18AA0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 82C18AA4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C18AA8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C18AAC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18AB0: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82C18AB4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18ABC: 419A000C  beq cr6, 0x82c18ac8
	if ctx.cr[6].eq {
	pc = 0x82C18AC8; continue 'dispatch;
	}
	// 82C18AC0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C18AC4: 419A0008  beq cr6, 0x82c18acc
	if ctx.cr[6].eq {
	pc = 0x82C18ACC; continue 'dispatch;
	}
	// 82C18AC8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18ACC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18AD0: 419A0064  beq cr6, 0x82c18b34
	if ctx.cr[6].eq {
	pc = 0x82C18B34; continue 'dispatch;
	}
	// 82C18AD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18AD8: 409A0008  bne cr6, 0x82c18ae0
	if !ctx.cr[6].eq {
	pc = 0x82C18AE0; continue 'dispatch;
	}
	// 82C18ADC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18AE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18AE4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C18AE8: 409A0008  bne cr6, 0x82c18af0
	if !ctx.cr[6].eq {
	pc = 0x82C18AF0; continue 'dispatch;
	}
	// 82C18AEC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18AF0: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C18AF4: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82C18AF8: 409A0028  bne cr6, 0x82c18b20
	if !ctx.cr[6].eq {
	pc = 0x82C18B20; continue 'dispatch;
	}
	// 82C18AFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C18B00: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C18B04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18B08: 4BFFECA1  bl 0x82c177a8
	ctx.lr = 0x82C18B0C;
	sub_82C177A8(ctx, base);
	// 82C18B0C: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82C18B10: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C18B14: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C18B18: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C18B1C: 4BFFFF98  b 0x82c18ab4
	pc = 0x82C18AB4; continue 'dispatch;
	// 82C18B20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C18B24: 4B93DD35  bl 0x82556858
	ctx.lr = 0x82C18B28;
	sub_82556858(ctx, base);
	// 82C18B28: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C18B2C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C18B30: 4BFFFF84  b 0x82c18ab4
	pc = 0x82C18AB4; continue 'dispatch;
	// 82C18B34: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18B38: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82C18B3C: 814B00A0  lwz r10, 0xa0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82C18B40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C18B44: 4099009C  ble cr6, 0x82c18be0
	if !ctx.cr[6].gt {
	pc = 0x82C18BE0; continue 'dispatch;
	}
	// 82C18B48: 8B410050  lbz r26, 0x50(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C18B4C: 3BBD0054  addi r29, r29, 0x54
	ctx.r[29].s64 = ctx.r[29].s64 + 84;
	// 82C18B50: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82C18B54: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C18B58: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82C18B5C: 4B60F20D  bl 0x82227d68
	ctx.lr = 0x82C18B60;
	sub_82227D68(ctx, base);
	// 82C18B60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C18B64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C18B68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18B6C: 4B614365  bl 0x8222ced0
	ctx.lr = 0x82C18B70;
	sub_8222CED0(ctx, base);
	// 82C18B70: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18B74: 4B671E65  bl 0x8228a9d8
	ctx.lr = 0x82C18B78;
	sub_8228A9D8(ctx, base);
	// 82C18B78: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82C18B7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18B80: 4B5FC259  bl 0x82214dd8
	ctx.lr = 0x82C18B84;
	sub_82214DD8(ctx, base);
	// 82C18B84: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C18B88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C18B8C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C18B90: 4BFFD0D1  bl 0x82c15c60
	ctx.lr = 0x82C18B94;
	sub_82C15C60(ctx, base);
	// 82C18B94: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82C18B98: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82C18B9C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C18BA0: EB810078  ld r28, 0x78(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82C18BA4: EB610070  ld r27, 0x70(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82C18BA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C18BAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C18BB0: 4BFFBFE9  bl 0x82c14b98
	ctx.lr = 0x82C18BB4;
	sub_82C14B98(ctx, base);
	// 82C18BB4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82C18BB8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C18BBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C18BC0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C18BC4: 4BFFF79D  bl 0x82c18360
	ctx.lr = 0x82C18BC8;
	sub_82C18360(ctx, base);
	// 82C18BC8: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18BCC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82C18BD0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82C18BD4: 814B00A0  lwz r10, 0xa0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82C18BD8: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18BDC: 4198FF78  blt cr6, 0x82c18b54
	if ctx.cr[6].lt {
	pc = 0x82C18B54; continue 'dispatch;
	}
	// 82C18BE0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18BE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C18BE8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C18BEC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18BF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C18BF4: 4E800421  bctrl
	ctx.lr = 0x82C18BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C18BF8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C18BFC: 4809084C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C18C00 size=256
    let mut pc: u32 = 0x82C18C00;
    'dispatch: loop {
        match pc {
            0x82C18C00 => {
    //   block [0x82C18C00..0x82C18D00)
	// 82C18C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C18C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C18C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C18C0C: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82C18C10: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82C18C14: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82C18C18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C18C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C18C20: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82C18C24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C18C28: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 82C18C2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18C30: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82C18C34: 4B61429D  bl 0x8222ced0
	ctx.lr = 0x82C18C38;
	sub_8222CED0(ctx, base);
	// 82C18C38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18C3C: 4B671D9D  bl 0x8228a9d8
	ctx.lr = 0x82C18C40;
	sub_8228A9D8(ctx, base);
	// 82C18C40: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82C18C44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18C48: 4B5FC191  bl 0x82214dd8
	ctx.lr = 0x82C18C4C;
	sub_82214DD8(ctx, base);
	// 82C18C4C: 3BFF0054  addi r31, r31, 0x54
	ctx.r[31].s64 = ctx.r[31].s64 + 84;
	// 82C18C50: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82C18C54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C18C58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18C5C: 4BFFE0C5  bl 0x82c16d20
	ctx.lr = 0x82C18C60;
	sub_82C16D20(ctx, base);
	// 82C18C60: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82C18C64: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18C68: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82C18C6C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C18C70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18C74: 419A000C  beq cr6, 0x82c18c80
	if ctx.cr[6].eq {
	pc = 0x82C18C80; continue 'dispatch;
	}
	// 82C18C78: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C18C7C: 419A0008  beq cr6, 0x82c18c84
	if ctx.cr[6].eq {
	pc = 0x82C18C84; continue 'dispatch;
	}
	// 82C18C80: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18C84: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18C88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18C8C: 409A0054  bne cr6, 0x82c18ce0
	if !ctx.cr[6].eq {
	pc = 0x82C18CE0; continue 'dispatch;
	}
	// 82C18C90: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82C18C94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C18C98: 4BFFFB01  bl 0x82c18798
	ctx.lr = 0x82C18C9C;
	sub_82C18798(ctx, base);
	// 82C18C9C: D3E30000  stfs f31, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C18CA0: D3A30004  stfs f29, 4(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C18CA4: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82C18CA8: 41990010  bgt cr6, 0x82c18cb8
	if ctx.cr[6].gt {
	pc = 0x82C18CB8; continue 'dispatch;
	}
	// 82C18CAC: D3E30010  stfs f31, 0x10(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C18CB0: D3E30014  stfs f31, 0x14(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C18CB4: 48000020  b 0x82c18cd4
	pc = 0x82C18CD4; continue 'dispatch;
	// 82C18CB8: FF1EE800  fcmpu cr6, f30, f29
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[29].f64);
	// 82C18CBC: 41980010  blt cr6, 0x82c18ccc
	if ctx.cr[6].lt {
	pc = 0x82C18CCC; continue 'dispatch;
	}
	// 82C18CC0: D3A30010  stfs f29, 0x10(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C18CC4: D3A30014  stfs f29, 0x14(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C18CC8: 4800000C  b 0x82c18cd4
	pc = 0x82C18CD4; continue 'dispatch;
	// 82C18CCC: D3C30010  stfs f30, 0x10(r3)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C18CD0: D3C30014  stfs f30, 0x14(r3)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C18CD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C18CD8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C18CDC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C18CE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C18CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C18CEC: CBA1FFD8  lfd f29, -0x28(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C18CF0: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82C18CF4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C18CF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C18CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C18D00 size=652
    let mut pc: u32 = 0x82C18D00;
    'dispatch: loop {
        match pc {
            0x82C18D00 => {
    //   block [0x82C18D00..0x82C18F8C)
	// 82C18D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C18D04: 480906F9  bl 0x82ca93fc
	ctx.lr = 0x82C18D08;
	sub_82CA93D0(ctx, base);
	// 82C18D08: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82C18D0C: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82C18D10: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82C18D14: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C18D18: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C18D1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C18D20: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82C18D24: 93C100DC  stw r30, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 82C18D28: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82C18D2C: 4B60652D  bl 0x8221f258
	ctx.lr = 0x82C18D30;
	sub_8221F258(ctx, base);
	// 82C18D30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C18D34: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C18D38: 419A0044  beq cr6, 0x82c18d7c
	if ctx.cr[6].eq {
	pc = 0x82C18D7C; continue 'dispatch;
	}
	// 82C18D3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C18D40: 93630008  stw r27, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82C18D44: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C18D48: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82C18D4C: 394BAC38  addi r10, r11, -0x53c8
	ctx.r[10].s64 = ctx.r[11].s64 + -21448;
	// 82C18D50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C18D54: 9363000C  stw r27, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82C18D58: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 82C18D5C: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82C18D60: 93630018  stw r27, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 82C18D64: 9363001C  stw r27, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[27].u32 ) };
	// 82C18D68: 93630020  stw r27, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82C18D6C: 93630024  stw r27, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82C18D70: 813F00A4  lwz r9, 0xa4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82C18D74: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C18D78: 48000008  b 0x82c18d80
	pc = 0x82C18D80; continue 'dispatch;
	// 82C18D7C: 7F79DB78  mr r25, r27
	ctx.r[25].u64 = ctx.r[27].u64;
	// 82C18D80: 38A100DC  addi r5, r1, 0xdc
	ctx.r[5].s64 = ctx.r[1].s64 + 220;
	// 82C18D84: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C18D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18D8C: 4B75C6F5  bl 0x82375480
	ctx.lr = 0x82C18D90;
	sub_82375480(ctx, base);
	// 82C18D90: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C18D94: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18D98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C18D9C: 419A000C  beq cr6, 0x82c18da8
	if ctx.cr[6].eq {
	pc = 0x82C18DA8; continue 'dispatch;
	}
	// 82C18DA0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82C18DA4: 419A0008  beq cr6, 0x82c18dac
	if ctx.cr[6].eq {
	pc = 0x82C18DAC; continue 'dispatch;
	}
	// 82C18DA8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18DAC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18DB0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C18DB4: 409A015C  bne cr6, 0x82c18f10
	if !ctx.cr[6].eq {
	pc = 0x82C18F10; continue 'dispatch;
	}
	// 82C18DB8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82C18DBC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C18DC0: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82C18DC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C18DC8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C18DCC: 480708E5  bl 0x82c896b0
	ctx.lr = 0x82C18DD0;
	sub_82C896B0(ctx, base);
	// 82C18DD0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C18DD4: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82C18DD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18DDC: 40990058  ble cr6, 0x82c18e34
	if !ctx.cr[6].gt {
	pc = 0x82C18E34; continue 'dispatch;
	}
	// 82C18DE0: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82C18DE4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C18DE8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C18DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18DF0: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C18DF4: 38AB0004  addi r5, r11, 4
	ctx.r[5].s64 = ctx.r[11].s64 + 4;
	// 82C18DF8: 4BFFCD19  bl 0x82c15b10
	ctx.lr = 0x82C18DFC;
	sub_82C15B10(ctx, base);
	// 82C18DFC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18E00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C18E04: 419A0014  beq cr6, 0x82c18e18
	if ctx.cr[6].eq {
	pc = 0x82C18E18; continue 'dispatch;
	}
	// 82C18E08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18E0C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18E10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C18E14: 4E800421  bctrl
	ctx.lr = 0x82C18E18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C18E18: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C18E1C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C18E20: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82C18E24: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C18E28: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82C18E2C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C18E30: 4198FFB4  blt cr6, 0x82c18de4
	if ctx.cr[6].lt {
	pc = 0x82C18DE4; continue 'dispatch;
	}
	// 82C18E34: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C18E38: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82C18E3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18E40: 40990034  ble cr6, 0x82c18e74
	if !ctx.cr[6].gt {
	pc = 0x82C18E74; continue 'dispatch;
	}
	// 82C18E44: 3B9A000C  addi r28, r26, 0xc
	ctx.r[28].s64 = ctx.r[26].s64 + 12;
	// 82C18E48: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82C18E4C: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82C18E50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C18E54: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82C18E58: 48043F39  bl 0x82c5cd90
	ctx.lr = 0x82C18E5C;
	sub_82C5CD90(ctx, base);
	// 82C18E5C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C18E60: 93230000  stw r25, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C18E64: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C18E68: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C18E6C: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18E70: 4198FFDC  blt cr6, 0x82c18e4c
	if ctx.cr[6].lt {
	pc = 0x82C18E4C; continue 'dispatch;
	}
	// 82C18E74: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C18E78: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82C18E7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18E80: 40990034  ble cr6, 0x82c18eb4
	if !ctx.cr[6].gt {
	pc = 0x82C18EB4; continue 'dispatch;
	}
	// 82C18E84: 3B9A0018  addi r28, r26, 0x18
	ctx.r[28].s64 = ctx.r[26].s64 + 24;
	// 82C18E88: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82C18E8C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C18E90: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C18E94: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C18E98: 48043EF9  bl 0x82c5cd90
	ctx.lr = 0x82C18E9C;
	sub_82C5CD90(ctx, base);
	// 82C18E9C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C18EA0: 93230000  stw r25, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C18EA4: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C18EA8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C18EAC: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18EB0: 4198FFDC  blt cr6, 0x82c18e8c
	if ctx.cr[6].lt {
	pc = 0x82C18E8C; continue 'dispatch;
	}
	// 82C18EB4: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82C18EB8: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82C18EBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18EC0: 409900B4  ble cr6, 0x82c18f74
	if !ctx.cr[6].gt {
	pc = 0x82C18F74; continue 'dispatch;
	}
	// 82C18EC4: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82C18EC8: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C18ECC: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C18ED0: C3E3000C  lfs f31, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C18ED4: C3C30008  lfs f30, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82C18ED8: C3A30004  lfs f29, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82C18EDC: 4B60EE8D  bl 0x82227d68
	ctx.lr = 0x82C18EE0;
	sub_82227D68(ctx, base);
	// 82C18EE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C18EE4: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82C18EE8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C18EEC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82C18EF0: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82C18EF4: 4BFFFD0D  bl 0x82c18c00
	ctx.lr = 0x82C18EF8;
	sub_82C18C00(ctx, base);
	// 82C18EF8: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82C18EFC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82C18F00: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C18F04: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18F08: 4198FFC0  blt cr6, 0x82c18ec8
	if ctx.cr[6].lt {
	pc = 0x82C18EC8; continue 'dispatch;
	}
	// 82C18F0C: 48000068  b 0x82c18f74
	pc = 0x82C18F74; continue 'dispatch;
	// 82C18F10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C18F14: 409A0008  bne cr6, 0x82c18f1c
	if !ctx.cr[6].eq {
	pc = 0x82C18F1C; continue 'dispatch;
	}
	// 82C18F18: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18F1C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18F20: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C18F24: 409A0008  bne cr6, 0x82c18f2c
	if !ctx.cr[6].eq {
	pc = 0x82C18F2C; continue 'dispatch;
	}
	// 82C18F28: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18F2C: 388100DC  addi r4, r1, 0xdc
	ctx.r[4].s64 = ctx.r[1].s64 + 220;
	// 82C18F30: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C18F34: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C18F38: 48043E59  bl 0x82c5cd90
	ctx.lr = 0x82C18F3C;
	sub_82C5CD90(ctx, base);
	// 82C18F3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C18F40: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82C18F44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C18F48: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C18F4C: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C18F50: 4BFFD509  bl 0x82c16458
	ctx.lr = 0x82C18F54;
	sub_82C16458(ctx, base);
	// 82C18F54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C18F58: 419A001C  beq cr6, 0x82c18f74
	if ctx.cr[6].eq {
	pc = 0x82C18F74; continue 'dispatch;
	}
	// 82C18F5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18F60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C18F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C18F68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C18F6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C18F70: 4E800421  bctrl
	ctx.lr = 0x82C18F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C18F74: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C18F78: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C18F7C: CBA1FFA8  lfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82C18F80: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82C18F84: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82C18F88: 480904C4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C18F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C18F90 size=324
    let mut pc: u32 = 0x82C18F90;
    'dispatch: loop {
        match pc {
            0x82C18F90 => {
    //   block [0x82C18F90..0x82C190D4)
	// 82C18F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C18F94: 48090471  bl 0x82ca9404
	ctx.lr = 0x82C18F98;
	sub_82CA93D0(ctx, base);
	// 82C18F98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C18F9C: 908100AC  stw r4, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 82C18FA0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C18FA4: 38A100AC  addi r5, r1, 0xac
	ctx.r[5].s64 = ctx.r[1].s64 + 172;
	// 82C18FA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C18FAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C18FB0: 4B75C4D1  bl 0x82375480
	ctx.lr = 0x82C18FB4;
	sub_82375480(ctx, base);
	// 82C18FB4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C18FB8: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18FC0: 419A000C  beq cr6, 0x82c18fcc
	if ctx.cr[6].eq {
	pc = 0x82C18FCC; continue 'dispatch;
	}
	// 82C18FC4: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C18FC8: 419A0008  beq cr6, 0x82c18fd0
	if ctx.cr[6].eq {
	pc = 0x82C18FD0; continue 'dispatch;
	}
	// 82C18FCC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18FD0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C18FD4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C18FD8: 409A0010  bne cr6, 0x82c18fe8
	if !ctx.cr[6].eq {
	pc = 0x82C18FE8; continue 'dispatch;
	}
	// 82C18FDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C18FE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C18FE4: 48090470  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C18FE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C18FEC: 409A0008  bne cr6, 0x82c18ff4
	if !ctx.cr[6].eq {
	pc = 0x82C18FF4; continue 'dispatch;
	}
	// 82C18FF0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C18FF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C18FF8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C18FFC: 409A0008  bne cr6, 0x82c19004
	if !ctx.cr[6].eq {
	pc = 0x82C19004; continue 'dispatch;
	}
	// 82C19000: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C19004: 83AA0010  lwz r29, 0x10(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C19008: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C1900C: 419AFFD0  beq cr6, 0x82c18fdc
	if ctx.cr[6].eq {
	pc = 0x82C18FDC; continue 'dispatch;
	}
	// 82C19010: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C19014: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C19018: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1901C: 4BFF9F95  bl 0x82c12fb0
	ctx.lr = 0x82C19020;
	sub_82C12FB0(ctx, base);
	// 82C19020: 3BDC0034  addi r30, r28, 0x34
	ctx.r[30].s64 = ctx.r[28].s64 + 52;
	// 82C19024: 83FC0034  lwz r31, 0x34(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C19028: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1902C: 419A0060  beq cr6, 0x82c1908c
	if ctx.cr[6].eq {
	pc = 0x82C1908C; continue 'dispatch;
	}
	// 82C19030: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C19034: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C19038: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C1903C: 409A0024  bne cr6, 0x82c19060
	if !ctx.cr[6].eq {
	pc = 0x82C19060; continue 'dispatch;
	}
	// 82C19040: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C19048: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C1904C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19050: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C19054: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19058: 4E800421  bctrl
	ctx.lr = 0x82C1905C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1905C: 937F0080  stw r27, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[27].u32 ) };
	// 82C19060: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C19064: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C19068: 409A0024  bne cr6, 0x82c1908c
	if !ctx.cr[6].eq {
	pc = 0x82C1908C; continue 'dispatch;
	}
	// 82C1906C: 83FF0060  lwz r31, 0x60(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C19070: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C19074: 419A0018  beq cr6, 0x82c1908c
	if ctx.cr[6].eq {
	pc = 0x82C1908C; continue 'dispatch;
	}
	// 82C19078: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C1907C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C19080: 409A000C  bne cr6, 0x82c1908c
	if !ctx.cr[6].eq {
	pc = 0x82C1908C; continue 'dispatch;
	}
	// 82C19084: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C19088: 409AFFAC  bne cr6, 0x82c19034
	if !ctx.cr[6].eq {
	pc = 0x82C19034; continue 'dispatch;
	}
	// 82C1908C: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C19090: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C19094: 419A0018  beq cr6, 0x82c190ac
	if ctx.cr[6].eq {
	pc = 0x82C190AC; continue 'dispatch;
	}
	// 82C19098: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C1909C: 4BFFD675  bl 0x82c16710
	ctx.lr = 0x82C190A0;
	sub_82C16710(ctx, base);
	// 82C190A0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C190A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C190A8: 409AFFF0  bne cr6, 0x82c19098
	if !ctx.cr[6].eq {
	pc = 0x82C19098; continue 'dispatch;
	}
	// 82C190AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C190B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C190B4: 4BFFF8ED  bl 0x82c189a0
	ctx.lr = 0x82C190B8;
	sub_82C189A0(ctx, base);
	// 82C190B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C190BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C190C0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C190C4: 4BFFE6E5  bl 0x82c177a8
	ctx.lr = 0x82C190C8;
	sub_82C177A8(ctx, base);
	// 82C190C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C190CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C190D0: 48090384  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C190D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C190D8 size=8
    let mut pc: u32 = 0x82C190D8;
    'dispatch: loop {
        match pc {
            0x82C190D8 => {
    //   block [0x82C190D8..0x82C190E0)
	// 82C190D8: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C190DC: 4B64C0C4  b 0x822651a0
	sub_822651A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C190E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C190E0 size=28
    let mut pc: u32 = 0x82C190E0;
    'dispatch: loop {
        match pc {
            0x82C190E0 => {
    //   block [0x82C190E0..0x82C190FC)
	// 82C190E0: C0030040  lfs f0, 0x40(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C190E4: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C190E8: C1A30044  lfs f13, 0x44(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C190EC: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C190F0: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C190F4: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C190F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19100 size=28
    let mut pc: u32 = 0x82C19100;
    'dispatch: loop {
        match pc {
            0x82C19100 => {
    //   block [0x82C19100..0x82C1911C)
	// 82C19100: C0030030  lfs f0, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C19104: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C19108: C1A30034  lfs f13, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C1910C: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C19110: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C19114: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C19118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19120 size=20
    let mut pc: u32 = 0x82C19120;
    'dispatch: loop {
        match pc {
            0x82C19120 => {
    //   block [0x82C19120..0x82C19134)
	// 82C19120: C0040030  lfs f0, 0x30(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C19124: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C19128: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C1912C: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C19130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19138 size=20
    let mut pc: u32 = 0x82C19138;
    'dispatch: loop {
        match pc {
            0x82C19138 => {
    //   block [0x82C19138..0x82C1914C)
	// 82C19138: C0040040  lfs f0, 0x40(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1913C: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C19140: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C19144: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C19148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19150 size=80
    let mut pc: u32 = 0x82C19150;
    'dispatch: loop {
        match pc {
            0x82C19150 => {
    //   block [0x82C19150..0x82C191A0)
	// 82C19150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1915C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19164: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C19168: 4803A7E1  bl 0x82c53948
	ctx.lr = 0x82C1916C;
	sub_82C53948(ctx, base);
	// 82C1916C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C19170: 4803A8A9  bl 0x82c53a18
	ctx.lr = 0x82C19174;
	sub_82C53A18(ctx, base);
	// 82C19174: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19178: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1917C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19180: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19184: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19188: 4E800421  bctrl
	ctx.lr = 0x82C1918C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1918C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C19190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1919C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C191A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C191A0 size=28
    let mut pc: u32 = 0x82C191A0;
    'dispatch: loop {
        match pc {
            0x82C191A0 => {
    //   block [0x82C191A0..0x82C191BC)
	// 82C191A0: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C191A4: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C191A8: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C191AC: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C191B0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C191B4: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C191B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C191C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C191C0 size=8
    let mut pc: u32 = 0x82C191C0;
    'dispatch: loop {
        match pc {
            0x82C191C0 => {
    //   block [0x82C191C0..0x82C191C8)
	// 82C191C0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C191C4: 48039BAC  b 0x82c52d70
	sub_82C52D70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C191C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C191C8 size=8
    let mut pc: u32 = 0x82C191C8;
    'dispatch: loop {
        match pc {
            0x82C191C8 => {
    //   block [0x82C191C8..0x82C191D0)
	// 82C191C8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C191CC: 4803A4DC  b 0x82c536a8
	sub_82C536A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C191D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C191D0 size=8
    let mut pc: u32 = 0x82C191D0;
    'dispatch: loop {
        match pc {
            0x82C191D0 => {
    //   block [0x82C191D0..0x82C191D8)
	// 82C191D0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82C191D4: 4B60EB94  b 0x82227d68
	sub_82227D68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C191D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C191D8 size=92
    let mut pc: u32 = 0x82C191D8;
    'dispatch: loop {
        match pc {
            0x82C191D8 => {
    //   block [0x82C191D8..0x82C19234)
	// 82C191D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C191DC: 48090231  bl 0x82ca940c
	ctx.lr = 0x82C191E0;
	sub_82CA93D0(ctx, base);
	// 82C191E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C191E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C191E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C191EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C191F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C191F4: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C191F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C191FC: 4E800421  bctrl
	ctx.lr = 0x82C19200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19200: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82C19204: E95F00B0  ld r10, 0xb0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) };
	// 82C19208: 7BA9F862  rldicl r9, r29, 0x3f, 0x21
	ctx.r[9].u64 = ctx.r[29].u64 & 0x0000000000000001u64;
	// 82C1920C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82C19210: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C19214: 7CE95214  add r7, r9, r10
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82C19218: 911F0090  stw r8, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82C1921C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C19220: F8FF00B0  std r7, 0xb0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[7].u64 ) };
	// 82C19224: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C19228: 48039EA1  bl 0x82c530c8
	ctx.lr = 0x82C1922C;
	sub_82C530C8(ctx, base);
	// 82C1922C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19230: 4809022C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19238 size=8
    let mut pc: u32 = 0x82C19238;
    'dispatch: loop {
        match pc {
            0x82C19238 => {
    //   block [0x82C19238..0x82C19240)
	// 82C19238: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1923C: 48039E24  b 0x82c53060
	sub_82C53060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19240 size=256
    let mut pc: u32 = 0x82C19240;
    'dispatch: loop {
        match pc {
            0x82C19240 => {
    //   block [0x82C19240..0x82C19340)
	// 82C19240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C19244: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C19248: 814300C8  lwz r10, 0xc8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) } as u64;
	// 82C1924C: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C19250: 81230098  lwz r9, 0x98(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C19254: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19258: 8103009C  lwz r8, 0x9c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C1925C: 9104000C  stw r8, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82C19260: C0030084  lfs f0, 0x84(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C19264: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C19268: 88E300D0  lbz r7, 0xd0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82C1926C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C19270: 419A000C  beq cr6, 0x82c1927c
	if ctx.cr[6].eq {
	pc = 0x82C1927C; continue 'dispatch;
	}
	// 82C19274: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C19278: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1927C: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82C19280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C19284: 419A0010  beq cr6, 0x82c19294
	if ctx.cr[6].eq {
	pc = 0x82C19294; continue 'dispatch;
	}
	// 82C19288: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1928C: 616A0002  ori r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u64 | 2;
	// 82C19290: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19294: 896300D5  lbz r11, 0xd5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(213 as u32) ) } as u64;
	// 82C19298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1929C: 419A0010  beq cr6, 0x82c192ac
	if ctx.cr[6].eq {
	pc = 0x82C192AC; continue 'dispatch;
	}
	// 82C192A0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C192A4: 616A0004  ori r10, r11, 4
	ctx.r[10].u64 = ctx.r[11].u64 | 4;
	// 82C192A8: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C192AC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C192B0: 814300BC  lwz r10, 0xbc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 82C192B4: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C192B8: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C192BC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C192C0: 41980010  blt cr6, 0x82c192d0
	if ctx.cr[6].lt {
	pc = 0x82C192D0; continue 'dispatch;
	}
	// 82C192C4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C192C8: 61490008  ori r9, r10, 8
	ctx.r[9].u64 = ctx.r[10].u64 | 8;
	// 82C192CC: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C192D0: 894300D6  lbz r10, 0xd6(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(214 as u32) ) } as u64;
	// 82C192D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C192D8: 419A0010  beq cr6, 0x82c192e8
	if ctx.cr[6].eq {
	pc = 0x82C192E8; continue 'dispatch;
	}
	// 82C192DC: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C192E0: 61490010  ori r9, r10, 0x10
	ctx.r[9].u64 = ctx.r[10].u64 | 16;
	// 82C192E4: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C192E8: 894300D7  lbz r10, 0xd7(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(215 as u32) ) } as u64;
	// 82C192EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C192F0: 419A0010  beq cr6, 0x82c19300
	if ctx.cr[6].eq {
	pc = 0x82C19300; continue 'dispatch;
	}
	// 82C192F4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C192F8: 61490020  ori r9, r10, 0x20
	ctx.r[9].u64 = ctx.r[10].u64 | 32;
	// 82C192FC: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19300: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C19304: 814300C0  lwz r10, 0xc0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82C19308: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C1930C: 41980010  blt cr6, 0x82c1931c
	if ctx.cr[6].lt {
	pc = 0x82C1931C; continue 'dispatch;
	}
	// 82C19310: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19314: 616A0040  ori r10, r11, 0x40
	ctx.r[10].u64 = ctx.r[11].u64 | 64;
	// 82C19318: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1931C: 896300D2  lbz r11, 0xd2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(210 as u32) ) } as u64;
	// 82C19320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C19324: 419A0010  beq cr6, 0x82c19334
	if ctx.cr[6].eq {
	pc = 0x82C19334; continue 'dispatch;
	}
	// 82C19328: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1932C: 616A0080  ori r10, r11, 0x80
	ctx.r[10].u64 = ctx.r[11].u64 | 128;
	// 82C19330: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19334: 896300D3  lbz r11, 0xd3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(211 as u32) ) } as u64;
	// 82C19338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1933C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19340 size=16
    let mut pc: u32 = 0x82C19340;
    'dispatch: loop {
        match pc {
            0x82C19340 => {
    //   block [0x82C19340..0x82C19350)
	// 82C19340: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19344: 616A0100  ori r10, r11, 0x100
	ctx.r[10].u64 = ctx.r[11].u64 | 256;
	// 82C19348: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1934C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19350 size=52
    let mut pc: u32 = 0x82C19350;
    'dispatch: loop {
        match pc {
            0x82C19350 => {
    //   block [0x82C19350..0x82C19384)
	// 82C19350: E96300A8  ld r11, 0xa8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) };
	// 82C19354: F9640000  std r11, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C19358: E94300A0  ld r10, 0xa0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) };
	// 82C1935C: F9440008  std r10, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82C19360: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C19364: 9124001C  stw r9, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82C19368: 81030090  lwz r8, 0x90(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82C1936C: 91040018  stw r8, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82C19370: E8E300B0  ld r7, 0xb0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) };
	// 82C19374: F8E40010  std r7, 0x10(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[7].u64 ) };
	// 82C19378: 80C300B8  lwz r6, 0xb8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82C1937C: 90C40020  stw r6, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82C19380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19388 size=64
    let mut pc: u32 = 0x82C19388;
    'dispatch: loop {
        match pc {
            0x82C19388 => {
    //   block [0x82C19388..0x82C193C8)
	// 82C19388: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82C1938C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C19390: 419A002C  beq cr6, 0x82c193bc
	if ctx.cr[6].eq {
	pc = 0x82C193BC; continue 'dispatch;
	}
	// 82C19394: 896300D5  lbz r11, 0xd5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(213 as u32) ) } as u64;
	// 82C19398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1939C: 419A0020  beq cr6, 0x82c193bc
	if ctx.cr[6].eq {
	pc = 0x82C193BC; continue 'dispatch;
	}
	// 82C193A0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C193A4: 814300BC  lwz r10, 0xbc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 82C193A8: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C193AC: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C193B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C193B4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C193B8: 40980008  bge cr6, 0x82c193c0
	if !ctx.cr[6].lt {
	pc = 0x82C193C0; continue 'dispatch;
	}
	// 82C193BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C193C0: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C193C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C193C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C193C8 size=96
    let mut pc: u32 = 0x82C193C8;
    'dispatch: loop {
        match pc {
            0x82C193C8 => {
    //   block [0x82C193C8..0x82C19428)
	// 82C193C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C193CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C193D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C193D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C193D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C193DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C193E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C193E4: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82C193E8: 895F00D4  lbz r10, 0xd4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82C193EC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C193F0: 419A0020  beq cr6, 0x82c19410
	if ctx.cr[6].eq {
	pc = 0x82C19410; continue 'dispatch;
	}
	// 82C193F4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C193F8: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C193FC: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19400: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C19404: 915F00BC  stw r10, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 82C19408: 48039B99  bl 0x82c52fa0
	ctx.lr = 0x82C1940C;
	sub_82C52FA0(ctx, base);
	// 82C1940C: 9BDF00D4  stb r30, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u8 ) };
	// 82C19410: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1941C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19428 size=64
    let mut pc: u32 = 0x82C19428;
    'dispatch: loop {
        match pc {
            0x82C19428 => {
    //   block [0x82C19428..0x82C19468)
	// 82C19428: 896300D6  lbz r11, 0xd6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(214 as u32) ) } as u64;
	// 82C1942C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C19430: 419A002C  beq cr6, 0x82c1945c
	if ctx.cr[6].eq {
	pc = 0x82C1945C; continue 'dispatch;
	}
	// 82C19434: 896300D7  lbz r11, 0xd7(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(215 as u32) ) } as u64;
	// 82C19438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1943C: 419A0020  beq cr6, 0x82c1945c
	if ctx.cr[6].eq {
	pc = 0x82C1945C; continue 'dispatch;
	}
	// 82C19440: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C19444: 814300C0  lwz r10, 0xc0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82C19448: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1944C: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C19450: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C19454: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C19458: 40980008  bge cr6, 0x82c19460
	if !ctx.cr[6].lt {
	pc = 0x82C19460; continue 'dispatch;
	}
	// 82C1945C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C19460: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C19464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19468 size=96
    let mut pc: u32 = 0x82C19468;
    'dispatch: loop {
        match pc {
            0x82C19468 => {
    //   block [0x82C19468..0x82C194C8)
	// 82C19468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C19474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1947C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19480: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C19484: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82C19488: 895F00D6  lbz r10, 0xd6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(214 as u32) ) } as u64;
	// 82C1948C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C19490: 419A0020  beq cr6, 0x82c194b0
	if ctx.cr[6].eq {
	pc = 0x82C194B0; continue 'dispatch;
	}
	// 82C19494: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C19498: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1949C: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C194A0: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C194A4: 915F00C0  stw r10, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 82C194A8: 48039B59  bl 0x82c53000
	ctx.lr = 0x82C194AC;
	sub_82C53000(ctx, base);
	// 82C194AC: 9BDF00D6  stb r30, 0xd6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[30].u8 ) };
	// 82C194B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C194B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C194B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C194BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C194C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C194C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C194C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C194C8 size=12
    let mut pc: u32 = 0x82C194C8;
    'dispatch: loop {
        match pc {
            0x82C194C8 => {
    //   block [0x82C194C8..0x82C194D4)
	// 82C194C8: 90830098  stw r4, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[4].u32 ) };
	// 82C194CC: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C194D0: 4BF7D9A8  b 0x82b96e78
	sub_82B96E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C194D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C194D8 size=12
    let mut pc: u32 = 0x82C194D8;
    'dispatch: loop {
        match pc {
            0x82C194D8 => {
    //   block [0x82C194D8..0x82C194E4)
	// 82C194D8: 9083009C  stw r4, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C194DC: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C194E0: 4BF7D998  b 0x82b96e78
	sub_82B96E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C194E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C194E8 size=4
    let mut pc: u32 = 0x82C194E8;
    'dispatch: loop {
        match pc {
            0x82C194E8 => {
    //   block [0x82C194E8..0x82C194EC)
	// 82C194E8: 48039638  b 0x82c52b20
	sub_82C52B20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C194F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C194F0 size=4
    let mut pc: u32 = 0x82C194F0;
    'dispatch: loop {
        match pc {
            0x82C194F0 => {
    //   block [0x82C194F0..0x82C194F4)
	// 82C194F0: 48039E08  b 0x82c532f8
	sub_82C532F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C194F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C194F8 size=8
    let mut pc: u32 = 0x82C194F8;
    'dispatch: loop {
        match pc {
            0x82C194F8 => {
    //   block [0x82C194F8..0x82C19500)
	// 82C194F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C194FC: 4803A304  b 0x82c53800
	sub_82C53800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19500 size=12
    let mut pc: u32 = 0x82C19500;
    'dispatch: loop {
        match pc {
            0x82C19500 => {
    //   block [0x82C19500..0x82C1950C)
	// 82C19500: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C19504: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C19508: 4803A370  b 0x82c53878
	sub_82C53878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19510 size=8
    let mut pc: u32 = 0x82C19510;
    'dispatch: loop {
        match pc {
            0x82C19510 => {
    //   block [0x82C19510..0x82C19518)
	// 82C19510: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C19514: 4803A49C  b 0x82c539b0
	sub_82C539B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19518 size=12
    let mut pc: u32 = 0x82C19518;
    'dispatch: loop {
        match pc {
            0x82C19518 => {
    //   block [0x82C19518..0x82C19524)
	// 82C19518: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C1951C: 806B4EF4  lwz r3, 0x4ef4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19528 size=24
    let mut pc: u32 = 0x82C19528;
    'dispatch: loop {
        match pc {
            0x82C19528 => {
    //   block [0x82C19528..0x82C19540)
	// 82C19528: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C1952C: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19530: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82C19534: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82C19538: 69230001  xori r3, r9, 1
	ctx.r[3].u64 = ctx.r[9].u64 ^ 1;
	// 82C1953C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19540 size=16
    let mut pc: u32 = 0x82C19540;
    'dispatch: loop {
        match pc {
            0x82C19540 => {
    //   block [0x82C19540..0x82C19550)
	// 82C19540: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C19544: 39040014  addi r8, r4, 0x14
	ctx.r[8].s64 = ctx.r[4].s64 + 20;
	// 82C19548: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C1954C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19550 size=52
    let mut pc: u32 = 0x82C19550;
    'dispatch: loop {
        match pc {
            0x82C19550 => {
    //   block [0x82C19550..0x82C19584)
	// 82C19550: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C19558: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1955C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C19560: 409A003C  bne cr6, 0x82c1959c
	if !ctx.cr[6].eq {
		sub_82C1959C(ctx, base);
		return;
	}
	// 82C19564: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19568: 409A001C  bne cr6, 0x82c19584
	if !ctx.cr[6].eq {
		sub_82C19584(ctx, base);
		return;
	}
	// 82C1956C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19570: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19574: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19578: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1957C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19584 size=24
    let mut pc: u32 = 0x82C19584;
    'dispatch: loop {
        match pc {
            0x82C19584 => {
    //   block [0x82C19584..0x82C1959C)
	// 82C19584: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19588: 912A0018  stw r9, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82C1958C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19590: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19594: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1959C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1959C size=32
    let mut pc: u32 = 0x82C1959C;
    'dispatch: loop {
        match pc {
            0x82C1959C => {
    //   block [0x82C1959C..0x82C195BC)
	// 82C1959C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C195A0: 409A001C  bne cr6, 0x82c195bc
	if !ctx.cr[6].eq {
		sub_82C195BC(ctx, base);
		return;
	}
	// 82C195A4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C195A8: 912B0014  stw r9, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82C195AC: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C195B0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C195B4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C195B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C195BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C195BC size=24
    let mut pc: u32 = 0x82C195BC;
    'dispatch: loop {
        match pc {
            0x82C195BC => {
    //   block [0x82C195BC..0x82C195D4)
	// 82C195BC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82C195C0: 916A0018  stw r11, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C195C4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C195C8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C195CC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C195D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C195D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C195D8 size=16
    let mut pc: u32 = 0x82C195D8;
    'dispatch: loop {
        match pc {
            0x82C195D8 => {
    //   block [0x82C195D8..0x82C195E8)
	// 82C195D8: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C195DC: 39040008  addi r8, r4, 8
	ctx.r[8].s64 = ctx.r[4].s64 + 8;
	// 82C195E0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C195E4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C195E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C195E8 size=52
    let mut pc: u32 = 0x82C195E8;
    'dispatch: loop {
        match pc {
            0x82C195E8 => {
    //   block [0x82C195E8..0x82C1961C)
	// 82C195E8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C195EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C195F0: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C195F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C195F8: 409A003C  bne cr6, 0x82c19634
	if !ctx.cr[6].eq {
		sub_82C19634(ctx, base);
		return;
	}
	// 82C195FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19600: 409A001C  bne cr6, 0x82c1961c
	if !ctx.cr[6].eq {
		sub_82C1961C(ctx, base);
		return;
	}
	// 82C19604: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19608: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C1960C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19610: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19614: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1961C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1961C size=24
    let mut pc: u32 = 0x82C1961C;
    'dispatch: loop {
        match pc {
            0x82C1961C => {
    //   block [0x82C1961C..0x82C19634)
	// 82C1961C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19620: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C19624: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19628: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1962C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19634 size=32
    let mut pc: u32 = 0x82C19634;
    'dispatch: loop {
        match pc {
            0x82C19634 => {
    //   block [0x82C19634..0x82C19654)
	// 82C19634: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19638: 409A001C  bne cr6, 0x82c19654
	if !ctx.cr[6].eq {
		sub_82C19654(ctx, base);
		return;
	}
	// 82C1963C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C19640: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19644: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19648: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1964C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19654 size=24
    let mut pc: u32 = 0x82C19654;
    'dispatch: loop {
        match pc {
            0x82C19654 => {
    //   block [0x82C19654..0x82C1966C)
	// 82C19654: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C19658: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C1965C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19660: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19664: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19670 size=32
    let mut pc: u32 = 0x82C19670;
    'dispatch: loop {
        match pc {
            0x82C19670 => {
    //   block [0x82C19670..0x82C19690)
	// 82C19670: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C19674: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19678: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1967C: D0230044  stfs f1, 0x44(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82C19680: 90A30048  stw r5, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[5].u32 ) };
	// 82C19684: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82C19688: 80630060  lwz r3, 0x60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1968C: 48039EE4  b 0x82c53570
	sub_82C53570(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19690 size=24
    let mut pc: u32 = 0x82C19690;
    'dispatch: loop {
        match pc {
            0x82C19690 => {
    //   block [0x82C19690..0x82C196A8)
	// 82C19690: C0030034  lfs f0, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C19694: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82C19698: 409A0010  bne cr6, 0x82c196a8
	if !ctx.cr[6].eq {
		sub_82C196A8(ctx, base);
		return;
	}
	// 82C1969C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C196A0: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C196A4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C196A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C196A8 size=32
    let mut pc: u32 = 0x82C196A8;
    'dispatch: loop {
        match pc {
            0x82C196A8 => {
    //   block [0x82C196A8..0x82C196C8)
	// 82C196A8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C196AC: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C196B0: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C196B4: D0230034  stfs f1, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82C196B8: 90A30038  stw r5, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[5].u32 ) };
	// 82C196BC: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82C196C0: 80630060  lwz r3, 0x60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C196C4: 48039F14  b 0x82c535d8
	sub_82C535D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C196C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C196C8 size=4
    let mut pc: u32 = 0x82C196C8;
    'dispatch: loop {
        match pc {
            0x82C196C8 => {
    //   block [0x82C196C8..0x82C196CC)
	// 82C196C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C196D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C196D0 size=72
    let mut pc: u32 = 0x82C196D0;
    'dispatch: loop {
        match pc {
            0x82C196D0 => {
    //   block [0x82C196D0..0x82C19718)
	// 82C196D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C196D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C196D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C196DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C196E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C196E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C196E8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C196EC: 392BD0F8  addi r9, r11, -0x2f08
	ctx.r[9].s64 = ctx.r[11].s64 + -12040;
	// 82C196F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C196F4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C196F8: 419A000C  beq cr6, 0x82c19704
	if ctx.cr[6].eq {
	pc = 0x82C19704; continue 'dispatch;
	}
	// 82C196FC: 4BC2C0B5  bl 0x828457b0
	ctx.lr = 0x82C19700;
	sub_828457B0(ctx, base);
	// 82C19700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C19708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19718 size=84
    let mut pc: u32 = 0x82C19718;
    'dispatch: loop {
        match pc {
            0x82C19718 => {
    //   block [0x82C19718..0x82C1976C)
	// 82C19718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1972C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C19730: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C19734: 392BADA4  addi r9, r11, -0x525c
	ctx.r[9].s64 = ctx.r[11].s64 + -21084;
	// 82C19738: 390AA37C  addi r8, r10, -0x5c84
	ctx.r[8].s64 = ctx.r[10].s64 + -23684;
	// 82C1973C: 548707FE  clrlwi r7, r4, 0x1f
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C19740: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19744: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C19748: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C1974C: 419A000C  beq cr6, 0x82c19758
	if ctx.cr[6].eq {
	pc = 0x82C19758; continue 'dispatch;
	}
	// 82C19750: 4BC2C061  bl 0x828457b0
	ctx.lr = 0x82C19754;
	sub_828457B0(ctx, base);
	// 82C19754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1975C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19770 size=156
    let mut pc: u32 = 0x82C19770;
    'dispatch: loop {
        match pc {
            0x82C19770 => {
    //   block [0x82C19770..0x82C1980C)
	// 82C19770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1977C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19788: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82C1978C: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C19790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C19794: 419A0014  beq cr6, 0x82c197a8
	if ctx.cr[6].eq {
	pc = 0x82C197A8; continue 'dispatch;
	}
	// 82C19798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1979C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C197A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C197A4: 4E800421  bctrl
	ctx.lr = 0x82C197A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C197A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C197AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C197B0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C197B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C197B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C197BC: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82C197C0: 4BFFFD81  bl 0x82c19540
	ctx.lr = 0x82C197C4;
	sub_82C19540(ctx, base);
	// 82C197C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C197C8: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C197CC: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C197D0: 48039D49  bl 0x82c53518
	ctx.lr = 0x82C197D4;
	sub_82C53518(ctx, base);
	// 82C197D4: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C197D8: 4803A241  bl 0x82c53a18
	ctx.lr = 0x82C197DC;
	sub_82C53A18(ctx, base);
	// 82C197DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C197E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C197E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C197E8: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C197EC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C197F0: 4E800421  bctrl
	ctx.lr = 0x82C197F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C197F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C197F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C197FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19800: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19810 size=32
    let mut pc: u32 = 0x82C19810;
    'dispatch: loop {
        match pc {
            0x82C19810 => {
    //   block [0x82C19810..0x82C19830)
	// 82C19810: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C19814: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19818: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1981C: D0230024  stfs f1, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C19820: 90A30028  stw r5, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[5].u32 ) };
	// 82C19824: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82C19828: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C1982C: 48039E14  b 0x82c53640
	sub_82C53640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19830 size=104
    let mut pc: u32 = 0x82C19830;
    'dispatch: loop {
        match pc {
            0x82C19830 => {
    //   block [0x82C19830..0x82C19898)
	// 82C19830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1983C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19848: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1984C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C19850: 392BADAC  addi r9, r11, -0x5254
	ctx.r[9].s64 = ctx.r[11].s64 + -21076;
	// 82C19854: 390AA37C  addi r8, r10, -0x5c84
	ctx.r[8].s64 = ctx.r[10].s64 + -23684;
	// 82C19858: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1985C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C19860: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C19864: 4BFEC27D  bl 0x82c05ae0
	ctx.lr = 0x82C19868;
	sub_82C05AE0(ctx, base);
	// 82C19868: 57C707FE  clrlwi r7, r30, 0x1f
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1986C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19870: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C19874: 419A000C  beq cr6, 0x82c19880
	if ctx.cr[6].eq {
	pc = 0x82C19880; continue 'dispatch;
	}
	// 82C19878: 4BC2BF39  bl 0x828457b0
	ctx.lr = 0x82C1987C;
	sub_828457B0(ctx, base);
	// 82C1987C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19880: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1988C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19898 size=92
    let mut pc: u32 = 0x82C19898;
    'dispatch: loop {
        match pc {
            0x82C19898 => {
    //   block [0x82C19898..0x82C198F4)
	// 82C19898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1989C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C198A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C198A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C198A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C198AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C198B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C198B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C198B8: 394BADB4  addi r10, r11, -0x524c
	ctx.r[10].s64 = ctx.r[11].s64 + -21068;
	// 82C198BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C198C0: 4BFEC221  bl 0x82c05ae0
	ctx.lr = 0x82C198C4;
	sub_82C05AE0(ctx, base);
	// 82C198C4: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C198C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C198CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C198D0: 419A000C  beq cr6, 0x82c198dc
	if ctx.cr[6].eq {
	pc = 0x82C198DC; continue 'dispatch;
	}
	// 82C198D4: 4BC2BEDD  bl 0x828457b0
	ctx.lr = 0x82C198D8;
	sub_828457B0(ctx, base);
	// 82C198D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C198DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C198E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C198E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C198E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C198EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C198F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C198F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C198F8 size=32
    let mut pc: u32 = 0x82C198F8;
    'dispatch: loop {
        match pc {
            0x82C198F8 => {
    //   block [0x82C198F8..0x82C19918)
	// 82C198F8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C198FC: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19900: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C19904: D0230024  stfs f1, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C19908: 90A30028  stw r5, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[5].u32 ) };
	// 82C1990C: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82C19910: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C19914: 48039E0C  b 0x82c53720
	sub_82C53720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19918 size=48
    let mut pc: u32 = 0x82C19918;
    'dispatch: loop {
        match pc {
            0x82C19918 => {
    //   block [0x82C19918..0x82C19948)
	// 82C19918: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19948 size=44
    let mut pc: u32 = 0x82C19948;
    'dispatch: loop {
        match pc {
            0x82C19948 => {
    //   block [0x82C19948..0x82C19974)
	// 82C19948: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C1994C: 39430050  addi r10, r3, 0x50
	ctx.r[10].s64 = ctx.r[3].s64 + 80;
	// 82C19950: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82C19954: 816B4EF4  lwz r11, 0x4ef4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C19958: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19974(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19974 size=4
    let mut pc: u32 = 0x82C19974;
    'dispatch: loop {
        match pc {
            0x82C19974 => {
    //   block [0x82C19974..0x82C19978)
	// 82C19974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19978 size=16
    let mut pc: u32 = 0x82C19978;
    'dispatch: loop {
        match pc {
            0x82C19978 => {
    //   block [0x82C19978..0x82C19988)
	// 82C19978: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1997C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C19980: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C19984: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19988 size=20
    let mut pc: u32 = 0x82C19988;
    'dispatch: loop {
        match pc {
            0x82C19988 => {
    //   block [0x82C19988..0x82C1999C)
	// 82C19988: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1998C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19990: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C19994: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19998: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1999C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1999C size=4
    let mut pc: u32 = 0x82C1999C;
    'dispatch: loop {
        match pc {
            0x82C1999C => {
    //   block [0x82C1999C..0x82C199A0)
	// 82C1999C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C199A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C199A0 size=16
    let mut pc: u32 = 0x82C199A0;
    'dispatch: loop {
        match pc {
            0x82C199A0 => {
    //   block [0x82C199A0..0x82C199B0)
	// 82C199A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C199A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C199A8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C199AC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C199B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C199B0 size=20
    let mut pc: u32 = 0x82C199B0;
    'dispatch: loop {
        match pc {
            0x82C199B0 => {
    //   block [0x82C199B0..0x82C199C4)
	// 82C199B0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C199B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C199B8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C199BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C199C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C199C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C199C4 size=4
    let mut pc: u32 = 0x82C199C4;
    'dispatch: loop {
        match pc {
            0x82C199C4 => {
    //   block [0x82C199C4..0x82C199C8)
	// 82C199C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C199C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C199C8 size=104
    let mut pc: u32 = 0x82C199C8;
    'dispatch: loop {
        match pc {
            0x82C199C8 => {
    //   block [0x82C199C8..0x82C19A30)
	// 82C199C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C199CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C199D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C199D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C199D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C199DC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C199E0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C199E4: 40820038  bne 0x82c19a1c
	if !ctx.cr[0].eq {
	pc = 0x82C19A1C; continue 'dispatch;
	}
	// 82C199E8: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C199EC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C199F0: 48039319  bl 0x82c52d08
	ctx.lr = 0x82C199F4;
	sub_82C52D08(ctx, base);
	// 82C199F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C199F8: 4803A021  bl 0x82c53a18
	ctx.lr = 0x82C199FC;
	sub_82C53A18(ctx, base);
	// 82C199FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C19A00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C19A04: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C19A08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19A0C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19A10: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19A14: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C19A18: 4E800421  bctrl
	ctx.lr = 0x82C19A1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C19A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19A30 size=16
    let mut pc: u32 = 0x82C19A30;
    'dispatch: loop {
        match pc {
            0x82C19A30 => {
    //   block [0x82C19A30..0x82C19A40)
	// 82C19A30: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C19A34: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C19A38: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C19A3C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19A40 size=20
    let mut pc: u32 = 0x82C19A40;
    'dispatch: loop {
        match pc {
            0x82C19A40 => {
    //   block [0x82C19A40..0x82C19A54)
	// 82C19A40: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19A44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19A48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19A4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19A50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19A54 size=4
    let mut pc: u32 = 0x82C19A54;
    'dispatch: loop {
        match pc {
            0x82C19A54 => {
    //   block [0x82C19A54..0x82C19A58)
	// 82C19A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19A58 size=24
    let mut pc: u32 = 0x82C19A58;
    'dispatch: loop {
        match pc {
            0x82C19A58 => {
    //   block [0x82C19A58..0x82C19A70)
	// 82C19A58: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19A5C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19A60: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C19A64: 4098000C  bge cr6, 0x82c19a70
	if !ctx.cr[6].lt {
		sub_82C19A70(ctx, base);
		return;
	}
	// 82C19A68: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82C19A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C19A70 size=16
    let mut pc: u32 = 0x82C19A70;
    'dispatch: loop {
        match pc {
            0x82C19A70 => {
    //   block [0x82C19A70..0x82C19A80)
	// 82C19A70: 7D6B5010  subfc r11, r11, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C19A74: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C19A78: 554307FE  clrlwi r3, r10, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C19A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19A80 size=28
    let mut pc: u32 = 0x82C19A80;
    'dispatch: loop {
        match pc {
            0x82C19A80 => {
    //   block [0x82C19A80..0x82C19A9C)
	// 82C19A80: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C19A84: 39640008  addi r11, r4, 8
	ctx.r[11].s64 = ctx.r[4].s64 + 8;
	// 82C19A88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C19A8C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C19A90: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C19A94: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19A98: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19A9C size=12
    let mut pc: u32 = 0x82C19A9C;
    'dispatch: loop {
        match pc {
            0x82C19A9C => {
    //   block [0x82C19A9C..0x82C19AA8)
	// 82C19A9C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19AA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19AA4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19AA8 size=12
    let mut pc: u32 = 0x82C19AA8;
    'dispatch: loop {
        match pc {
            0x82C19AA8 => {
    //   block [0x82C19AA8..0x82C19AB4)
	// 82C19AA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19AAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19AB0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19AB4 size=36
    let mut pc: u32 = 0x82C19AB4;
    'dispatch: loop {
        match pc {
            0x82C19AB4 => {
    //   block [0x82C19AB4..0x82C19AD8)
	// 82C19AB4: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C19AB8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19ABC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19AC0: 409A0018  bne cr6, 0x82c19ad8
	if !ctx.cr[6].eq {
		sub_82C19AD8(ctx, base);
		return;
	}
	// 82C19AC4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C19AC8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19ACC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C19AD0: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C19AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19AD8 size=32
    let mut pc: u32 = 0x82C19AD8;
    'dispatch: loop {
        match pc {
            0x82C19AD8 => {
    //   block [0x82C19AD8..0x82C19AF8)
	// 82C19AD8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C19AE0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19AE4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C19AE8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19AEC: 90880008  stw r4, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82C19AF0: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C19AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19AF8 size=28
    let mut pc: u32 = 0x82C19AF8;
    'dispatch: loop {
        match pc {
            0x82C19AF8 => {
    //   block [0x82C19AF8..0x82C19B14)
	// 82C19AF8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C19AFC: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 82C19B00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C19B04: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C19B08: 8124001C  lwz r9, 0x1c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C19B0C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19B10: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19B14 size=12
    let mut pc: u32 = 0x82C19B14;
    'dispatch: loop {
        match pc {
            0x82C19B14 => {
    //   block [0x82C19B14..0x82C19B20)
	// 82C19B14: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19B18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19B1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19B20 size=12
    let mut pc: u32 = 0x82C19B20;
    'dispatch: loop {
        match pc {
            0x82C19B20 => {
    //   block [0x82C19B20..0x82C19B2C)
	// 82C19B20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19B24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19B28: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19B2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19B2C size=36
    let mut pc: u32 = 0x82C19B2C;
    'dispatch: loop {
        match pc {
            0x82C19B2C => {
    //   block [0x82C19B2C..0x82C19B50)
	// 82C19B2C: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C19B30: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19B34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C19B38: 409A0018  bne cr6, 0x82c19b50
	if !ctx.cr[6].eq {
		sub_82C19B50(ctx, base);
		return;
	}
	// 82C19B3C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C19B40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19B44: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C19B48: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C19B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C19B50 size=32
    let mut pc: u32 = 0x82C19B50;
    'dispatch: loop {
        match pc {
            0x82C19B50 => {
    //   block [0x82C19B50..0x82C19B70)
	// 82C19B50: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C19B58: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C19B5C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C19B60: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19B64: 90880014  stw r4, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82C19B68: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C19B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19B70 size=148
    let mut pc: u32 = 0x82C19B70;
    'dispatch: loop {
        match pc {
            0x82C19B70 => {
    //   block [0x82C19B70..0x82C19C04)
	// 82C19B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19B74: 4808F891  bl 0x82ca9404
	ctx.lr = 0x82C19B78;
	sub_82CA93D0(ctx, base);
	// 82C19B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19B7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C19B80: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C19B84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C19B88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C19B8C: 8B9E005C  lbz r28, 0x5c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C19B90: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 82C19B94: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82C19B98: 995E005C  stb r10, 0x5c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[10].u8 ) };
	// 82C19B9C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19BA0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19BA4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C19BA8: 4E800421  bctrl
	ctx.lr = 0x82C19BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19BAC: 387E0054  addi r3, r30, 0x54
	ctx.r[3].s64 = ctx.r[30].s64 + 84;
	// 82C19BB0: 48033409  bl 0x82c4cfb8
	ctx.lr = 0x82C19BB4;
	sub_82C4CFB8(ctx, base);
	// 82C19BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19BB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C19BBC: 419A0040  beq cr6, 0x82c19bfc
	if ctx.cr[6].eq {
	pc = 0x82C19BFC; continue 'dispatch;
	}
	// 82C19BC0: 3BDE0050  addi r30, r30, 0x50
	ctx.r[30].s64 = ctx.r[30].s64 + 80;
	// 82C19BC4: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 82C19BC8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19BCC: 8963005C  lbz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C19BD0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C19BD4: 409A000C  bne cr6, 0x82c19be0
	if !ctx.cr[6].eq {
	pc = 0x82C19BE0; continue 'dispatch;
	}
	// 82C19BD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C19BDC: 4BFFFF95  bl 0x82c19b70
	ctx.lr = 0x82C19BE0;
	sub_82C19B70(ctx, base);
	// 82C19BE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C19BE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C19BE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C19BEC: 4803395D  bl 0x82c4d548
	ctx.lr = 0x82C19BF0;
	sub_82C4D548(ctx, base);
	// 82C19BF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19BF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C19BF8: 409AFFD0  bne cr6, 0x82c19bc8
	if !ctx.cr[6].eq {
	pc = 0x82C19BC8; continue 'dispatch;
	}
	// 82C19BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C19C00: 4808F854  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19C08 size=116
    let mut pc: u32 = 0x82C19C08;
    'dispatch: loop {
        match pc {
            0x82C19C08 => {
    //   block [0x82C19C08..0x82C19C7C)
	// 82C19C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19C0C: 4808F7FD  bl 0x82ca9408
	ctx.lr = 0x82C19C10;
	sub_82CA93D0(ctx, base);
	// 82C19C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19C14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C19C18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C19C1C: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82C19C20: 895E005C  lbz r10, 0x5c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C19C24: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C19C28: 419A004C  beq cr6, 0x82c19c74
	if ctx.cr[6].eq {
	pc = 0x82C19C74; continue 'dispatch;
	}
	// 82C19C2C: 9BBE005C  stb r29, 0x5c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[29].u8 ) };
	// 82C19C30: 387E0054  addi r3, r30, 0x54
	ctx.r[3].s64 = ctx.r[30].s64 + 84;
	// 82C19C34: 48033385  bl 0x82c4cfb8
	ctx.lr = 0x82C19C38;
	sub_82C4CFB8(ctx, base);
	// 82C19C38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19C3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C19C40: 419A0034  beq cr6, 0x82c19c74
	if ctx.cr[6].eq {
	pc = 0x82C19C74; continue 'dispatch;
	}
	// 82C19C44: 3BDE0050  addi r30, r30, 0x50
	ctx.r[30].s64 = ctx.r[30].s64 + 80;
	// 82C19C48: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 82C19C4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C19C50: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19C54: 4BFFFFB5  bl 0x82c19c08
	ctx.lr = 0x82C19C58;
	sub_82C19C08(ctx, base);
	// 82C19C58: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C19C5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C19C60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C19C64: 480338E5  bl 0x82c4d548
	ctx.lr = 0x82C19C68;
	sub_82C4D548(ctx, base);
	// 82C19C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19C6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C19C70: 409AFFDC  bne cr6, 0x82c19c4c
	if !ctx.cr[6].eq {
	pc = 0x82C19C4C; continue 'dispatch;
	}
	// 82C19C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C19C78: 4808F7E0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19C80 size=96
    let mut pc: u32 = 0x82C19C80;
    'dispatch: loop {
        match pc {
            0x82C19C80 => {
    //   block [0x82C19C80..0x82C19CE0)
	// 82C19C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19C88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C19C8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19C90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19C94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19C98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C19C9C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C19CA0: 394BAE40  addi r10, r11, -0x51c0
	ctx.r[10].s64 = ctx.r[11].s64 + -20928;
	// 82C19CA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C19CA8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19CAC: 48033EF5  bl 0x82c4dba0
	ctx.lr = 0x82C19CB0;
	sub_82C4DBA0(ctx, base);
	// 82C19CB0: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C19CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19CB8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19CBC: 419A000C  beq cr6, 0x82c19cc8
	if ctx.cr[6].eq {
	pc = 0x82C19CC8; continue 'dispatch;
	}
	// 82C19CC0: 4BC2BAF1  bl 0x828457b0
	ctx.lr = 0x82C19CC4;
	sub_828457B0(ctx, base);
	// 82C19CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19CD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19CE0 size=96
    let mut pc: u32 = 0x82C19CE0;
    'dispatch: loop {
        match pc {
            0x82C19CE0 => {
    //   block [0x82C19CE0..0x82C19D40)
	// 82C19CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C19CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19CF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19CF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C19CFC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C19D00: 394BAE68  addi r10, r11, -0x5198
	ctx.r[10].s64 = ctx.r[11].s64 + -20888;
	// 82C19D04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C19D08: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19D0C: 48033E95  bl 0x82c4dba0
	ctx.lr = 0x82C19D10;
	sub_82C4DBA0(ctx, base);
	// 82C19D10: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C19D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19D18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19D1C: 419A000C  beq cr6, 0x82c19d28
	if ctx.cr[6].eq {
	pc = 0x82C19D28; continue 'dispatch;
	}
	// 82C19D20: 4BC2BA91  bl 0x828457b0
	ctx.lr = 0x82C19D24;
	sub_828457B0(ctx, base);
	// 82C19D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19D28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19D34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19D38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C19D40 size=176
    let mut pc: u32 = 0x82C19D40;
    'dispatch: loop {
        match pc {
            0x82C19D40 => {
    //   block [0x82C19D40..0x82C19DF0)
	// 82C19D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19D48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C19D4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19D50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19D54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19D58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C19D5C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C19D60: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C19D64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C19D68: 390BAE30  addi r8, r11, -0x51d0
	ctx.r[8].s64 = ctx.r[11].s64 + -20944;
	// 82C19D6C: 38EAAE00  addi r7, r10, -0x5200
	ctx.r[7].s64 = ctx.r[10].s64 + -20992;
	// 82C19D70: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C19D74: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C19D78: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82C19D7C: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C19D80: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C19D84: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C19D88: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C19D8C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82C19D90: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C19D94: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C19D98: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82C19D9C: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C19DA0: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82C19DA4: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82C19DA8: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82C19DAC: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82C19DB0: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C19DB4: 4B6054A5  bl 0x8221f258
	ctx.lr = 0x82C19DB8;
	sub_8221F258(ctx, base);
	// 82C19DB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C19DBC: 419A0014  beq cr6, 0x82c19dd0
	if ctx.cr[6].eq {
	pc = 0x82C19DD0; continue 'dispatch;
	}
	// 82C19DC0: 48039E19  bl 0x82c53bd8
	ctx.lr = 0x82C19DC4;
	sub_82C53BD8(ctx, base);
	// 82C19DC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C19DC8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82C19DCC: 48000008  b 0x82c19dd4
	pc = 0x82C19DD4; continue 'dispatch;
	// 82C19DD0: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C19DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19DD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19DE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19DE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19DF0 size=124
    let mut pc: u32 = 0x82C19DF0;
    'dispatch: loop {
        match pc {
            0x82C19DF0 => {
    //   block [0x82C19DF0..0x82C19E6C)
	// 82C19DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19DF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C19DFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19E00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19E04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19E08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C19E0C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82C19E10: 394BAE30  addi r10, r11, -0x51d0
	ctx.r[10].s64 = ctx.r[11].s64 + -20944;
	// 82C19E14: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C19E18: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C19E1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C19E20: 419A0014  beq cr6, 0x82c19e34
	if ctx.cr[6].eq {
	pc = 0x82C19E34; continue 'dispatch;
	}
	// 82C19E24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19E28: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19E2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19E30: 4E800421  bctrl
	ctx.lr = 0x82C19E34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19E34: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C19E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C19E3C: 392AA37C  addi r9, r10, -0x5c84
	ctx.r[9].s64 = ctx.r[10].s64 + -23684;
	// 82C19E40: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C19E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19E48: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C19E4C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C19E50: 4BFEBCA1  bl 0x82c05af0
	ctx.lr = 0x82C19E54;
	sub_82C05AF0(ctx, base);
	// 82C19E54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C19E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C19E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19E70 size=328
    let mut pc: u32 = 0x82C19E70;
    'dispatch: loop {
        match pc {
            0x82C19E70 => {
    //   block [0x82C19E70..0x82C19FB8)
	// 82C19E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19E74: 4808F58D  bl 0x82ca9400
	ctx.lr = 0x82C19E78;
	sub_82CA93D0(ctx, base);
	// 82C19E78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19E7C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C19E80: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82C19E84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C19E88: 4B6053D1  bl 0x8221f258
	ctx.lr = 0x82C19E8C;
	sub_8221F258(ctx, base);
	// 82C19E8C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C19E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C19E94: 419A0010  beq cr6, 0x82c19ea4
	if ctx.cr[6].eq {
	pc = 0x82C19EA4; continue 'dispatch;
	}
	// 82C19E98: 4BFFFEA9  bl 0x82c19d40
	ctx.lr = 0x82C19E9C;
	sub_82C19D40(ctx, base);
	// 82C19E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19EA0: 48000008  b 0x82c19ea8
	pc = 0x82C19EA8; continue 'dispatch;
	// 82C19EA4: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82C19EA8: 37BC0014  addic. r29, r28, 0x14
	ctx.xer.ca = (ctx.r[28].u32 > (!(20 as u32)));
	ctx.r[29].s64 = ctx.r[28].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C19EAC: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C19EB0: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82C19EB4: 41820018  beq 0x82c19ecc
	if ctx.cr[0].eq {
	pc = 0x82C19ECC; continue 'dispatch;
	}
	// 82C19EB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19EBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C19EC0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19EC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19EC8: 4E800421  bctrl
	ctx.lr = 0x82C19ECC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19ECC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19ED0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C19ED4: 419A0014  beq cr6, 0x82c19ee8
	if ctx.cr[6].eq {
	pc = 0x82C19EE8; continue 'dispatch;
	}
	// 82C19ED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19EDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19EE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19EE4: 4E800421  bctrl
	ctx.lr = 0x82C19EE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19EE8: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C19EEC: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82C19EF0: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C19EF4: 395C0030  addi r10, r28, 0x30
	ctx.r[10].s64 = ctx.r[28].s64 + 48;
	// 82C19EF8: 93FF000C  stw r31, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82C19EFC: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C19F00: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19F04: 409A0054  bne cr6, 0x82c19f58
	if !ctx.cr[6].eq {
	pc = 0x82C19F58; continue 'dispatch;
	}
	// 82C19F08: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19F0C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19F10: 409A0048  bne cr6, 0x82c19f58
	if !ctx.cr[6].eq {
	pc = 0x82C19F58; continue 'dispatch;
	}
	// 82C19F14: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19F18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19F1C: 409A003C  bne cr6, 0x82c19f58
	if !ctx.cr[6].eq {
	pc = 0x82C19F58; continue 'dispatch;
	}
	// 82C19F20: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C19F24: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19F28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C19F2C: 409A0014  bne cr6, 0x82c19f40
	if !ctx.cr[6].eq {
	pc = 0x82C19F40; continue 'dispatch;
	}
	// 82C19F30: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C19F34: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C19F38: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C19F3C: 48000018  b 0x82c19f54
	pc = 0x82C19F54; continue 'dispatch;
	// 82C19F40: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19F44: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C19F48: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C19F4C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19F50: 93E80014  stw r31, 0x14(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82C19F54: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C19F58: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C19F5C: 807C0048  lwz r3, 0x48(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C19F60: 480391C9  bl 0x82c53128
	ctx.lr = 0x82C19F64;
	sub_82C53128(ctx, base);
	// 82C19F64: 37DF0008  addic. r30, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C19F68: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C19F6C: 937A0004  stw r27, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C19F70: 41820018  beq 0x82c19f88
	if ctx.cr[0].eq {
	pc = 0x82C19F88; continue 'dispatch;
	}
	// 82C19F74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C19F7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19F80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19F84: 4E800421  bctrl
	ctx.lr = 0x82C19F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19F88: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19F8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C19F90: 419A0014  beq cr6, 0x82c19fa4
	if ctx.cr[6].eq {
	pc = 0x82C19FA4; continue 'dispatch;
	}
	// 82C19F94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C19F98: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C19F9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C19FA0: 4E800421  bctrl
	ctx.lr = 0x82C19FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C19FA4: 93DA0004  stw r30, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C19FA8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C19FAC: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C19FB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C19FB4: 4808F49C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C19FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C19FB8 size=100
    let mut pc: u32 = 0x82C19FB8;
    'dispatch: loop {
        match pc {
            0x82C19FB8 => {
    //   block [0x82C19FB8..0x82C1A01C)
	// 82C19FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C19FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C19FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C19FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C19FC8: 896400D0  lbz r11, 0xd0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(208 as u32) ) } as u64;
	// 82C19FCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C19FD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C19FD4: 419A0028  beq cr6, 0x82c19ffc
	if ctx.cr[6].eq {
	pc = 0x82C19FFC; continue 'dispatch;
	}
	// 82C19FD8: 38A40014  addi r5, r4, 0x14
	ctx.r[5].s64 = ctx.r[4].s64 + 20;
	// 82C19FDC: 80840038  lwz r4, 0x38(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C19FE0: 4B5BB201  bl 0x821d51e0
	ctx.lr = 0x82C19FE4;
	sub_821D51E0(ctx, base);
	// 82C19FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C19FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C19FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C19FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C19FF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C19FF8: 4E800020  blr
	return;
	// 82C19FFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1A000: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1A004: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1A008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1A00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A020 size=124
    let mut pc: u32 = 0x82C1A020;
    'dispatch: loop {
        match pc {
            0x82C1A020 => {
    //   block [0x82C1A020..0x82C1A09C)
	// 82C1A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1A038: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C1A03C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1A040: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1A044: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1A048: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C1A04C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C1A050: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C1A054: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C1A058: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82C1A05C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82C1A060: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82C1A064: 909F0028  stw r4, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82C1A068: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C1A06C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82C1A070: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82C1A074: 4803810D  bl 0x82c52180
	ctx.lr = 0x82C1A078;
	sub_82C52180(ctx, base);
	// 82C1A078: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82C1A07C: 4803815D  bl 0x82c521d8
	ctx.lr = 0x82C1A080;
	sub_82C521D8(ctx, base);
	// 82C1A080: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C1A084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1A08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1A0A0 size=232
    let mut pc: u32 = 0x82C1A0A0;
    'dispatch: loop {
        match pc {
            0x82C1A0A0 => {
    //   block [0x82C1A0A0..0x82C1A188)
	// 82C1A0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A0A4: 4808F365  bl 0x82ca9408
	ctx.lr = 0x82C1A0A8;
	sub_82CA93D0(ctx, base);
	// 82C1A0A8: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1A188 size=520
    let mut pc: u32 = 0x82C1A188;
    'dispatch: loop {
        match pc {
            0x82C1A188 => {
    //   block [0x82C1A188..0x82C1A390)
	// 82C1A188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A194: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A19C: 48037FB5  bl 0x82c52150
	ctx.lr = 0x82C1A1A0;
	sub_82C52150(ctx, base);
	// 82C1A1A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1A1A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1A1A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C1A1AC: 4B5E65C5  bl 0x82200770
	ctx.lr = 0x82C1A1B0;
	sub_82200770(ctx, base);
	// 82C1A1B0: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C1A1B4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1A1B8: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 82C1A1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A1C0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1A1C4: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82C1A1C8: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A1CC: 90FF0024  stw r7, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82C1A1D0: 419A0084  beq cr6, 0x82c1a254
	if ctx.cr[6].eq {
	pc = 0x82C1A254; continue 'dispatch;
	}
	// 82C1A1D4: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1A1D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1A1DC: 419A0048  beq cr6, 0x82c1a224
	if ctx.cr[6].eq {
	pc = 0x82C1A224; continue 'dispatch;
	}
	// 82C1A1E0: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C1A1E4: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A1E8: 80EB003C  lwz r7, 0x3c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C1A1EC: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82C1A1F0: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C1A1F4: 4198000C  blt cr6, 0x82c1a200
	if ctx.cr[6].lt {
	pc = 0x82C1A200; continue 'dispatch;
	}
	// 82C1A1F8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A1FC: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82C1A200: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1A204: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C1A208: 80EB004C  lwz r7, 0x4c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1A20C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C1A210: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A214: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82C1A218: 4198000C  blt cr6, 0x82c1a224
	if ctx.cr[6].lt {
	pc = 0x82C1A224; continue 'dispatch;
	}
	// 82C1A21C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1A220: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82C1A224: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1A228: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1A22C: 409A001C  bne cr6, 0x82c1a248
	if !ctx.cr[6].eq {
	pc = 0x82C1A248; continue 'dispatch;
	}
	// 82C1A230: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1A234: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A238: 419A001C  beq cr6, 0x82c1a254
	if ctx.cr[6].eq {
	pc = 0x82C1A254; continue 'dispatch;
	}
	// 82C1A23C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1A240: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1A244: 419A0008  beq cr6, 0x82c1a24c
	if ctx.cr[6].eq {
	pc = 0x82C1A24C; continue 'dispatch;
	}
	// 82C1A248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1A24C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A250: 409AFF84  bne cr6, 0x82c1a1d4
	if !ctx.cr[6].eq {
	pc = 0x82C1A1D4; continue 'dispatch;
	}
	// 82C1A254: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A258: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 82C1A25C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1A260: 419A0114  beq cr6, 0x82c1a374
	if ctx.cr[6].eq {
	pc = 0x82C1A374; continue 'dispatch;
	}
	// 82C1A264: 816A0048  lwz r11, 0x48(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1A268: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C1A26C: 810A002C  lwz r8, 0x2c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C1A270: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C1A274: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A278: D00A0020  stfs f0, 0x20(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82C1A27C: 4198000C  blt cr6, 0x82c1a288
	if ctx.cr[6].lt {
	pc = 0x82C1A288; continue 'dispatch;
	}
	// 82C1A280: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A284: 916A0028  stw r11, 0x28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C1A288: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C1A28C: 390A0030  addi r8, r10, 0x30
	ctx.r[8].s64 = ctx.r[10].s64 + 48;
	// 82C1A290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A294: 419A0054  beq cr6, 0x82c1a2e8
	if ctx.cr[6].eq {
	pc = 0x82C1A2E8; continue 'dispatch;
	}
	// 82C1A298: 812B0038  lwz r9, 0x38(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C1A29C: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C1A2A0: 80AB002C  lwz r5, 0x2c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C1A2A4: 7F062840  cmplw cr6, r6, r5
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C1A2A8: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A2AC: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82C1A2B0: 4198000C  blt cr6, 0x82c1a2bc
	if ctx.cr[6].lt {
	pc = 0x82C1A2BC; continue 'dispatch;
	}
	// 82C1A2B4: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A2B8: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82C1A2BC: 812B001C  lwz r9, 0x1c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1A2C0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C1A2C4: 409A0024  bne cr6, 0x82c1a2e8
	if !ctx.cr[6].eq {
	pc = 0x82C1A2E8; continue 'dispatch;
	}
	// 82C1A2C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1A2CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A2D0: 419A0018  beq cr6, 0x82c1a2e8
	if ctx.cr[6].eq {
	pc = 0x82C1A2E8; continue 'dispatch;
	}
	// 82C1A2D4: 812B001C  lwz r9, 0x1c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1A2D8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C1A2DC: 409A000C  bne cr6, 0x82c1a2e8
	if !ctx.cr[6].eq {
	pc = 0x82C1A2E8; continue 'dispatch;
	}
	// 82C1A2E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A2E4: 409AFFB4  bne cr6, 0x82c1a298
	if !ctx.cr[6].eq {
	pc = 0x82C1A298; continue 'dispatch;
	}
	// 82C1A2E8: 816A0048  lwz r11, 0x48(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1A2EC: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A2F0: D00A0088  stfs f0, 0x88(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82C1A2F4: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C1A2F8: D1AA0084  stfs f13, 0x84(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82C1A2FC: C18B0010  lfs f12, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C1A300: D18A008C  stfs f12, 0x8c(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82C1A304: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1A308: 912A0094  stw r9, 0x94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82C1A30C: 890B001C  lbz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1A310: 990A00D2  stb r8, 0xd2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(210 as u32), ctx.r[8].u8 ) };
	// 82C1A314: 88CB001D  lbz r6, 0x1d(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 82C1A318: 98CA00D3  stb r6, 0xd3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(211 as u32), ctx.r[6].u8 ) };
	// 82C1A31C: E8AB0020  ld r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82C1A320: F8AA00A0  std r5, 0xa0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(160 as u32), ctx.r[5].u64 ) };
	// 82C1A324: E88B0028  ld r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82C1A328: F88A00A8  std r4, 0xa8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(168 as u32), ctx.r[4].u64 ) };
	// 82C1A32C: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C1A330: 906A00B8  stw r3, 0xb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(184 as u32), ctx.r[3].u32 ) };
	// 82C1A334: 892B001E  lbz r9, 0x1e(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(30 as u32) ) } as u64;
	// 82C1A338: 992A00D5  stb r9, 0xd5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(213 as u32), ctx.r[9].u8 ) };
	// 82C1A33C: 890B001F  lbz r8, 0x1f(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(31 as u32) ) } as u64;
	// 82C1A340: 990A00D7  stb r8, 0xd7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(215 as u32), ctx.r[8].u8 ) };
	// 82C1A344: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1A348: 7F063840  cmplw cr6, r6, r7
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C1A34C: 409A001C  bne cr6, 0x82c1a368
	if !ctx.cr[6].eq {
	pc = 0x82C1A368; continue 'dispatch;
	}
	// 82C1A350: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1A354: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1A358: 419A001C  beq cr6, 0x82c1a374
	if ctx.cr[6].eq {
	pc = 0x82C1A374; continue 'dispatch;
	}
	// 82C1A35C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1A360: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C1A364: 419A0008  beq cr6, 0x82c1a36c
	if ctx.cr[6].eq {
	pc = 0x82C1A36C; continue 'dispatch;
	}
	// 82C1A368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C1A36C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1A370: 409AFEF4  bne cr6, 0x82c1a264
	if !ctx.cr[6].eq {
	pc = 0x82C1A264; continue 'dispatch;
	}
	// 82C1A374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1A378: 4B5A9241  bl 0x821c35b8
	ctx.lr = 0x82C1A37C;
	sub_821C35B8(ctx, base);
	// 82C1A37C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1A380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1A390 size=264
    let mut pc: u32 = 0x82C1A390;
    'dispatch: loop {
        match pc {
            0x82C1A390 => {
    //   block [0x82C1A390..0x82C1A498)
	// 82C1A390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A394: 4808F071  bl 0x82ca9404
	ctx.lr = 0x82C1A398;
	sub_82CA93D0(ctx, base);
	// 82C1A398: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82C1A39C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A3A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C1A3A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C1A3A8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C1A3AC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C1A3B0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C1A3B4: 4B604EA5  bl 0x8221f258
	ctx.lr = 0x82C1A3B8;
	sub_8221F258(ctx, base);
	// 82C1A3B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1A3BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C1A3C0: 419A002C  beq cr6, 0x82c1a3ec
	if ctx.cr[6].eq {
	pc = 0x82C1A3EC; continue 'dispatch;
	}
	// 82C1A3C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1A3C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1A3CC: 392BADA4  addi r9, r11, -0x525c
	ctx.r[9].s64 = ctx.r[11].s64 + -21084;
	// 82C1A3D0: 390AAE18  addi r8, r10, -0x51e8
	ctx.r[8].s64 = ctx.r[10].s64 + -20968;
	// 82C1A3D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1A3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A3DC: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C1A3E0: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C1A3E4: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C1A3E8: 48000008  b 0x82c1a3f0
	pc = 0x82C1A3F0; continue 'dispatch;
	// 82C1A3EC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82C1A3F0: 93FF0008  stw r31, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C1A3F4: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82C1A3F8: 4B604E61  bl 0x8221f258
	ctx.lr = 0x82C1A3FC;
	sub_8221F258(ctx, base);
	// 82C1A3FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1A400: 419A000C  beq cr6, 0x82c1a40c
	if ctx.cr[6].eq {
	pc = 0x82C1A40C; continue 'dispatch;
	}
	// 82C1A404: 4803974D  bl 0x82c53b50
	ctx.lr = 0x82C1A408;
	sub_82C53B50(ctx, base);
	// 82C1A408: 48000008  b 0x82c1a410
	pc = 0x82C1A410; continue 'dispatch;
	// 82C1A40C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1A410: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82C1A414: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82C1A418: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1A41C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C1A420: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1A424: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1A428: 4808F059  bl 0x82ca9480
	ctx.lr = 0x82C1A42C;
	sub_82CA9480(ctx, base);
	// 82C1A42C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1A430: D3EB0034  stfs f31, 0x34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82C1A434: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1A438: 480394A9  bl 0x82c538e0
	ctx.lr = 0x82C1A43C;
	sub_82C538E0(ctx, base);
	// 82C1A43C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82C1A440: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C1A444: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1A448: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C1A44C: 419A0018  beq cr6, 0x82c1a464
	if ctx.cr[6].eq {
	pc = 0x82C1A464; continue 'dispatch;
	}
	// 82C1A450: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1A454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A458: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1A45C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1A460: 4E800421  bctrl
	ctx.lr = 0x82C1A464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1A464: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A468: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1A46C: 419A0014  beq cr6, 0x82c1a480
	if ctx.cr[6].eq {
	pc = 0x82C1A480; continue 'dispatch;
	}
	// 82C1A470: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1A474: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A478: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1A47C: 4E800421  bctrl
	ctx.lr = 0x82C1A480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1A480: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C1A484: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1A488: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C1A48C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1A490: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82C1A494: 4808EFC0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A498 size=196
    let mut pc: u32 = 0x82C1A498;
    'dispatch: loop {
        match pc {
            0x82C1A498 => {
    //   block [0x82C1A498..0x82C1A55C)
	// 82C1A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A49C: 4808EF71  bl 0x82ca940c
	ctx.lr = 0x82C1A4A0;
	sub_82CA93D0(ctx, base);
	// 82C1A4A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A4A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C1A4A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1A4AC: 889D005C  lbz r4, 0x5c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C1A4B0: 897E005C  lbz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C1A4B4: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C1A4B8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C1A4BC: 419A000C  beq cr6, 0x82c1a4c8
	if ctx.cr[6].eq {
	pc = 0x82C1A4C8; continue 'dispatch;
	}
	// 82C1A4C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1A4C4: 4BFFF745  bl 0x82c19c08
	ctx.lr = 0x82C1A4C8;
	sub_82C19C08(ctx, base);
	// 82C1A4C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C1A4CC: 4B604D8D  bl 0x8221f258
	ctx.lr = 0x82C1A4D0;
	sub_8221F258(ctx, base);
	// 82C1A4D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A4D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1A4D8: 419A001C  beq cr6, 0x82c1a4f4
	if ctx.cr[6].eq {
	pc = 0x82C1A4F4; continue 'dispatch;
	}
	// 82C1A4DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1A4E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1A4E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1A4E8: 480335A9  bl 0x82c4da90
	ctx.lr = 0x82C1A4EC;
	sub_82C4DA90(ctx, base);
	// 82C1A4EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1A4F0: 48000008  b 0x82c1a4f8
	pc = 0x82C1A4F8; continue 'dispatch;
	// 82C1A4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C1A4F8: 93C50000  stw r30, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C1A4FC: 389D0050  addi r4, r29, 0x50
	ctx.r[4].s64 = ctx.r[29].s64 + 80;
	// 82C1A500: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C1A504: 48032DBD  bl 0x82c4d2c0
	ctx.lr = 0x82C1A508;
	sub_82C4D2C0(ctx, base);
	// 82C1A508: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C1A50C: 4B604D4D  bl 0x8221f258
	ctx.lr = 0x82C1A510;
	sub_8221F258(ctx, base);
	// 82C1A510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A514: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1A518: 419A001C  beq cr6, 0x82c1a534
	if ctx.cr[6].eq {
	pc = 0x82C1A534; continue 'dispatch;
	}
	// 82C1A51C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1A520: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1A524: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1A528: 48033569  bl 0x82c4da90
	ctx.lr = 0x82C1A52C;
	sub_82C4DA90(ctx, base);
	// 82C1A52C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1A530: 48000008  b 0x82c1a538
	pc = 0x82C1A538; continue 'dispatch;
	// 82C1A534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C1A538: 93A50000  stw r29, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C1A53C: 389E0050  addi r4, r30, 0x50
	ctx.r[4].s64 = ctx.r[30].s64 + 80;
	// 82C1A540: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C1A544: 48032D7D  bl 0x82c4d2c0
	ctx.lr = 0x82C1A548;
	sub_82C4D2C0(ctx, base);
	// 82C1A548: 809E0060  lwz r4, 0x60(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1A54C: 807D0060  lwz r3, 0x60(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1A550: 480389F9  bl 0x82c52f48
	ctx.lr = 0x82C1A554;
	sub_82C52F48(ctx, base);
	// 82C1A554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1A558: 4808EF04  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A560 size=164
    let mut pc: u32 = 0x82C1A560;
    'dispatch: loop {
        match pc {
            0x82C1A560 => {
    //   block [0x82C1A560..0x82C1A604)
	// 82C1A560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1A56C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A574: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C1A578: 817E4EF4  lwz r11, 0x4ef4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1A57C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A580: 419A000C  beq cr6, 0x82c1a58c
	if ctx.cr[6].eq {
	pc = 0x82C1A58C; continue 'dispatch;
	}
	// 82C1A584: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1A588: 48000064  b 0x82c1a5ec
	pc = 0x82C1A5EC; continue 'dispatch;
	// 82C1A58C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82C1A590: 4B604CC9  bl 0x8221f258
	ctx.lr = 0x82C1A594;
	sub_8221F258(ctx, base);
	// 82C1A594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1A598: 419A0010  beq cr6, 0x82c1a5a8
	if ctx.cr[6].eq {
	pc = 0x82C1A5A8; continue 'dispatch;
	}
	// 82C1A59C: 480395A5  bl 0x82c53b40
	ctx.lr = 0x82C1A5A0;
	sub_82C53B40(ctx, base);
	// 82C1A5A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A5A4: 48000008  b 0x82c1a5ac
	pc = 0x82C1A5AC; continue 'dispatch;
	// 82C1A5A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C1A5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A5B0: 48037A31  bl 0x82c51fe0
	ctx.lr = 0x82C1A5B4;
	sub_82C51FE0(ctx, base);
	// 82C1A5B4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1A5B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A5BC: 419AFFC8  beq cr6, 0x82c1a584
	if ctx.cr[6].eq {
	pc = 0x82C1A584; continue 'dispatch;
	}
	// 82C1A5C0: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82C1A5C4: 4B604C95  bl 0x8221f258
	ctx.lr = 0x82C1A5C8;
	sub_8221F258(ctx, base);
	// 82C1A5C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1A5CC: 419A0014  beq cr6, 0x82c1a5e0
	if ctx.cr[6].eq {
	pc = 0x82C1A5E0; continue 'dispatch;
	}
	// 82C1A5D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1A5D4: 4BFFFA4D  bl 0x82c1a020
	ctx.lr = 0x82C1A5D8;
	sub_82C1A020(ctx, base);
	// 82C1A5D8: 907E4EF4  stw r3, 0x4ef4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20212 as u32), ctx.r[3].u32 ) };
	// 82C1A5DC: 4800000C  b 0x82c1a5e8
	pc = 0x82C1A5E8; continue 'dispatch;
	// 82C1A5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1A5E4: 917E4EF4  stw r11, 0x4ef4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20212 as u32), ctx.r[11].u32 ) };
	// 82C1A5E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C1A5EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1A5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A5F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1A5FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1A608 size=64
    let mut pc: u32 = 0x82C1A608;
    'dispatch: loop {
        match pc {
            0x82C1A608 => {
    //   block [0x82C1A608..0x82C1A648)
	// 82C1A608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A614: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C1A618: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1A61C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C1A620: 392AADBC  addi r9, r10, -0x5244
	ctx.r[9].s64 = ctx.r[10].s64 + -21060;
	// 82C1A624: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A628: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C1A62C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C1A630: 4BFFF541  bl 0x82c19b70
	ctx.lr = 0x82C1A634;
	sub_82C19B70(ctx, base);
	// 82C1A634: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C1A638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1A63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1A648 size=64
    let mut pc: u32 = 0x82C1A648;
    'dispatch: loop {
        match pc {
            0x82C1A648 => {
    //   block [0x82C1A648..0x82C1A688)
	// 82C1A648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A654: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C1A658: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1A65C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C1A660: 392AADC4  addi r9, r10, -0x523c
	ctx.r[9].s64 = ctx.r[10].s64 + -21052;
	// 82C1A664: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A668: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C1A66C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C1A670: 4BFFF501  bl 0x82c19b70
	ctx.lr = 0x82C1A674;
	sub_82C19B70(ctx, base);
	// 82C1A674: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C1A678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1A67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A688 size=128
    let mut pc: u32 = 0x82C1A688;
    'dispatch: loop {
        match pc {
            0x82C1A688 => {
    //   block [0x82C1A688..0x82C1A708)
	// 82C1A688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A69C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1A6A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1A6A4: 392BAE90  addi r9, r11, -0x5170
	ctx.r[9].s64 = ctx.r[11].s64 + -20848;
	// 82C1A6A8: 390AAE68  addi r8, r10, -0x5198
	ctx.r[8].s64 = ctx.r[10].s64 + -20888;
	// 82C1A6AC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1A6B0: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82C1A6B4: 911F0050  stw r8, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82C1A6B8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1A6BC: 480334E5  bl 0x82c4dba0
	ctx.lr = 0x82C1A6C0;
	sub_82C4DBA0(ctx, base);
	// 82C1A6C0: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C1A6C4: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82C1A6C8: 38C7AE40  addi r6, r7, -0x51c0
	ctx.r[6].s64 = ctx.r[7].s64 + -20928;
	// 82C1A6CC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1A6D0: 90DF0024  stw r6, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 82C1A6D4: 480334CD  bl 0x82c4dba0
	ctx.lr = 0x82C1A6D8;
	sub_82C4DBA0(ctx, base);
	// 82C1A6D8: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C1A6DC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1A6E0: 3885A37C  addi r4, r5, -0x5c84
	ctx.r[4].s64 = ctx.r[5].s64 + -23684;
	// 82C1A6E4: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82C1A6E8: 4B5FA6F1  bl 0x82214dd8
	ctx.lr = 0x82C1A6EC;
	sub_82214DD8(ctx, base);
	// 82C1A6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A6F0: 4BFEB411  bl 0x82c05b00
	ctx.lr = 0x82C1A6F4;
	sub_82C05B00(ctx, base);
	// 82C1A6F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1A6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A708 size=80
    let mut pc: u32 = 0x82C1A708;
    'dispatch: loop {
        match pc {
            0x82C1A708 => {
    //   block [0x82C1A708..0x82C1A758)
	// 82C1A708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1A714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A71C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1A724: 4BFFF6CD  bl 0x82c19df0
	ctx.lr = 0x82C1A728;
	sub_82C19DF0(ctx, base);
	// 82C1A728: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1A72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A730: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1A734: 419A000C  beq cr6, 0x82c1a740
	if ctx.cr[6].eq {
	pc = 0x82C1A740; continue 'dispatch;
	}
	// 82C1A738: 4BC2B079  bl 0x828457b0
	ctx.lr = 0x82C1A73C;
	sub_828457B0(ctx, base);
	// 82C1A73C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1A744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A74C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1A750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1A758 size=412
    let mut pc: u32 = 0x82C1A758;
    'dispatch: loop {
        match pc {
            0x82C1A758 => {
    //   block [0x82C1A758..0x82C1A8F4)
	// 82C1A758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A760: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1A764: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A768: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82C1A76C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A770: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A774: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1A778: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1A77C: 394BAEB8  addi r10, r11, -0x5148
	ctx.r[10].s64 = ctx.r[11].s64 + -20808;
	// 82C1A780: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1A784: 4B5DD07D  bl 0x821f7800
	ctx.lr = 0x82C1A788;
	sub_821F7800(ctx, base);
	// 82C1A788: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C1A78C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C1A790: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C1A794: 38E9AE24  addi r7, r9, -0x51dc
	ctx.r[7].s64 = ctx.r[9].s64 + -20956;
	// 82C1A798: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C1A79C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C1A7A0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C1A7A4: 397F003C  addi r11, r31, 0x3c
	ctx.r[11].s64 = ctx.r[31].s64 + 60;
	// 82C1A7A8: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C1A7AC: C3E80C18  lfs f31, 0xc18(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C1A7B0: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82C1A7B4: 38A6ADCC  addi r5, r6, -0x5234
	ctx.r[5].s64 = ctx.r[6].s64 + -21044;
	// 82C1A7B8: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C1A7BC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1A7C0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C1A7C4: D3FF0020  stfs f31, 0x20(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82C1A7C8: D3FF0024  stfs f31, 0x24(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C1A7CC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82C1A7D0: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82C1A7D4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82C1A7D8: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82C1A7DC: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C1A7E0: 90BF003C  stw r5, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[5].u32 ) };
	// 82C1A7E4: 480332C5  bl 0x82c4daa8
	ctx.lr = 0x82C1A7E8;
	sub_82C4DAA8(ctx, base);
	// 82C1A7E8: 3C808333  lis r4, -0x7ccd
	ctx.r[4].s64 = -2093809664;
	// 82C1A7EC: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 82C1A7F0: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C1A7F4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C1A7F8: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C1A7FC: 3903ADCC  addi r8, r3, -0x5234
	ctx.r[8].s64 = ctx.r[3].s64 + -21044;
	// 82C1A800: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82C1A804: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C1A808: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82C1A80C: 81644EF4  lwz r11, 0x4ef4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1A810: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82C1A814: 911F003C  stw r8, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[8].u32 ) };
	// 82C1A818: 395F0050  addi r10, r31, 0x50
	ctx.r[10].s64 = ctx.r[31].s64 + 80;
	// 82C1A81C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82C1A820: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1A824: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82C1A828: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82C1A82C: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82C1A830: 13E03C07  vcmpneb. (lvlx128) v31, v0, v7
	tmp.u32 = ctx.r[7].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C1A834: D3FF0084  stfs f31, 0x84(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82C1A838: 93DF0090  stw r30, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82C1A83C: D3FF0088  stfs f31, 0x88(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82C1A840: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82C1A844: D01F008C  stfs f0, 0x8c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82C1A848: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 82C1A84C: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 82C1A850: 13C03407  vcmpneb. (lvlx128) v30, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C1A854: FBDF00A0  std r30, 0xa0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u64 ) };
	// 82C1A858: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82C1A85C: FBDF00A8  std r30, 0xa8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u64 ) };
	// 82C1A860: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82C1A864: FBDF00B0  std r30, 0xb0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u64 ) };
	// 82C1A868: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 82C1A86C: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 82C1A870: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82C1A874: 80EB0034  lwz r7, 0x34(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C1A878: 90FF00C4  stw r7, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[7].u32 ) };
	// 82C1A87C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C1A880: 38690001  addi r3, r9, 1
	ctx.r[3].s64 = ctx.r[9].s64 + 1;
	// 82C1A884: 906B0034  stw r3, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82C1A888: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 82C1A88C: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 82C1A890: 9BDF00D0  stb r30, 0xd0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u8 ) };
	// 82C1A894: 9BDF00D1  stb r30, 0xd1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(209 as u32), ctx.r[30].u8 ) };
	// 82C1A898: 9BDF00D2  stb r30, 0xd2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(210 as u32), ctx.r[30].u8 ) };
	// 82C1A89C: 9BDF00D3  stb r30, 0xd3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(211 as u32), ctx.r[30].u8 ) };
	// 82C1A8A0: 9BDF00D4  stb r30, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u8 ) };
	// 82C1A8A4: 9BDF00D5  stb r30, 0xd5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(213 as u32), ctx.r[30].u8 ) };
	// 82C1A8A8: 9BDF00D6  stb r30, 0xd6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[30].u8 ) };
	// 82C1A8AC: 9BDF00D7  stb r30, 0xd7(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(215 as u32), ctx.r[30].u8 ) };
	// 82C1A8B0: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82C1A8B4: 13A02C07  vcmpneb. (lvlx128) v29, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A8F8 size=96
    let mut pc: u32 = 0x82C1A8F8;
    'dispatch: loop {
        match pc {
            0x82C1A8F8 => {
    //   block [0x82C1A8F8..0x82C1A958)
	// 82C1A8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1A904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A90C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A910: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1A914: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1A918: 394BADCC  addi r10, r11, -0x5234
	ctx.r[10].s64 = ctx.r[11].s64 + -21044;
	// 82C1A91C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1A920: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1A924: 4803327D  bl 0x82c4dba0
	ctx.lr = 0x82C1A928;
	sub_82C4DBA0(ctx, base);
	// 82C1A928: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1A92C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A930: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1A934: 419A000C  beq cr6, 0x82c1a940
	if ctx.cr[6].eq {
	pc = 0x82C1A940; continue 'dispatch;
	}
	// 82C1A938: 4BC2AE79  bl 0x828457b0
	ctx.lr = 0x82C1A93C;
	sub_828457B0(ctx, base);
	// 82C1A93C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1A944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A94C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1A950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A958 size=144
    let mut pc: u32 = 0x82C1A958;
    'dispatch: loop {
        match pc {
            0x82C1A958 => {
    //   block [0x82C1A958..0x82C1A9E8)
	// 82C1A958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A960: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A968: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A96C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1A970: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1A974: 392BAEB8  addi r9, r11, -0x5148
	ctx.r[9].s64 = ctx.r[11].s64 + -20808;
	// 82C1A978: 390AADCC  addi r8, r10, -0x5234
	ctx.r[8].s64 = ctx.r[10].s64 + -21044;
	// 82C1A97C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1A980: 397F003C  addi r11, r31, 0x3c
	ctx.r[11].s64 = ctx.r[31].s64 + 60;
	// 82C1A984: 911F003C  stw r8, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[8].u32 ) };
	// 82C1A988: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1A98C: 48033215  bl 0x82c4dba0
	ctx.lr = 0x82C1A990;
	sub_82C4DBA0(ctx, base);
	// 82C1A990: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C1A994: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1A998: 419A0020  beq cr6, 0x82c1a9b8
	if ctx.cr[6].eq {
	pc = 0x82C1A9B8; continue 'dispatch;
	}
	// 82C1A99C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1A9A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1A9A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1A9A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1A9AC: 4E800421  bctrl
	ctx.lr = 0x82C1A9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1A9B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C1A9B4: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82C1A9B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1A9BC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1A9C0: 394BA37C  addi r10, r11, -0x5c84
	ctx.r[10].s64 = ctx.r[11].s64 + -23684;
	// 82C1A9C4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82C1A9C8: 4B5FA411  bl 0x82214dd8
	ctx.lr = 0x82C1A9CC;
	sub_82214DD8(ctx, base);
	// 82C1A9CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1A9D0: 4BFEB141  bl 0x82c05b10
	ctx.lr = 0x82C1A9D4;
	sub_82C05B10(ctx, base);
	// 82C1A9D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1A9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1A9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1A9E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1A9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1A9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1A9E8 size=72
    let mut pc: u32 = 0x82C1A9E8;
    'dispatch: loop {
        match pc {
            0x82C1A9E8 => {
    //   block [0x82C1A9E8..0x82C1AA30)
	// 82C1A9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1A9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1A9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1A9F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1A9F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1A9FC: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1AA00: 48038D89  bl 0x82c53788
	ctx.lr = 0x82C1AA04;
	sub_82C53788(ctx, base);
	// 82C1AA04: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1AA08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C1AA0C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82C1AA10: 480390B9  bl 0x82c53ac8
	ctx.lr = 0x82C1AA14;
	sub_82C53AC8(ctx, base);
	// 82C1AA14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1AA18: 4BFFF771  bl 0x82c1a188
	ctx.lr = 0x82C1AA1C;
	sub_82C1A188(ctx, base);
	// 82C1AA1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1AA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1AA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1AA28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1AA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1AA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1AA30 size=412
    let mut pc: u32 = 0x82C1AA30;
    'dispatch: loop {
        match pc {
            0x82C1AA30 => {
    //   block [0x82C1AA30..0x82C1ABCC)
	// 82C1AA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1AA34: 4808E9C1  bl 0x82ca93f4
	ctx.lr = 0x82C1AA38;
	sub_82CA93D0(ctx, base);
	// 82C1AA38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1AA3C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82C1AA40: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82C1AA44: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82C1AA48: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C1AA4C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C1AA50: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82C1AA54: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82C1AA58: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82C1AA5C: 4B6047FD  bl 0x8221f258
	ctx.lr = 0x82C1AA60;
	sub_8221F258(ctx, base);
	// 82C1AA60: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82C1AA64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AA68: 419A0010  beq cr6, 0x82c1aa78
	if ctx.cr[6].eq {
	pc = 0x82C1AA78; continue 'dispatch;
	}
	// 82C1AA6C: 4BFFFCED  bl 0x82c1a758
	ctx.lr = 0x82C1AA70;
	sub_82C1A758(ctx, base);
	// 82C1AA70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1AA74: 48000008  b 0x82c1aa7c
	pc = 0x82C1AA7C; continue 'dispatch;
	// 82C1AA78: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82C1AA7C: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1AA80: 93FF0018  stw r31, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82C1AA84: 57CAFFFE  rlwinm r10, r30, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1AA88: 93BF00C8  stw r29, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[29].u32 ) };
	// 82C1AA8C: 57C9EFFE  rlwinm r9, r30, 0x1d, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	// 82C1AA90: 939F00CC  stw r28, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[28].u32 ) };
	// 82C1AA94: 57C8E7FE  rlwinm r8, r30, 0x1c, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	// 82C1AA98: 997F00D0  stb r11, 0xd0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u8 ) };
	// 82C1AA9C: 995F00D1  stb r10, 0xd1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(209 as u32), ctx.r[10].u8 ) };
	// 82C1AAA0: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82C1AAA4: 993F00D4  stb r9, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[9].u8 ) };
	// 82C1AAA8: 991F00D6  stb r8, 0xd6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[8].u8 ) };
	// 82C1AAAC: 4B6047AD  bl 0x8221f258
	ctx.lr = 0x82C1AAB0;
	sub_8221F258(ctx, base);
	// 82C1AAB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AAB4: 419A000C  beq cr6, 0x82c1aac0
	if ctx.cr[6].eq {
	pc = 0x82C1AAC0; continue 'dispatch;
	}
	// 82C1AAB8: 480390A9  bl 0x82c53b60
	ctx.lr = 0x82C1AABC;
	sub_82C53B60(ctx, base);
	// 82C1AABC: 48000008  b 0x82c1aac4
	pc = 0x82C1AAC4; continue 'dispatch;
	// 82C1AAC0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C1AAC4: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82C1AAC8: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82C1AACC: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 82C1AAD0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C1AAD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C1AAD8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C1AADC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C1AAE0: 9963001E  stb r11, 0x1e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u8 ) };
	// 82C1AAE4: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1AAE8: 893F00D6  lbz r9, 0xd6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(214 as u32) ) } as u64;
	// 82C1AAEC: 992A001F  stb r9, 0x1f(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(31 as u32), ctx.r[9].u8 ) };
	// 82C1AAF0: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1AAF4: 4803878D  bl 0x82c53280
	ctx.lr = 0x82C1AAF8;
	sub_82C53280(ctx, base);
	// 82C1AAF8: 891F00D0  lbz r8, 0xd0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82C1AAFC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C1AB00: 419A006C  beq cr6, 0x82c1ab6c
	if ctx.cr[6].eq {
	pc = 0x82C1AB6C; continue 'dispatch;
	}
	// 82C1AB04: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82C1AB08: 4B604751  bl 0x8221f258
	ctx.lr = 0x82C1AB0C;
	sub_8221F258(ctx, base);
	// 82C1AB0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AB10: 419A0020  beq cr6, 0x82c1ab30
	if ctx.cr[6].eq {
	pc = 0x82C1AB30; continue 'dispatch;
	}
	// 82C1AB14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1AB18: 93030004  stw r24, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82C1AB1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C1AB20: 93030008  stw r24, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82C1AB24: 392AADB4  addi r9, r10, -0x524c
	ctx.r[9].s64 = ctx.r[10].s64 + -21068;
	// 82C1AB28: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1AB2C: 48000008  b 0x82c1ab34
	pc = 0x82C1AB34; continue 'dispatch;
	// 82C1AB30: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82C1AB34: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82C1AB38: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82C1AB3C: 4B60471D  bl 0x8221f258
	ctx.lr = 0x82C1AB40;
	sub_8221F258(ctx, base);
	// 82C1AB40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AB44: 419A000C  beq cr6, 0x82c1ab50
	if ctx.cr[6].eq {
	pc = 0x82C1AB50; continue 'dispatch;
	}
	// 82C1AB48: 4BF2AF99  bl 0x82b45ae0
	ctx.lr = 0x82C1AB4C;
	sub_82B45AE0(ctx, base);
	// 82C1AB4C: 48000008  b 0x82c1ab54
	pc = 0x82C1AB54; continue 'dispatch;
	// 82C1AB50: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82C1AB54: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C1AB58: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C1AB5C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C1AB60: 808A0004  lwz r4, 4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1AB64: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1AB68: 48038619  bl 0x82c53180
	ctx.lr = 0x82C1AB6C;
	sub_82C53180(ctx, base);
	// 82C1AB6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1AB70: 38770004  addi r3, r23, 4
	ctx.r[3].s64 = ctx.r[23].s64 + 4;
	// 82C1AB74: 4BFFEF0D  bl 0x82c19a80
	ctx.lr = 0x82C1AB78;
	sub_82C19A80(ctx, base);
	// 82C1AB78: 37DF0014  addic. r30, r31, 0x14
	ctx.xer.ca = (ctx.r[31].u32 > (!(20 as u32)));
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C1AB7C: 93190000  stw r24, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82C1AB80: 93190004  stw r24, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82C1AB84: 41820018  beq 0x82c1ab9c
	if ctx.cr[0].eq {
	pc = 0x82C1AB9C; continue 'dispatch;
	}
	// 82C1AB88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1AB8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1AB90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1AB94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1AB98: 4E800421  bctrl
	ctx.lr = 0x82C1AB9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1AB9C: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1ABA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1ABA4: 419A0014  beq cr6, 0x82c1abb8
	if ctx.cr[6].eq {
	pc = 0x82C1ABB8; continue 'dispatch;
	}
	// 82C1ABA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1ABAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1ABB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1ABB4: 4E800421  bctrl
	ctx.lr = 0x82C1ABB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1ABB8: 93D90004  stw r30, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C1ABBC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C1ABC0: 93F90000  stw r31, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C1ABC4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C1ABC8: 4808E87C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1ABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1ABD0 size=216
    let mut pc: u32 = 0x82C1ABD0;
    'dispatch: loop {
        match pc {
            0x82C1ABD0 => {
    //   block [0x82C1ABD0..0x82C1ACA8)
	// 82C1ABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1ABD4: 4808E835  bl 0x82ca9408
	ctx.lr = 0x82C1ABD8;
	sub_82CA93D0(ctx, base);
	// 82C1ABD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1ABDC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1ABE0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C1ABE4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82C1ABE8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C1ABEC: 3B9D003C  addi r28, r29, 0x3c
	ctx.r[28].s64 = ctx.r[29].s64 + 60;
	// 82C1ABF0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1ABF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C1ABF8: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82C1ABFC: 480325DD  bl 0x82c4d1d8
	ctx.lr = 0x82C1AC00;
	sub_82C4D1D8(ctx, base);
	// 82C1AC00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AC04: 419A0024  beq cr6, 0x82c1ac28
	if ctx.cr[6].eq {
	pc = 0x82C1AC28; continue 'dispatch;
	}
	// 82C1AC08: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1AC0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C1AC10: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1AC14: 809E0060  lwz r4, 0x60(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AC18: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1AC1C: 4803879D  bl 0x82c533b8
	ctx.lr = 0x82C1AC20;
	sub_82C533B8(ctx, base);
	// 82C1AC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1AC24: 4808E834  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C1AC28: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82C1AC2C: 4B60462D  bl 0x8221f258
	ctx.lr = 0x82C1AC30;
	sub_8221F258(ctx, base);
	// 82C1AC30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1AC34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1AC38: 419A0028  beq cr6, 0x82c1ac60
	if ctx.cr[6].eq {
	pc = 0x82C1AC60; continue 'dispatch;
	}
	// 82C1AC3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1AC40: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C1AC44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1AC48: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1AC4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1AC50: 48032E41  bl 0x82c4da90
	ctx.lr = 0x82C1AC54;
	sub_82C4DA90(ctx, base);
	// 82C1AC54: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82C1AC58: 48032E39  bl 0x82c4da90
	ctx.lr = 0x82C1AC5C;
	sub_82C4DA90(ctx, base);
	// 82C1AC5C: 48000008  b 0x82c1ac64
	pc = 0x82C1AC64; continue 'dispatch;
	// 82C1AC60: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C1AC64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1AC68: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C1AC6C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C1AC70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1AC74: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1AC78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C1AC7C: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82C1AC80: 48032641  bl 0x82c4d2c0
	ctx.lr = 0x82C1AC84;
	sub_82C4D2C0(ctx, base);
	// 82C1AC84: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 82C1AC88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1AC8C: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C1AC90: 48032631  bl 0x82c4d2c0
	ctx.lr = 0x82C1AC94;
	sub_82C4D2C0(ctx, base);
	// 82C1AC94: 809E0060  lwz r4, 0x60(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AC98: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1AC9C: 48038775  bl 0x82c53410
	ctx.lr = 0x82C1ACA0;
	sub_82C53410(ctx, base);
	// 82C1ACA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1ACA4: 4808E7B4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1ACA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1ACA8 size=164
    let mut pc: u32 = 0x82C1ACA8;
    'dispatch: loop {
        match pc {
            0x82C1ACA8 => {
    //   block [0x82C1ACA8..0x82C1AD4C)
	// 82C1ACA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1ACAC: 4808E75D  bl 0x82ca9408
	ctx.lr = 0x82C1ACB0;
	sub_82CA93D0(ctx, base);
	// 82C1ACB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1ACB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C1ACB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C1ACBC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C1ACC0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C1ACC4: 3BDC003C  addi r30, r28, 0x3c
	ctx.r[30].s64 = ctx.r[28].s64 + 60;
	// 82C1ACC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1ACCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1ACD0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C1ACD4: 48032505  bl 0x82c4d1d8
	ctx.lr = 0x82C1ACD8;
	sub_82C4D1D8(ctx, base);
	// 82C1ACD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1ACDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1ACE0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C1ACE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1ACE8: 41820018  beq 0x82c1ad00
	if ctx.cr[0].eq {
	pc = 0x82C1AD00; continue 'dispatch;
	}
	// 82C1ACEC: 809D0060  lwz r4, 0x60(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1ACF0: 807C0048  lwz r3, 0x48(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1ACF4: 480387CD  bl 0x82c534c0
	ctx.lr = 0x82C1ACF8;
	sub_82C534C0(ctx, base);
	// 82C1ACF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1ACFC: 4808E75C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C1AD00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1AD04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1AD08: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C1AD0C: 48032D2D  bl 0x82c4da38
	ctx.lr = 0x82C1AD10;
	sub_82C4DA38(ctx, base);
	// 82C1AD10: 389D0024  addi r4, r29, 0x24
	ctx.r[4].s64 = ctx.r[29].s64 + 36;
	// 82C1AD14: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1AD18: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C1AD1C: 48032D1D  bl 0x82c4da38
	ctx.lr = 0x82C1AD20;
	sub_82C4DA38(ctx, base);
	// 82C1AD20: 809D0060  lwz r4, 0x60(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AD24: 807C0048  lwz r3, 0x48(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1AD28: 48038741  bl 0x82c53468
	ctx.lr = 0x82C1AD2C;
	sub_82C53468(ctx, base);
	// 82C1AD2C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82C1AD30: 4BF7C149  bl 0x82b96e78
	ctx.lr = 0x82C1AD34;
	sub_82B96E78(ctx, base);
	// 82C1AD34: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C1AD38: 4BF7C141  bl 0x82b96e78
	ctx.lr = 0x82C1AD3C;
	sub_82B96E78(ctx, base);
	// 82C1AD3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1AD40: 4BC2AA71  bl 0x828457b0
	ctx.lr = 0x82C1AD44;
	sub_828457B0(ctx, base);
	// 82C1AD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1AD48: 4808E710  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1AD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1AD50 size=360
    let mut pc: u32 = 0x82C1AD50;
    'dispatch: loop {
        match pc {
            0x82C1AD50 => {
    //   block [0x82C1AD50..0x82C1AEB8)
	// 82C1AD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1AD54: 4808E6A9  bl 0x82ca93fc
	ctx.lr = 0x82C1AD58;
	sub_82CA93D0(ctx, base);
	// 82C1AD58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1AD5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1AD60: 3B3F0054  addi r25, r31, 0x54
	ctx.r[25].s64 = ctx.r[31].s64 + 84;
	// 82C1AD64: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C1AD68: 48032251  bl 0x82c4cfb8
	ctx.lr = 0x82C1AD6C;
	sub_82C4CFB8(ctx, base);
	// 82C1AD6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AD70: 419A00B8  beq cr6, 0x82c1ae28
	if ctx.cr[6].eq {
	pc = 0x82C1AE28; continue 'dispatch;
	}
	// 82C1AD74: 3B7F0050  addi r27, r31, 0x50
	ctx.r[27].s64 = ctx.r[31].s64 + 80;
	// 82C1AD78: 3B5B0004  addi r26, r27, 4
	ctx.r[26].s64 = ctx.r[27].s64 + 4;
	// 82C1AD7C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1AD80: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C1AD84: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1AD88: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C1AD8C: 387B0004  addi r3, r27, 4
	ctx.r[3].s64 = ctx.r[27].s64 + 4;
	// 82C1AD90: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C1AD94: 48032445  bl 0x82c4d1d8
	ctx.lr = 0x82C1AD98;
	sub_82C4D1D8(ctx, base);
	// 82C1AD98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C1AD9C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C1ADA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C1ADA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C1ADA8: 48032C91  bl 0x82c4da38
	ctx.lr = 0x82C1ADAC;
	sub_82C4DA38(ctx, base);
	// 82C1ADAC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C1ADB0: 419A0014  beq cr6, 0x82c1adc4
	if ctx.cr[6].eq {
	pc = 0x82C1ADC4; continue 'dispatch;
	}
	// 82C1ADB4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C1ADB8: 4BF7C0C1  bl 0x82b96e78
	ctx.lr = 0x82C1ADBC;
	sub_82B96E78(ctx, base);
	// 82C1ADBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1ADC0: 4BC2A9F1  bl 0x828457b0
	ctx.lr = 0x82C1ADC4;
	sub_828457B0(ctx, base);
	// 82C1ADC4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82C1ADC8: 3BDD0050  addi r30, r29, 0x50
	ctx.r[30].s64 = ctx.r[29].s64 + 80;
	// 82C1ADCC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82C1ADD0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82C1ADD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1ADD8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C1ADDC: 480323FD  bl 0x82c4d1d8
	ctx.lr = 0x82C1ADE0;
	sub_82C4D1D8(ctx, base);
	// 82C1ADE0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C1ADE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1ADE8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C1ADEC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C1ADF0: 48032C49  bl 0x82c4da38
	ctx.lr = 0x82C1ADF4;
	sub_82C4DA38(ctx, base);
	// 82C1ADF4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C1ADF8: 419A0014  beq cr6, 0x82c1ae0c
	if ctx.cr[6].eq {
	pc = 0x82C1AE0C; continue 'dispatch;
	}
	// 82C1ADFC: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82C1AE00: 4BF7C079  bl 0x82b96e78
	ctx.lr = 0x82C1AE04;
	sub_82B96E78(ctx, base);
	// 82C1AE04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C1AE08: 4BC2A9A9  bl 0x828457b0
	ctx.lr = 0x82C1AE0C;
	sub_828457B0(ctx, base);
	// 82C1AE0C: 809D0060  lwz r4, 0x60(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AE10: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AE14: 480380DD  bl 0x82c52ef0
	ctx.lr = 0x82C1AE18;
	sub_82C52EF0(ctx, base);
	// 82C1AE18: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C1AE1C: 4803219D  bl 0x82c4cfb8
	ctx.lr = 0x82C1AE20;
	sub_82C4CFB8(ctx, base);
	// 82C1AE20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AE24: 409AFF58  bne cr6, 0x82c1ad7c
	if !ctx.cr[6].eq {
	pc = 0x82C1AD7C; continue 'dispatch;
	}
	// 82C1AE28: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 82C1AE2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1AE30: 48032189  bl 0x82c4cfb8
	ctx.lr = 0x82C1AE34;
	sub_82C4CFB8(ctx, base);
	// 82C1AE34: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C1AE38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AE3C: 419A0028  beq cr6, 0x82c1ae64
	if ctx.cr[6].eq {
	pc = 0x82C1AE64; continue 'dispatch;
	}
	// 82C1AE40: 817E4EF4  lwz r11, 0x4ef4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1AE44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1AE48: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1AE4C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C1AE50: 4BFFFE59  bl 0x82c1aca8
	ctx.lr = 0x82C1AE54;
	sub_82C1ACA8(ctx, base);
	// 82C1AE54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1AE58: 48032161  bl 0x82c4cfb8
	ctx.lr = 0x82C1AE5C;
	sub_82C4CFB8(ctx, base);
	// 82C1AE5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1AE60: 409AFFE0  bne cr6, 0x82c1ae40
	if !ctx.cr[6].eq {
	pc = 0x82C1AE40; continue 'dispatch;
	}
	// 82C1AE64: 817E4EF4  lwz r11, 0x4ef4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1AE68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1AE6C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82C1AE70: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1AE74: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C1AE78: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82C1AE7C: 4BFFE6C5  bl 0x82c19540
	ctx.lr = 0x82C1AE80;
	sub_82C19540(ctx, base);
	// 82C1AE80: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AE84: 48037F9D  bl 0x82c52e20
	ctx.lr = 0x82C1AE88;
	sub_82C52E20(ctx, base);
	// 82C1AE88: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C1AE8C: 48038B8D  bl 0x82c53a18
	ctx.lr = 0x82C1AE90;
	sub_82C53A18(ctx, base);
	// 82C1AE90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C1AE94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1AE98: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82C1AE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1AEA0: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1AEA4: 80E8001C  lwz r7, 0x1c(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1AEA8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C1AEAC: 4E800421  bctrl
	ctx.lr = 0x82C1AEB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1AEB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C1AEB4: 4808E598  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1AEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1AEB8 size=256
    let mut pc: u32 = 0x82C1AEB8;
    'dispatch: loop {
        match pc {
            0x82C1AEB8 => {
    //   block [0x82C1AEB8..0x82C1AFB8)
	// 82C1AEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1AEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1AEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1AEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1AEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1AECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1AED0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1AED4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1AED8: 394BAE90  addi r10, r11, -0x5170
	ctx.r[10].s64 = ctx.r[11].s64 + -20848;
	// 82C1AEDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1AEE0: 4B5DC921  bl 0x821f7800
	ctx.lr = 0x82C1AEE4;
	sub_821F7800(ctx, base);
	// 82C1AEE4: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 82C1AEE8: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82C1AEEC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C1AEF0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1AEF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C1AEF8: 38E8ADF4  addi r7, r8, -0x520c
	ctx.r[7].s64 = ctx.r[8].s64 + -21004;
	// 82C1AEFC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C1AF00: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C1AF04: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C1AF08: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C1AF0C: 38A6AE40  addi r5, r6, -0x51c0
	ctx.r[5].s64 = ctx.r[6].s64 + -20928;
	// 82C1AF10: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82C1AF14: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C1AF18: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C1AF1C: 81694EF4  lwz r11, 0x4ef4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1AF20: 808B0030  lwz r4, 0x30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C1AF24: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82C1AF28: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C1AF2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C1AF30: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82C1AF34: 90BF0024  stw r5, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82C1AF38: 48032B71  bl 0x82c4daa8
	ctx.lr = 0x82C1AF3C;
	sub_82C4DAA8(ctx, base);
	// 82C1AF3C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C1AF40: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C1AF44: 38E9AE40  addi r7, r9, -0x51c0
	ctx.r[7].s64 = ctx.r[9].s64 + -20928;
	// 82C1AF48: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C1AF4C: 90FF0024  stw r7, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82C1AF50: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82C1AF54: 38A6AE68  addi r5, r6, -0x5198
	ctx.r[5].s64 = ctx.r[6].s64 + -20888;
	// 82C1AF58: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C1AF5C: C0080C14  lfs f0, 0xc14(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1AF60: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82C1AF64: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82C1AF68: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1AF6C: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82C1AF70: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82C1AF74: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82C1AF78: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82C1AF7C: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82C1AF80: 90BF0050  stw r5, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82C1AF84: 48032B25  bl 0x82c4daa8
	ctx.lr = 0x82C1AF88;
	sub_82C4DAA8(ctx, base);
	// 82C1AF88: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82C1AF8C: 3864AE68  addi r3, r4, -0x5198
	ctx.r[3].s64 = ctx.r[4].s64 + -20888;
	// 82C1AF90: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82C1AF94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1AF98: 9BDF005C  stb r30, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u8 ) };
	// 82C1AF9C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82C1AFA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1AFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1AFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1AFAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1AFB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1AFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1AFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1AFB8 size=80
    let mut pc: u32 = 0x82C1AFB8;
    'dispatch: loop {
        match pc {
            0x82C1AFB8 => {
    //   block [0x82C1AFB8..0x82C1B008)
	// 82C1AFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1AFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1AFC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1AFC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1AFC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1AFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1AFD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1AFD4: 4BFFF6B5  bl 0x82c1a688
	ctx.lr = 0x82C1AFD8;
	sub_82C1A688(ctx, base);
	// 82C1AFD8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1AFDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1AFE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1AFE4: 419A000C  beq cr6, 0x82c1aff0
	if ctx.cr[6].eq {
	pc = 0x82C1AFF0; continue 'dispatch;
	}
	// 82C1AFE8: 4BC2A7C9  bl 0x828457b0
	ctx.lr = 0x82C1AFEC;
	sub_828457B0(ctx, base);
	// 82C1AFEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1AFF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1AFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1AFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1AFFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B008 size=176
    let mut pc: u32 = 0x82C1B008;
    'dispatch: loop {
        match pc {
            0x82C1B008 => {
    //   block [0x82C1B008..0x82C1B0B8)
	// 82C1B008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B00C: 4808E401  bl 0x82ca940c
	ctx.lr = 0x82C1B010;
	sub_82CA93D0(ctx, base);
	// 82C1B010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B018: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
	// 82C1B01C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1B020: 48031F99  bl 0x82c4cfb8
	ctx.lr = 0x82C1B024;
	sub_82C4CFB8(ctx, base);
	// 82C1B024: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C1B028: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1B02C: 419A0028  beq cr6, 0x82c1b054
	if ctx.cr[6].eq {
	pc = 0x82C1B054; continue 'dispatch;
	}
	// 82C1B030: 817E4EF4  lwz r11, 0x4ef4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1B034: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1B038: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B03C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C1B040: 4BFFFC69  bl 0x82c1aca8
	ctx.lr = 0x82C1B044;
	sub_82C1ACA8(ctx, base);
	// 82C1B044: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1B048: 48031F71  bl 0x82c4cfb8
	ctx.lr = 0x82C1B04C;
	sub_82C4CFB8(ctx, base);
	// 82C1B04C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1B050: 409AFFE0  bne cr6, 0x82c1b030
	if !ctx.cr[6].eq {
	pc = 0x82C1B030; continue 'dispatch;
	}
	// 82C1B054: 817E4EF4  lwz r11, 0x4ef4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20212 as u32) ) } as u64;
	// 82C1B058: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1B05C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1B060: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1B064: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C1B068: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C1B06C: 4BFFE56D  bl 0x82c195d8
	ctx.lr = 0x82C1B070;
	sub_82C195D8(ctx, base);
	// 82C1B070: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1B074: 48037E15  bl 0x82c52e88
	ctx.lr = 0x82C1B078;
	sub_82C52E88(ctx, base);
	// 82C1B078: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C1B07C: 4803899D  bl 0x82c53a18
	ctx.lr = 0x82C1B080;
	sub_82C53A18(ctx, base);
	// 82C1B080: 893F00D0  lbz r9, 0xd0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82C1B084: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B088: 419A0010  beq cr6, 0x82c1b098
	if ctx.cr[6].eq {
	pc = 0x82C1B098; continue 'dispatch;
	}
	// 82C1B08C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C1B090: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B094: 48038985  bl 0x82c53a18
	ctx.lr = 0x82C1B098;
	sub_82C53A18(ctx, base);
	// 82C1B098: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B09C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1B0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B0A4: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C1B0A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B0AC: 4E800421  bctrl
	ctx.lr = 0x82C1B0B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B0B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B0B4: 4808E3A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B0B8 size=80
    let mut pc: u32 = 0x82C1B0B8;
    'dispatch: loop {
        match pc {
            0x82C1B0B8 => {
    //   block [0x82C1B0B8..0x82C1B108)
	// 82C1B0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B0C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B0C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B0C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B0CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B0D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1B0D4: 4BFFF885  bl 0x82c1a958
	ctx.lr = 0x82C1B0D8;
	sub_82C1A958(ctx, base);
	// 82C1B0D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1B0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B0E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B0E4: 419A000C  beq cr6, 0x82c1b0f0
	if ctx.cr[6].eq {
	pc = 0x82C1B0F0; continue 'dispatch;
	}
	// 82C1B0E8: 4BC2A6C9  bl 0x828457b0
	ctx.lr = 0x82C1B0EC;
	sub_828457B0(ctx, base);
	// 82C1B0EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B0F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B0FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B108 size=192
    let mut pc: u32 = 0x82C1B108;
    'dispatch: loop {
        match pc {
            0x82C1B108 => {
    //   block [0x82C1B108..0x82C1B1C8)
	// 82C1B108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B10C: 4808E2FD  bl 0x82ca9408
	ctx.lr = 0x82C1B110;
	sub_82CA93D0(ctx, base);
	// 82C1B110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B114: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C1B118: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82C1B11C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C1B120: 4B604139  bl 0x8221f258
	ctx.lr = 0x82C1B124;
	sub_8221F258(ctx, base);
	// 82C1B124: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C1B128: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1B12C: 419A0010  beq cr6, 0x82c1b13c
	if ctx.cr[6].eq {
	pc = 0x82C1B13C; continue 'dispatch;
	}
	// 82C1B130: 4BFFFD89  bl 0x82c1aeb8
	ctx.lr = 0x82C1B134;
	sub_82C1AEB8(ctx, base);
	// 82C1B134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B138: 48000008  b 0x82c1b140
	pc = 0x82C1B140; continue 'dispatch;
	// 82C1B13C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82C1B140: 93FF000C  stw r31, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82C1B144: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C1B148: 4B604111  bl 0x8221f258
	ctx.lr = 0x82C1B14C;
	sub_8221F258(ctx, base);
	// 82C1B14C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1B150: 419A000C  beq cr6, 0x82c1b15c
	if ctx.cr[6].eq {
	pc = 0x82C1B15C; continue 'dispatch;
	}
	// 82C1B154: 48038A5D  bl 0x82c53bb0
	ctx.lr = 0x82C1B158;
	sub_82C53BB0(ctx, base);
	// 82C1B158: 48000008  b 0x82c1b160
	pc = 0x82C1B160; continue 'dispatch;
	// 82C1B15C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C1B160: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82C1B164: 480381ED  bl 0x82c53350
	ctx.lr = 0x82C1B168;
	sub_82C53350(ctx, base);
	// 82C1B168: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1B16C: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 82C1B170: 4BFFE989  bl 0x82c19af8
	ctx.lr = 0x82C1B174;
	sub_82C19AF8(ctx, base);
	// 82C1B174: 37BF0008  addic. r29, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C1B178: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C1B17C: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C1B180: 41820018  beq 0x82c1b198
	if ctx.cr[0].eq {
	pc = 0x82C1B198; continue 'dispatch;
	}
	// 82C1B184: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B188: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1B18C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B190: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B194: 4E800421  bctrl
	ctx.lr = 0x82C1B198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B198: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B19C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1B1A0: 419A0014  beq cr6, 0x82c1b1b4
	if ctx.cr[6].eq {
	pc = 0x82C1B1B4; continue 'dispatch;
	}
	// 82C1B1A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B1A8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B1AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B1B0: 4E800421  bctrl
	ctx.lr = 0x82C1B1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B1B4: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C1B1B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B1BC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C1B1C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1B1C4: 4808E294  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B1C8 size=16
    let mut pc: u32 = 0x82C1B1C8;
    'dispatch: loop {
        match pc {
            0x82C1B1C8 => {
    //   block [0x82C1B1C8..0x82C1B1D8)
	// 82C1B1C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1B1CC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C1B1D0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1B1D4: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B1D8 size=8
    let mut pc: u32 = 0x82C1B1D8;
    'dispatch: loop {
        match pc {
            0x82C1B1D8 => {
    //   block [0x82C1B1D8..0x82C1B1E0)
	// 82C1B1D8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B1DC: 4BFFFE2C  b 0x82c1b008
	sub_82C1B008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B1E0 size=4
    let mut pc: u32 = 0x82C1B1E0;
    'dispatch: loop {
        match pc {
            0x82C1B1E0 => {
    //   block [0x82C1B1E0..0x82C1B1E4)
	// 82C1B1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1B1E8 size=76
    let mut pc: u32 = 0x82C1B1E8;
    'dispatch: loop {
        match pc {
            0x82C1B1E8 => {
    //   block [0x82C1B1E8..0x82C1B234)
	// 82C1B1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B1F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B1F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B1F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B1FC: 4B5DC605  bl 0x821f7800
	ctx.lr = 0x82C1B200;
	sub_821F7800(ctx, base);
	// 82C1B200: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C1B204: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C1B208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B20C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1B210: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C1B214: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C1B218: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C1B21C: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C1B220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1B224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B22C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B238 size=24
    let mut pc: u32 = 0x82C1B238;
    'dispatch: loop {
        match pc {
            0x82C1B238 => {
    //   block [0x82C1B238..0x82C1B250)
	// 82C1B238: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B23C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1B240: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B244: 409A000C  bne cr6, 0x82c1b250
	if !ctx.cr[6].eq {
		sub_82C1B250(ctx, base);
		return;
	}
	// 82C1B248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1B24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B250 size=12
    let mut pc: u32 = 0x82C1B250;
    'dispatch: loop {
        match pc {
            0x82C1B250 => {
    //   block [0x82C1B250..0x82C1B25C)
	// 82C1B250: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B254: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C1B258: 48009668  b 0x82c248c0
	sub_82C248C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B260 size=24
    let mut pc: u32 = 0x82C1B260;
    'dispatch: loop {
        match pc {
            0x82C1B260 => {
    //   block [0x82C1B260..0x82C1B278)
	// 82C1B260: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B264: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1B268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B26C: 409A000C  bne cr6, 0x82c1b278
	if !ctx.cr[6].eq {
		sub_82C1B278(ctx, base);
		return;
	}
	// 82C1B270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1B274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B278 size=12
    let mut pc: u32 = 0x82C1B278;
    'dispatch: loop {
        match pc {
            0x82C1B278 => {
    //   block [0x82C1B278..0x82C1B284)
	// 82C1B278: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B27C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C1B280: 480096A8  b 0x82c24928
	sub_82C24928(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B288 size=24
    let mut pc: u32 = 0x82C1B288;
    'dispatch: loop {
        match pc {
            0x82C1B288 => {
    //   block [0x82C1B288..0x82C1B2A0)
	// 82C1B288: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B28C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82C1B290: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1B294: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1B298: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B29C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B2A0 size=12
    let mut pc: u32 = 0x82C1B2A0;
    'dispatch: loop {
        match pc {
            0x82C1B2A0 => {
    //   block [0x82C1B2A0..0x82C1B2AC)
	// 82C1B2A0: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1B2A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B2A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B2AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B2AC size=8
    let mut pc: u32 = 0x82C1B2AC;
    'dispatch: loop {
        match pc {
            0x82C1B2AC => {
    //   block [0x82C1B2AC..0x82C1B2B4)
	// 82C1B2AC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B2B0: 48009360  b 0x82c24610
	sub_82C24610(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B2B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1B2B4 size=4
    let mut pc: u32 = 0x82C1B2B4;
    'dispatch: loop {
        match pc {
            0x82C1B2B4 => {
    //   block [0x82C1B2B4..0x82C1B2B8)
	// 82C1B2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B2B8 size=148
    let mut pc: u32 = 0x82C1B2B8;
    'dispatch: loop {
        match pc {
            0x82C1B2B8 => {
    //   block [0x82C1B2B8..0x82C1B34C)
	// 82C1B2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B2D0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1B2D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B2D8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1B2DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B2E0: 4E800421  bctrl
	ctx.lr = 0x82C1B2E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B2E4: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B2E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B2EC: 419A0040  beq cr6, 0x82c1b32c
	if ctx.cr[6].eq {
	pc = 0x82C1B32C; continue 'dispatch;
	}
	// 82C1B2F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1B2F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B2F8: 4B5F8E91  bl 0x82214188
	ctx.lr = 0x82C1B2FC;
	sub_82214188(ctx, base);
	// 82C1B2FC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B300: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B304: 419A0028  beq cr6, 0x82c1b32c
	if ctx.cr[6].eq {
	pc = 0x82C1B32C; continue 'dispatch;
	}
	// 82C1B308: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B310: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1B314: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B318: 4E800421  bctrl
	ctx.lr = 0x82C1B31C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B31C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B320: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B324: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B328: 409A0008  bne cr6, 0x82c1b330
	if !ctx.cr[6].eq {
	pc = 0x82C1B330; continue 'dispatch;
	}
	// 82C1B32C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B330: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B350 size=148
    let mut pc: u32 = 0x82C1B350;
    'dispatch: loop {
        match pc {
            0x82C1B350 => {
    //   block [0x82C1B350..0x82C1B3E4)
	// 82C1B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B35C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B368: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1B36C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B370: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1B374: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B378: 4E800421  bctrl
	ctx.lr = 0x82C1B37C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B37C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B380: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B384: 419A0040  beq cr6, 0x82c1b3c4
	if ctx.cr[6].eq {
	pc = 0x82C1B3C4; continue 'dispatch;
	}
	// 82C1B388: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1B38C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B390: 4BFEBD99  bl 0x82c07128
	ctx.lr = 0x82C1B394;
	sub_82C07128(ctx, base);
	// 82C1B394: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B39C: 419A0028  beq cr6, 0x82c1b3c4
	if ctx.cr[6].eq {
	pc = 0x82C1B3C4; continue 'dispatch;
	}
	// 82C1B3A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B3A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B3A8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1B3AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B3B0: 4E800421  bctrl
	ctx.lr = 0x82C1B3B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B3B4: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B3B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B3BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B3C0: 409A0008  bne cr6, 0x82c1b3c8
	if !ctx.cr[6].eq {
	pc = 0x82C1B3C8; continue 'dispatch;
	}
	// 82C1B3C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B3C8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B3CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B3D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B3DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B3E8 size=148
    let mut pc: u32 = 0x82C1B3E8;
    'dispatch: loop {
        match pc {
            0x82C1B3E8 => {
    //   block [0x82C1B3E8..0x82C1B47C)
	// 82C1B3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B3F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B3F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B3F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B3FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B400: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1B404: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B408: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1B40C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B410: 4E800421  bctrl
	ctx.lr = 0x82C1B414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B414: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B418: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B41C: 419A0040  beq cr6, 0x82c1b45c
	if ctx.cr[6].eq {
	pc = 0x82C1B45C; continue 'dispatch;
	}
	// 82C1B420: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1B424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B428: 4805DBB1  bl 0x82c78fd8
	ctx.lr = 0x82C1B42C;
	sub_82C78FD8(ctx, base);
	// 82C1B42C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B434: 419A0028  beq cr6, 0x82c1b45c
	if ctx.cr[6].eq {
	pc = 0x82C1B45C; continue 'dispatch;
	}
	// 82C1B438: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B43C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B440: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1B444: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B448: 4E800421  bctrl
	ctx.lr = 0x82C1B44C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B44C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B450: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B454: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B458: 409A0008  bne cr6, 0x82c1b460
	if !ctx.cr[6].eq {
	pc = 0x82C1B460; continue 'dispatch;
	}
	// 82C1B45C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B460: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B470: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B480 size=148
    let mut pc: u32 = 0x82C1B480;
    'dispatch: loop {
        match pc {
            0x82C1B480 => {
    //   block [0x82C1B480..0x82C1B514)
	// 82C1B480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B498: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1B49C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B4A0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1B4A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B4A8: 4E800421  bctrl
	ctx.lr = 0x82C1B4AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B4AC: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B4B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B4B4: 419A0040  beq cr6, 0x82c1b4f4
	if ctx.cr[6].eq {
	pc = 0x82C1B4F4; continue 'dispatch;
	}
	// 82C1B4B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1B4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B4C0: 4BFEBC49  bl 0x82c07108
	ctx.lr = 0x82C1B4C4;
	sub_82C07108(ctx, base);
	// 82C1B4C4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B4C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B4CC: 419A0028  beq cr6, 0x82c1b4f4
	if ctx.cr[6].eq {
	pc = 0x82C1B4F4; continue 'dispatch;
	}
	// 82C1B4D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B4D8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1B4DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B4E0: 4E800421  bctrl
	ctx.lr = 0x82C1B4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B4E4: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B4E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B4EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B4F0: 409A0008  bne cr6, 0x82c1b4f8
	if !ctx.cr[6].eq {
	pc = 0x82C1B4F8; continue 'dispatch;
	}
	// 82C1B4F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B4F8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B4FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B508: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B50C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B518 size=148
    let mut pc: u32 = 0x82C1B518;
    'dispatch: loop {
        match pc {
            0x82C1B518 => {
    //   block [0x82C1B518..0x82C1B5AC)
	// 82C1B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B52C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B530: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1B534: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B538: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1B53C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B540: 4E800421  bctrl
	ctx.lr = 0x82C1B544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B544: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B548: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B54C: 419A0040  beq cr6, 0x82c1b58c
	if ctx.cr[6].eq {
	pc = 0x82C1B58C; continue 'dispatch;
	}
	// 82C1B550: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1B554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B558: 4B733859  bl 0x8234edb0
	ctx.lr = 0x82C1B55C;
	sub_8234EDB0(ctx, base);
	// 82C1B55C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B560: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B564: 419A0028  beq cr6, 0x82c1b58c
	if ctx.cr[6].eq {
	pc = 0x82C1B58C; continue 'dispatch;
	}
	// 82C1B568: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B570: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1B574: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B578: 4E800421  bctrl
	ctx.lr = 0x82C1B57C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B57C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B580: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B584: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B588: 409A0008  bne cr6, 0x82c1b590
	if !ctx.cr[6].eq {
	pc = 0x82C1B590; continue 'dispatch;
	}
	// 82C1B58C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B590: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B5A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B5A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B5B0 size=148
    let mut pc: u32 = 0x82C1B5B0;
    'dispatch: loop {
        match pc {
            0x82C1B5B0 => {
    //   block [0x82C1B5B0..0x82C1B644)
	// 82C1B5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B5B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B5BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B5C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B5C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B5C8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C1B5CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B5D0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1B5D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B5D8: 4E800421  bctrl
	ctx.lr = 0x82C1B5DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B5DC: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B5E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B5E4: 419A0040  beq cr6, 0x82c1b624
	if ctx.cr[6].eq {
	pc = 0x82C1B624; continue 'dispatch;
	}
	// 82C1B5E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1B5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B5F0: 4BFEBB29  bl 0x82c07118
	ctx.lr = 0x82C1B5F4;
	sub_82C07118(ctx, base);
	// 82C1B5F4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B5F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B5FC: 419A0028  beq cr6, 0x82c1b624
	if ctx.cr[6].eq {
	pc = 0x82C1B624; continue 'dispatch;
	}
	// 82C1B600: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B608: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1B60C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B610: 4E800421  bctrl
	ctx.lr = 0x82C1B614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B614: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B618: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B61C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1B620: 409A0008  bne cr6, 0x82c1b628
	if !ctx.cr[6].eq {
	pc = 0x82C1B628; continue 'dispatch;
	}
	// 82C1B624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B628: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B62C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B638: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B63C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B648 size=468
    let mut pc: u32 = 0x82C1B648;
    'dispatch: loop {
        match pc {
            0x82C1B648 => {
    //   block [0x82C1B648..0x82C1B81C)
	// 82C1B648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B64C: 4808DDC1  bl 0x82ca940c
	ctx.lr = 0x82C1B650;
	sub_82CA93D0(ctx, base);
	// 82C1B650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B654: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C1B658: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C1B65C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B660: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B664: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1B668: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B66C: 4E800421  bctrl
	ctx.lr = 0x82C1B670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B670: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B674: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C1B678: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C1B67C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B680: 388BAF88  addi r4, r11, -0x5078
	ctx.r[4].s64 = ctx.r[11].s64 + -20600;
	// 82C1B684: 409A0064  bne cr6, 0x82c1b6e8
	if !ctx.cr[6].eq {
	pc = 0x82C1B6E8; continue 'dispatch;
	}
	// 82C1B688: 4BFEBB21  bl 0x82c071a8
	ctx.lr = 0x82C1B68C;
	sub_82C071A8(ctx, base);
	// 82C1B68C: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B690: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B694: 409A0058  bne cr6, 0x82c1b6ec
	if !ctx.cr[6].eq {
	pc = 0x82C1B6EC; continue 'dispatch;
	}
	// 82C1B698: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1B69C: 4B5DC165  bl 0x821f7800
	ctx.lr = 0x82C1B6A0;
	sub_821F7800(ctx, base);
	// 82C1B6A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B6A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1B6A8: 388BAF7C  addi r4, r11, -0x5084
	ctx.r[4].s64 = ctx.r[11].s64 + -20612;
	// 82C1B6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B6B0: 4BFFFC09  bl 0x82c1b2b8
	ctx.lr = 0x82C1B6B4;
	sub_82C1B2B8(ctx, base);
	// 82C1B6B4: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B6B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1B6BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B6C0: 409A0014  bne cr6, 0x82c1b6d4
	if !ctx.cr[6].eq {
	pc = 0x82C1B6D4; continue 'dispatch;
	}
	// 82C1B6C4: 4B5F9715  bl 0x82214dd8
	ctx.lr = 0x82C1B6C8;
	sub_82214DD8(ctx, base);
	// 82C1B6C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1B6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1B6D0: 4808DD8C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C1B6D4: 4B66F305  bl 0x8228a9d8
	ctx.lr = 0x82C1B6D8;
	sub_8228A9D8(ctx, base);
	// 82C1B6D8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C1B6DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1B6E0: 4B5F96F9  bl 0x82214dd8
	ctx.lr = 0x82C1B6E4;
	sub_82214DD8(ctx, base);
	// 82C1B6E4: 48000008  b 0x82c1b6ec
	pc = 0x82C1B6EC; continue 'dispatch;
	// 82C1B6E8: 4BFEBAC1  bl 0x82c071a8
	ctx.lr = 0x82C1B6EC;
	sub_82C071A8(ctx, base);
	// 82C1B6EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B6F0: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 82C1B6F4: 388BAF70  addi r4, r11, -0x5090
	ctx.r[4].s64 = ctx.r[11].s64 + -20624;
	// 82C1B6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B6FC: 4BFEBAAD  bl 0x82c071a8
	ctx.lr = 0x82C1B700;
	sub_82C071A8(ctx, base);
	// 82C1B700: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B704: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B708: 419A0028  beq cr6, 0x82c1b730
	if ctx.cr[6].eq {
	pc = 0x82C1B730; continue 'dispatch;
	}
	// 82C1B70C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B710: 38BE000C  addi r5, r30, 0xc
	ctx.r[5].s64 = ctx.r[30].s64 + 12;
	// 82C1B714: 388BAF68  addi r4, r11, -0x5098
	ctx.r[4].s64 = ctx.r[11].s64 + -20632;
	// 82C1B718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B71C: 4BFFFC35  bl 0x82c1b350
	ctx.lr = 0x82C1B720;
	sub_82C1B350(ctx, base);
	// 82C1B720: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B724: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82C1B728: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B72C: 409A0008  bne cr6, 0x82c1b734
	if !ctx.cr[6].eq {
	pc = 0x82C1B734; continue 'dispatch;
	}
	// 82C1B730: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C1B734: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B73C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1B740: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1B744: 4E800421  bctrl
	ctx.lr = 0x82C1B748;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B748: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C1B74C: 409A0018  bne cr6, 0x82c1b764
	if !ctx.cr[6].eq {
	pc = 0x82C1B764; continue 'dispatch;
	}
	// 82C1B750: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1B754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1B758: 409A000C  bne cr6, 0x82c1b764
	if !ctx.cr[6].eq {
	pc = 0x82C1B764; continue 'dispatch;
	}
	// 82C1B75C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82C1B760: 997E000C  stb r11, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82C1B764: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B768: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 82C1B76C: 388BAF5C  addi r4, r11, -0x50a4
	ctx.r[4].s64 = ctx.r[11].s64 + -20644;
	// 82C1B770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B774: 4BFFFC75  bl 0x82c1b3e8
	ctx.lr = 0x82C1B778;
	sub_82C1B3E8(ctx, base);
	// 82C1B778: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1B77C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B780: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1B784: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C1B788: 4E800421  bctrl
	ctx.lr = 0x82C1B78C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1B78C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1B790: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C1B794: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1B798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B79C: 388AAF4C  addi r4, r10, -0x50b4
	ctx.r[4].s64 = ctx.r[10].s64 + -20660;
	// 82C1B7A0: 409A0028  bne cr6, 0x82c1b7c8
	if !ctx.cr[6].eq {
	pc = 0x82C1B7C8; continue 'dispatch;
	}
	// 82C1B7A4: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82C1B7A8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C1B7AC: 4BFEB9FD  bl 0x82c071a8
	ctx.lr = 0x82C1B7B0;
	sub_82C071A8(ctx, base);
	// 82C1B7B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C1B7B4: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82C1B7B8: 40990008  ble cr6, 0x82c1b7c0
	if !ctx.cr[6].gt {
	pc = 0x82C1B7C0; continue 'dispatch;
	}
	// 82C1B7BC: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82C1B7C0: 997E000A  stb r11, 0xa(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82C1B7C4: 48000010  b 0x82c1b7d4
	pc = 0x82C1B7D4; continue 'dispatch;
	// 82C1B7C8: 897E000A  lbz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82C1B7CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C1B7D0: 4BFEB9D9  bl 0x82c071a8
	ctx.lr = 0x82C1B7D4;
	sub_82C071A8(ctx, base);
	// 82C1B7D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B7D8: 38BE000D  addi r5, r30, 0xd
	ctx.r[5].s64 = ctx.r[30].s64 + 13;
	// 82C1B7DC: 388BAF30  addi r4, r11, -0x50d0
	ctx.r[4].s64 = ctx.r[11].s64 + -20688;
	// 82C1B7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B7E4: 4BFFFC9D  bl 0x82c1b480
	ctx.lr = 0x82C1B7E8;
	sub_82C1B480(ctx, base);
	// 82C1B7E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1B7EC: 38BE000E  addi r5, r30, 0xe
	ctx.r[5].s64 = ctx.r[30].s64 + 14;
	// 82C1B7F0: 388AAF18  addi r4, r10, -0x50e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20712;
	// 82C1B7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B7F8: 4BFFFC89  bl 0x82c1b480
	ctx.lr = 0x82C1B7FC;
	sub_82C1B480(ctx, base);
	// 82C1B7FC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82C1B800: 38BE000F  addi r5, r30, 0xf
	ctx.r[5].s64 = ctx.r[30].s64 + 15;
	// 82C1B804: 38892EBC  addi r4, r9, 0x2ebc
	ctx.r[4].s64 = ctx.r[9].s64 + 11964;
	// 82C1B808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1B80C: 4BFFFC75  bl 0x82c1b480
	ctx.lr = 0x82C1B810;
	sub_82C1B480(ctx, base);
	// 82C1B810: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C1B814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1B818: 4808DC44  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B820 size=128
    let mut pc: u32 = 0x82C1B820;
    'dispatch: loop {
        match pc {
            0x82C1B820 => {
    //   block [0x82C1B820..0x82C1B8A0)
	// 82C1B820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B82C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B838: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1B83C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B840: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1B844: 388BAF94  addi r4, r11, -0x506c
	ctx.r[4].s64 = ctx.r[11].s64 + -20588;
	// 82C1B848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B84C: 4BFEB95D  bl 0x82c071a8
	ctx.lr = 0x82C1B850;
	sub_82C071A8(ctx, base);
	// 82C1B850: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B854: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B858: 419A0028  beq cr6, 0x82c1b880
	if ctx.cr[6].eq {
	pc = 0x82C1B880; continue 'dispatch;
	}
	// 82C1B85C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C1B860: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82C1B864: 388B2DF8  addi r4, r11, 0x2df8
	ctx.r[4].s64 = ctx.r[11].s64 + 11768;
	// 82C1B868: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B86C: 4BFFFA4D  bl 0x82c1b2b8
	ctx.lr = 0x82C1B870;
	sub_82C1B2B8(ctx, base);
	// 82C1B870: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B874: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B878: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B87C: 409A0008  bne cr6, 0x82c1b884
	if !ctx.cr[6].eq {
	pc = 0x82C1B884; continue 'dispatch;
	}
	// 82C1B880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B884: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B894: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B8A0 size=128
    let mut pc: u32 = 0x82C1B8A0;
    'dispatch: loop {
        match pc {
            0x82C1B8A0 => {
    //   block [0x82C1B8A0..0x82C1B920)
	// 82C1B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B8A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B8AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B8B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B8B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1B8BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B8C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1B8C4: 388BAF70  addi r4, r11, -0x5090
	ctx.r[4].s64 = ctx.r[11].s64 + -20624;
	// 82C1B8C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B8CC: 4BFEB8DD  bl 0x82c071a8
	ctx.lr = 0x82C1B8D0;
	sub_82C071A8(ctx, base);
	// 82C1B8D0: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B8D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B8D8: 419A0028  beq cr6, 0x82c1b900
	if ctx.cr[6].eq {
	pc = 0x82C1B900; continue 'dispatch;
	}
	// 82C1B8DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B8E0: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82C1B8E4: 388BAF9C  addi r4, r11, -0x5064
	ctx.r[4].s64 = ctx.r[11].s64 + -20580;
	// 82C1B8E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B8EC: 4BFEB8BD  bl 0x82c071a8
	ctx.lr = 0x82C1B8F0;
	sub_82C071A8(ctx, base);
	// 82C1B8F0: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B8F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B8F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B8FC: 409A0008  bne cr6, 0x82c1b904
	if !ctx.cr[6].eq {
	pc = 0x82C1B904; continue 'dispatch;
	}
	// 82C1B900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B904: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B914: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B920 size=160
    let mut pc: u32 = 0x82C1B920;
    'dispatch: loop {
        match pc {
            0x82C1B920 => {
    //   block [0x82C1B920..0x82C1B9C0)
	// 82C1B920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B92C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B938: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1B93C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B940: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1B944: 388BAF70  addi r4, r11, -0x5090
	ctx.r[4].s64 = ctx.r[11].s64 + -20624;
	// 82C1B948: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B94C: 4BFEB85D  bl 0x82c071a8
	ctx.lr = 0x82C1B950;
	sub_82C071A8(ctx, base);
	// 82C1B950: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B954: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B958: 419A0048  beq cr6, 0x82c1b9a0
	if ctx.cr[6].eq {
	pc = 0x82C1B9A0; continue 'dispatch;
	}
	// 82C1B95C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B960: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82C1B964: 388BAFB4  addi r4, r11, -0x504c
	ctx.r[4].s64 = ctx.r[11].s64 + -20556;
	// 82C1B968: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B96C: 4BFEB83D  bl 0x82c071a8
	ctx.lr = 0x82C1B970;
	sub_82C071A8(ctx, base);
	// 82C1B970: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B974: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B978: 419A0028  beq cr6, 0x82c1b9a0
	if ctx.cr[6].eq {
	pc = 0x82C1B9A0; continue 'dispatch;
	}
	// 82C1B97C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1B980: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 82C1B984: 388BAFA8  addi r4, r11, -0x5058
	ctx.r[4].s64 = ctx.r[11].s64 + -20568;
	// 82C1B988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B98C: 4BFEB81D  bl 0x82c071a8
	ctx.lr = 0x82C1B990;
	sub_82C071A8(ctx, base);
	// 82C1B990: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B994: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1B998: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B99C: 409A0008  bne cr6, 0x82c1b9a4
	if !ctx.cr[6].eq {
	pc = 0x82C1B9A4; continue 'dispatch;
	}
	// 82C1B9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1B9A4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1B9A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1B9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1B9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1B9B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1B9B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1B9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1B9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1B9C0 size=192
    let mut pc: u32 = 0x82C1B9C0;
    'dispatch: loop {
        match pc {
            0x82C1B9C0 => {
    //   block [0x82C1B9C0..0x82C1BA80)
	// 82C1B9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1B9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1B9C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1B9CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1B9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1B9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1B9D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1B9DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C1B9E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1B9E4: 388B2DF8  addi r4, r11, 0x2df8
	ctx.r[4].s64 = ctx.r[11].s64 + 11768;
	// 82C1B9E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1B9EC: 4BFFF8CD  bl 0x82c1b2b8
	ctx.lr = 0x82C1B9F0;
	sub_82C1B2B8(ctx, base);
	// 82C1B9F0: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1B9F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1B9F8: 419A0068  beq cr6, 0x82c1ba60
	if ctx.cr[6].eq {
	pc = 0x82C1BA60; continue 'dispatch;
	}
	// 82C1B9FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1BA00: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82C1BA04: 388BAFC0  addi r4, r11, -0x5040
	ctx.r[4].s64 = ctx.r[11].s64 + -20544;
	// 82C1BA08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1BA0C: 4BFFFBA5  bl 0x82c1b5b0
	ctx.lr = 0x82C1BA10;
	sub_82C1B5B0(ctx, base);
	// 82C1BA10: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BA14: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1BA18: 419A0048  beq cr6, 0x82c1ba60
	if ctx.cr[6].eq {
	pc = 0x82C1BA60; continue 'dispatch;
	}
	// 82C1BA1C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82C1BA20: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 82C1BA24: 388BC314  addi r4, r11, -0x3cec
	ctx.r[4].s64 = ctx.r[11].s64 + -15596;
	// 82C1BA28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1BA2C: 4BFFFB85  bl 0x82c1b5b0
	ctx.lr = 0x82C1BA30;
	sub_82C1B5B0(ctx, base);
	// 82C1BA30: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BA34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1BA38: 419A0028  beq cr6, 0x82c1ba60
	if ctx.cr[6].eq {
	pc = 0x82C1BA60; continue 'dispatch;
	}
	// 82C1BA3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82C1BA40: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82C1BA44: 388BC320  addi r4, r11, -0x3ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -15584;
	// 82C1BA48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1BA4C: 4BFFFB65  bl 0x82c1b5b0
	ctx.lr = 0x82C1BA50;
	sub_82C1B5B0(ctx, base);
	// 82C1BA50: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BA54: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1BA58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1BA5C: 409A0008  bne cr6, 0x82c1ba64
	if !ctx.cr[6].eq {
	pc = 0x82C1BA64; continue 'dispatch;
	}
	// 82C1BA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1BA64: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1BA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1BA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1BA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1BA74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1BA78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1BA80 size=600
    let mut pc: u32 = 0x82C1BA80;
    'dispatch: loop {
        match pc {
            0x82C1BA80 => {
    //   block [0x82C1BA80..0x82C1BCD8)
	// 82C1BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1BA84: 4808D985  bl 0x82ca9408
	ctx.lr = 0x82C1BA88;
	sub_82CA93D0(ctx, base);
	// 82C1BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1BA8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C1BA90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1BA94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C1BA98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BA9C: 388BA328  addi r4, r11, -0x5cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -23768;
	// 82C1BAA0: 4BFECB29  bl 0x82c085c8
	ctx.lr = 0x82C1BAA4;
	sub_82C085C8(ctx, base);
	// 82C1BAA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C1BAA8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BAAC: 812A004C  lwz r9, 0x4c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1BAB0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1BAB4: 419A0108  beq cr6, 0x82c1bbbc
	if ctx.cr[6].eq {
	pc = 0x82C1BBBC; continue 'dispatch;
	}
	// 82C1BAB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BAC0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1BAC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BAC8: 4E800421  bctrl
	ctx.lr = 0x82C1BACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BACC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C1BAD0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BAD4: 409A0148  bne cr6, 0x82c1bc1c
	if !ctx.cr[6].eq {
	pc = 0x82C1BC1C; continue 'dispatch;
	}
	// 82C1BAD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1BADC: 419A0020  beq cr6, 0x82c1bafc
	if ctx.cr[6].eq {
	pc = 0x82C1BAFC; continue 'dispatch;
	}
	// 82C1BAE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BAE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1BAE8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BAEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BAF0: 4E800421  bctrl
	ctx.lr = 0x82C1BAF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C1BAF8: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C1BAFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1BB00: 4B5DBD01  bl 0x821f7800
	ctx.lr = 0x82C1BB04;
	sub_821F7800(ctx, base);
	// 82C1BB04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1BB08: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1BB0C: 388BAFF4  addi r4, r11, -0x500c
	ctx.r[4].s64 = ctx.r[11].s64 + -20492;
	// 82C1BB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BB14: 4BFFF7A5  bl 0x82c1b2b8
	ctx.lr = 0x82C1BB18;
	sub_82C1B2B8(ctx, base);
	// 82C1BB18: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BB1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1BB20: 419A0094  beq cr6, 0x82c1bbb4
	if ctx.cr[6].eq {
	pc = 0x82C1BBB4; continue 'dispatch;
	}
	// 82C1BB24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BB28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1BB2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BB30: 388AAFEC  addi r4, r10, -0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + -20500;
	// 82C1BB34: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1BB38: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C1BB3C: 4E800421  bctrl
	ctx.lr = 0x82C1BB40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BB40: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BB44: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C1BB48: 419A006C  beq cr6, 0x82c1bbb4
	if ctx.cr[6].eq {
	pc = 0x82C1BBB4; continue 'dispatch;
	}
	// 82C1BB4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BB50: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1BB54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C1BB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1BB5C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BB60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BB64: 4E800421  bctrl
	ctx.lr = 0x82C1BB68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BB68: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C1BB6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1BB70: 409A0058  bne cr6, 0x82c1bbc8
	if !ctx.cr[6].eq {
	pc = 0x82C1BBC8; continue 'dispatch;
	}
	// 82C1BB74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BB78: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82C1BB7C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C1BB80: 3BCA934C  addi r30, r10, -0x6cb4
	ctx.r[30].s64 = ctx.r[10].s64 + -27828;
	// 82C1BB84: 3BA9AFD0  addi r29, r9, -0x5030
	ctx.r[29].s64 = ctx.r[9].s64 + -20528;
	// 82C1BB88: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 82C1BB8C: 810B0040  lwz r8, 0x40(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C1BB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BB94: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C1BB98: 4E800421  bctrl
	ctx.lr = 0x82C1BB9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BB9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C1BBA0: 4BFE9C59  bl 0x82c057f8
	ctx.lr = 0x82C1BBA4;
	sub_82C057F8(ctx, base);
	// 82C1BBA4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C1BBA8: 4BFE9C19  bl 0x82c057c0
	ctx.lr = 0x82C1BBAC;
	sub_82C057C0(ctx, base);
	// 82C1BBAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1BBB0: 4BFE9C49  bl 0x82c057f8
	ctx.lr = 0x82C1BBB4;
	sub_82C057F8(ctx, base);
	// 82C1BBB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1BBB8: 4B5F9221  bl 0x82214dd8
	ctx.lr = 0x82C1BBBC;
	sub_82214DD8(ctx, base);
	// 82C1BBBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1BBC4: 4808D894  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C1BBC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BBCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1BBD0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1BBD4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BBD8: 4E800421  bctrl
	ctx.lr = 0x82C1BBDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BBDC: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BBE0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1BBE4: 419AFFD0  beq cr6, 0x82c1bbb4
	if ctx.cr[6].eq {
	pc = 0x82C1BBB4; continue 'dispatch;
	}
	// 82C1BBE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BBEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BBF0: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1BBF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BBF8: 4E800421  bctrl
	ctx.lr = 0x82C1BBFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BBFC: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BC00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1BC04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1BC08: 409A00C0  bne cr6, 0x82c1bcc8
	if !ctx.cr[6].eq {
	pc = 0x82C1BCC8; continue 'dispatch;
	}
	// 82C1BC0C: 4B5F91CD  bl 0x82214dd8
	ctx.lr = 0x82C1BC10;
	sub_82214DD8(ctx, base);
	// 82C1BC10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1BC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1BC18: 4808D840  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C1BC1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BC20: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BC24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BC28: 4E800421  bctrl
	ctx.lr = 0x82C1BC2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BC2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1BC30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1BC34: 4B5D460D  bl 0x821f0240
	ctx.lr = 0x82C1BC38;
	sub_821F0240(ctx, base);
	// 82C1BC38: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C1BC3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C1BC40: 3889AFF4  addi r4, r9, -0x500c
	ctx.r[4].s64 = ctx.r[9].s64 + -20492;
	// 82C1BC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BC48: 4BFFF671  bl 0x82c1b2b8
	ctx.lr = 0x82C1BC4C;
	sub_82C1B2B8(ctx, base);
	// 82C1BC4C: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BC50: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C1BC54: 419AFF60  beq cr6, 0x82c1bbb4
	if ctx.cr[6].eq {
	pc = 0x82C1BBB4; continue 'dispatch;
	}
	// 82C1BC58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BC5C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1BC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BC64: 388AAFEC  addi r4, r10, -0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + -20500;
	// 82C1BC68: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1BC6C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C1BC70: 4E800421  bctrl
	ctx.lr = 0x82C1BC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BC74: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BC78: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C1BC7C: 419AFF38  beq cr6, 0x82c1bbb4
	if ctx.cr[6].eq {
	pc = 0x82C1BBB4; continue 'dispatch;
	}
	// 82C1BC80: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BC84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C1BC88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BC8C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1BC90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BC94: 4E800421  bctrl
	ctx.lr = 0x82C1BC98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BC98: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BC9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1BCA0: 419AFF14  beq cr6, 0x82c1bbb4
	if ctx.cr[6].eq {
	pc = 0x82C1BBB4; continue 'dispatch;
	}
	// 82C1BCA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BCA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BCAC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1BCB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BCB4: 4E800421  bctrl
	ctx.lr = 0x82C1BCB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BCB8: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1BCBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1BCC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1BCC4: 419AFEF4  beq cr6, 0x82c1bbb8
	if ctx.cr[6].eq {
	pc = 0x82C1BBB8; continue 'dispatch;
	}
	// 82C1BCC8: 4B5F9111  bl 0x82214dd8
	ctx.lr = 0x82C1BCCC;
	sub_82214DD8(ctx, base);
	// 82C1BCCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C1BCD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1BCD4: 4808D784  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C1BCD8 size=128
    let mut pc: u32 = 0x82C1BCD8;
    'dispatch: loop {
        match pc {
            0x82C1BCD8 => {
    //   block [0x82C1BCD8..0x82C1BD58)
	// 82C1BCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1BCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C1BCE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1BCE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1BCE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1BCEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1BCF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1BCF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1BCF8: 394BAF00  addi r10, r11, -0x5100
	ctx.r[10].s64 = ctx.r[11].s64 + -20736;
	// 82C1BCFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BD00: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1BD04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1BD08: 419A0020  beq cr6, 0x82c1bd28
	if ctx.cr[6].eq {
	pc = 0x82C1BD28; continue 'dispatch;
	}
	// 82C1BD0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BD10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1BD14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BD18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BD1C: 4E800421  bctrl
	ctx.lr = 0x82C1BD20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BD20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C1BD24: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C1BD28: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C1BD2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BD30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BD34: 419A000C  beq cr6, 0x82c1bd40
	if ctx.cr[6].eq {
	pc = 0x82C1BD40; continue 'dispatch;
	}
	// 82C1BD38: 4BC29A79  bl 0x828457b0
	ctx.lr = 0x82C1BD3C;
	sub_828457B0(ctx, base);
	// 82C1BD3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BD40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1BD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1BD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1BD4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C1BD50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD58 size=16
    let mut pc: u32 = 0x82C1BD58;
    'dispatch: loop {
        match pc {
            0x82C1BD58 => {
    //   block [0x82C1BD58..0x82C1BD68)
	// 82C1BD58: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1BD5C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1BD60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BD64: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD68 size=8
    let mut pc: u32 = 0x82C1BD68;
    'dispatch: loop {
        match pc {
            0x82C1BD68 => {
    //   block [0x82C1BD68..0x82C1BD70)
	// 82C1BD68: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BD6C: 480088A4  b 0x82c24610
	sub_82C24610(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD70 size=4
    let mut pc: u32 = 0x82C1BD70;
    'dispatch: loop {
        match pc {
            0x82C1BD70 => {
    //   block [0x82C1BD70..0x82C1BD74)
	// 82C1BD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD78 size=16
    let mut pc: u32 = 0x82C1BD78;
    'dispatch: loop {
        match pc {
            0x82C1BD78 => {
    //   block [0x82C1BD78..0x82C1BD88)
	// 82C1BD78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BD7C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1BD80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BD84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD88 size=8
    let mut pc: u32 = 0x82C1BD88;
    'dispatch: loop {
        match pc {
            0x82C1BD88 => {
    //   block [0x82C1BD88..0x82C1BD90)
	// 82C1BD88: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BD8C: 48008884  b 0x82c24610
	sub_82C24610(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD90 size=4
    let mut pc: u32 = 0x82C1BD90;
    'dispatch: loop {
        match pc {
            0x82C1BD90 => {
    //   block [0x82C1BD90..0x82C1BD94)
	// 82C1BD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BD98 size=16
    let mut pc: u32 = 0x82C1BD98;
    'dispatch: loop {
        match pc {
            0x82C1BD98 => {
    //   block [0x82C1BD98..0x82C1BDA8)
	// 82C1BD98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BD9C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C1BDA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BDA4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BDA8 size=16
    let mut pc: u32 = 0x82C1BDA8;
    'dispatch: loop {
        match pc {
            0x82C1BDA8 => {
    //   block [0x82C1BDA8..0x82C1BDB8)
	// 82C1BDA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BDAC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C1BDB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BDB4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BDB8 size=16
    let mut pc: u32 = 0x82C1BDB8;
    'dispatch: loop {
        match pc {
            0x82C1BDB8 => {
    //   block [0x82C1BDB8..0x82C1BDC8)
	// 82C1BDB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BDBC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1BDC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BDC4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1BDC8 size=16
    let mut pc: u32 = 0x82C1BDC8;
    'dispatch: loop {
        match pc {
            0x82C1BDC8 => {
    //   block [0x82C1BDC8..0x82C1BDD8)
	// 82C1BDC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BDCC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C1BDD0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1BDD4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C1BDD8 size=324
    let mut pc: u32 = 0x82C1BDD8;
    'dispatch: loop {
        match pc {
            0x82C1BDD8 => {
    //   block [0x82C1BDD8..0x82C1BF1C)
	// 82C1BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1BDDC: 4808D621  bl 0x82ca93fc
	ctx.lr = 0x82C1BDE0;
	sub_82CA93D0(ctx, base);
	// 82C1BDE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1BDE4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C1BDE8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C1BDEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C1BDF0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82C1BDF4: 9BBA0000  stb r29, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82C1BDF8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1BDFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BE00: 409A0010  bne cr6, 0x82c1be10
	if !ctx.cr[6].eq {
	pc = 0x82C1BE10; continue 'dispatch;
	}
	// 82C1BE04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1BE08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1BE0C: 4808D640  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C1BE10: 37EBFFFF  addic. r31, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82C1BE14: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82C1BE18: 3B6BFFDF  addi r27, r11, -0x21
	ctx.r[27].s64 = ctx.r[11].s64 + -33;
	// 82C1BE1C: 4180007C  blt 0x82c1be98
	if ctx.cr[0].lt {
	pc = 0x82C1BE98; continue 'dispatch;
	}
	// 82C1BE20: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82C1BE24: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BE28: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82C1BE2C: 7FC90194  addze r30, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[30].s64 = tmp.s64;
	// 82C1BE30: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C1BE34: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82C1BE38: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BE3C: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BE40: 80A60004  lwz r5, 4(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BE44: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82C1BE48: 4E800421  bctrl
	ctx.lr = 0x82C1BE4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BE4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BE50: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C1BE54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BE58: 419A0008  beq cr6, 0x82c1be60
	if ctx.cr[6].eq {
	pc = 0x82C1BE60; continue 'dispatch;
	}
	// 82C1BE5C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BE60: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BE64: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C1BE68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BE6C: 419A0008  beq cr6, 0x82c1be74
	if ctx.cr[6].eq {
	pc = 0x82C1BE74; continue 'dispatch;
	}
	// 82C1BE70: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BE74: 48090CDD  bl 0x82cacb50
	ctx.lr = 0x82C1BE78;
	sub_82CACB50(ctx, base);
	// 82C1BE78: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C1BE7C: 419A008C  beq cr6, 0x82c1bf08
	if ctx.cr[6].eq {
	pc = 0x82C1BF08; continue 'dispatch;
	}
	// 82C1BE80: 4098000C  bge cr6, 0x82c1be8c
	if !ctx.cr[6].lt {
	pc = 0x82C1BE8C; continue 'dispatch;
	}
	// 82C1BE84: 3BFEFFFF  addi r31, r30, -1
	ctx.r[31].s64 = ctx.r[30].s64 + -1;
	// 82C1BE88: 48000008  b 0x82c1be90
	pc = 0x82C1BE90; continue 'dispatch;
	// 82C1BE8C: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 82C1BE90: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82C1BE94: 4099FF8C  ble cr6, 0x82c1be20
	if !ctx.cr[6].gt {
	pc = 0x82C1BE20; continue 'dispatch;
	}
	// 82C1BE98: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82C1BE9C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BEA0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82C1BEA4: 7FE90194  addze r31, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[31].s64 = tmp.s64;
	// 82C1BEA8: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C1BEAC: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82C1BEB0: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BEB4: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BEB8: 80A60004  lwz r5, 4(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1BEBC: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82C1BEC0: 4E800421  bctrl
	ctx.lr = 0x82C1BEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1BEC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BEC8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C1BECC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BED0: 419A0008  beq cr6, 0x82c1bed8
	if ctx.cr[6].eq {
	pc = 0x82C1BED8; continue 'dispatch;
	}
	// 82C1BED4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BED8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BEDC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C1BEE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1BEE4: 419A0008  beq cr6, 0x82c1beec
	if ctx.cr[6].eq {
	pc = 0x82C1BEEC; continue 'dispatch;
	}
	// 82C1BEE8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1BEEC: 48090C65  bl 0x82cacb50
	ctx.lr = 0x82C1BEF0;
	sub_82CACB50(ctx, base);
	// 82C1BEF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C1BEF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1BEF8: 41980008  blt cr6, 0x82c1bf00
	if ctx.cr[6].lt {
	pc = 0x82C1BF00; continue 'dispatch;
	}
	// 82C1BEFC: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 82C1BF00: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1BF04: 4808D548  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C1BF08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C1BF0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C1BF10: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82C1BF14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1BF18: 4808D534  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


