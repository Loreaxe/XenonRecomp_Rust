pub fn sub_82632640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632640 size=100
    let mut pc: u32 = 0x82632640;
    'dispatch: loop {
        match pc {
            0x82632640 => {
    //   block [0x82632640..0x826326A4)
	// 82632640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263264C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632654: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82632658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632660: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82632664: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263266C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632674: 386A7774  addi r3, r10, 0x7774
	ctx.r[3].s64 = ctx.r[10].s64 + 30580;
	// 82632678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263267C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632680: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82632684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632688: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263268C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632690: 4BE34791  bl 0x82466e20
	ctx.lr = 0x82632694;
	sub_82466E20(ctx, base);
	// 82632694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263269C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826326A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826326A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826326A8 size=100
    let mut pc: u32 = 0x826326A8;
    'dispatch: loop {
        match pc {
            0x826326A8 => {
    //   block [0x826326A8..0x8263270C)
	// 826326A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826326AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826326B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826326B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826326B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826326BC: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 826326C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826326C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826326C8: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826326CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826326D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826326D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826326D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826326DC: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 826326E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826326E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826326E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826326EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826326F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826326F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826326F8: 4BE34729  bl 0x82466e20
	ctx.lr = 0x826326FC;
	sub_82466E20(ctx, base);
	// 826326FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632710 size=112
    let mut pc: u32 = 0x82632710;
    'dispatch: loop {
        match pc {
            0x82632710 => {
    //   block [0x82632710..0x82632780)
	// 82632710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263271C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632720: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632724: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263272C: 390BE980  addi r8, r11, -0x1680
	ctx.r[8].s64 = ctx.r[11].s64 + -5760;
	// 82632730: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82632734: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82632738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263273C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632748: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 8263274C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263275C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263276C: 4BE346B5  bl 0x82466e20
	ctx.lr = 0x82632770;
	sub_82466E20(ctx, base);
	// 82632770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632780 size=112
    let mut pc: u32 = 0x82632780;
    'dispatch: loop {
        match pc {
            0x82632780 => {
    //   block [0x82632780..0x826327F0)
	// 82632780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263278C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632790: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632794: 38AA7594  addi r5, r10, 0x7594
	ctx.r[5].s64 = ctx.r[10].s64 + 30100;
	// 82632798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263279C: 390BEA10  addi r8, r11, -0x15f0
	ctx.r[8].s64 = ctx.r[11].s64 + -5616;
	// 826327A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826327A4: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826327A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826327AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826327B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826327B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826327B8: 386A7804  addi r3, r10, 0x7804
	ctx.r[3].s64 = ctx.r[10].s64 + 30724;
	// 826327BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826327C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826327C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826327C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826327CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826327D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826327D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826327D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826327DC: 4BE34645  bl 0x82466e20
	ctx.lr = 0x826327E0;
	sub_82466E20(ctx, base);
	// 826327E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826327E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826327E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826327EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826327F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826327F0 size=112
    let mut pc: u32 = 0x826327F0;
    'dispatch: loop {
        match pc {
            0x826327F0 => {
    //   block [0x826327F0..0x82632860)
	// 826327F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826327F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826327F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826327FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632800: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632804: 38AA76E4  addi r5, r10, 0x76e4
	ctx.r[5].s64 = ctx.r[10].s64 + 30436;
	// 82632808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263280C: 390BEA28  addi r8, r11, -0x15d8
	ctx.r[8].s64 = ctx.r[11].s64 + -5592;
	// 82632810: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82632814: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82632818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263281C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632828: 386A7834  addi r3, r10, 0x7834
	ctx.r[3].s64 = ctx.r[10].s64 + 30772;
	// 8263282C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263283C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263284C: 4BE345D5  bl 0x82466e20
	ctx.lr = 0x82632850;
	sub_82466E20(ctx, base);
	// 82632850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263285C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632860 size=112
    let mut pc: u32 = 0x82632860;
    'dispatch: loop {
        match pc {
            0x82632860 => {
    //   block [0x82632860..0x826328D0)
	// 82632860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263286C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632870: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632874: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82632878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263287C: 390BEA58  addi r8, r11, -0x15a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5544;
	// 82632880: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82632884: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82632888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263288C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632898: 386A7864  addi r3, r10, 0x7864
	ctx.r[3].s64 = ctx.r[10].s64 + 30820;
	// 8263289C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826328A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826328A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826328A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826328AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826328B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826328B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826328B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826328BC: 4BE34565  bl 0x82466e20
	ctx.lr = 0x826328C0;
	sub_82466E20(ctx, base);
	// 826328C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826328C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826328C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826328CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826328D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826328D0 size=112
    let mut pc: u32 = 0x826328D0;
    'dispatch: loop {
        match pc {
            0x826328D0 => {
    //   block [0x826328D0..0x82632940)
	// 826328D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826328D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826328D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826328DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826328E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826328E4: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 826328E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826328EC: 390BEAA0  addi r8, r11, -0x1560
	ctx.r[8].s64 = ctx.r[11].s64 + -5472;
	// 826328F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826328F4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826328F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826328FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632908: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 8263290C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263291C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263292C: 4BE344F5  bl 0x82466e20
	ctx.lr = 0x82632930;
	sub_82466E20(ctx, base);
	// 82632930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263293C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632940 size=112
    let mut pc: u32 = 0x82632940;
    'dispatch: loop {
        match pc {
            0x82632940 => {
    //   block [0x82632940..0x826329B0)
	// 82632940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263294C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632950: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632954: 38AA7264  addi r5, r10, 0x7264
	ctx.r[5].s64 = ctx.r[10].s64 + 29284;
	// 82632958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263295C: 390BEAE8  addi r8, r11, -0x1518
	ctx.r[8].s64 = ctx.r[11].s64 + -5400;
	// 82632960: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632964: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82632968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263296C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632978: 386A78C4  addi r3, r10, 0x78c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30916;
	// 8263297C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263298C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263299C: 4BE34485  bl 0x82466e20
	ctx.lr = 0x826329A0;
	sub_82466E20(ctx, base);
	// 826329A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826329A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826329A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826329AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826329B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826329B0 size=112
    let mut pc: u32 = 0x826329B0;
    'dispatch: loop {
        match pc {
            0x826329B0 => {
    //   block [0x826329B0..0x82632A20)
	// 826329B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826329B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826329B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826329BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826329C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826329C4: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 826329C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826329CC: 390BEB00  addi r8, r11, -0x1500
	ctx.r[8].s64 = ctx.r[11].s64 + -5376;
	// 826329D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826329D4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826329D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826329DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826329E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826329E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826329E8: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 826329EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826329F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826329F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826329F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826329FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632A0C: 4BE34415  bl 0x82466e20
	ctx.lr = 0x82632A10;
	sub_82466E20(ctx, base);
	// 82632A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632A20 size=112
    let mut pc: u32 = 0x82632A20;
    'dispatch: loop {
        match pc {
            0x82632A20 => {
    //   block [0x82632A20..0x82632A90)
	// 82632A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632A2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632A30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632A34: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632A3C: 390BEB30  addi r8, r11, -0x14d0
	ctx.r[8].s64 = ctx.r[11].s64 + -5328;
	// 82632A40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82632A44: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82632A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632A58: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 82632A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632A7C: 4BE343A5  bl 0x82466e20
	ctx.lr = 0x82632A80;
	sub_82466E20(ctx, base);
	// 82632A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632A90 size=112
    let mut pc: u32 = 0x82632A90;
    'dispatch: loop {
        match pc {
            0x82632A90 => {
    //   block [0x82632A90..0x82632B00)
	// 82632A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632A9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632AA0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632AA4: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82632AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632AAC: 390BEB90  addi r8, r11, -0x1470
	ctx.r[8].s64 = ctx.r[11].s64 + -5232;
	// 82632AB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632AB4: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82632AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632ABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632AC8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 82632ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632AEC: 4BE34335  bl 0x82466e20
	ctx.lr = 0x82632AF0;
	sub_82466E20(ctx, base);
	// 82632AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632B00 size=112
    let mut pc: u32 = 0x82632B00;
    'dispatch: loop {
        match pc {
            0x82632B00 => {
    //   block [0x82632B00..0x82632B70)
	// 82632B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632B0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632B10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632B14: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82632B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632B1C: 390BEBA8  addi r8, r11, -0x1458
	ctx.r[8].s64 = ctx.r[11].s64 + -5208;
	// 82632B20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82632B24: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82632B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632B38: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 82632B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632B5C: 4BE342C5  bl 0x82466e20
	ctx.lr = 0x82632B60;
	sub_82466E20(ctx, base);
	// 82632B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632B70 size=112
    let mut pc: u32 = 0x82632B70;
    'dispatch: loop {
        match pc {
            0x82632B70 => {
    //   block [0x82632B70..0x82632BE0)
	// 82632B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632B7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632B80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632B84: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82632B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632B8C: 390BEBD8  addi r8, r11, -0x1428
	ctx.r[8].s64 = ctx.r[11].s64 + -5160;
	// 82632B90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632B94: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82632B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632B9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632BA8: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 82632BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632BCC: 4BE34255  bl 0x82466e20
	ctx.lr = 0x82632BD0;
	sub_82466E20(ctx, base);
	// 82632BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82632BE0 size=24
    let mut pc: u32 = 0x82632BE0;
    'dispatch: loop {
        match pc {
            0x82632BE0 => {
    //   block [0x82632BE0..0x82632BF8)
	// 82632BE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632BE4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82632BE8: 394A23B0  addi r10, r10, 0x23b0
	ctx.r[10].s64 = ctx.r[10].s64 + 9136;
	// 82632BEC: 816BEBF0  lwz r11, -0x1410(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5136 as u32) ) } as u64;
	// 82632BF0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82632BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632BF8 size=112
    let mut pc: u32 = 0x82632BF8;
    'dispatch: loop {
        match pc {
            0x82632BF8 => {
    //   block [0x82632BF8..0x82632C68)
	// 82632BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632C04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632C08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632C0C: 392A4EE8  addi r9, r10, 0x4ee8
	ctx.r[9].s64 = ctx.r[10].s64 + 20200;
	// 82632C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632C14: 390B23B0  addi r8, r11, 0x23b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9136;
	// 82632C18: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82632C1C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 82632C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632C30: 386A79E4  addi r3, r10, 0x79e4
	ctx.r[3].s64 = ctx.r[10].s64 + 31204;
	// 82632C34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632C38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82632C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632C54: 4BE341CD  bl 0x82466e20
	ctx.lr = 0x82632C58;
	sub_82466E20(ctx, base);
	// 82632C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632C68 size=108
    let mut pc: u32 = 0x82632C68;
    'dispatch: loop {
        match pc {
            0x82632C68 => {
    //   block [0x82632C68..0x82632CD4)
	// 82632C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632C74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632C7C: 38EBEBF4  addi r7, r11, -0x140c
	ctx.r[7].s64 = ctx.r[11].s64 + -5132;
	// 82632C80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632C84: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82632C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632C8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632C90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632C98: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 82632C9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632CC0: 4BE34161  bl 0x82466e20
	ctx.lr = 0x82632CC4;
	sub_82466E20(ctx, base);
	// 82632CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632CD8 size=108
    let mut pc: u32 = 0x82632CD8;
    'dispatch: loop {
        match pc {
            0x82632CD8 => {
    //   block [0x82632CD8..0x82632D44)
	// 82632CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632CE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632CEC: 38EBEC10  addi r7, r11, -0x13f0
	ctx.r[7].s64 = ctx.r[11].s64 + -5104;
	// 82632CF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82632CF4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 82632CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632CFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632D00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632D08: 386A7A44  addi r3, r10, 0x7a44
	ctx.r[3].s64 = ctx.r[10].s64 + 31300;
	// 82632D0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632D30: 4BE340F1  bl 0x82466e20
	ctx.lr = 0x82632D34;
	sub_82466E20(ctx, base);
	// 82632D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632D48 size=116
    let mut pc: u32 = 0x82632D48;
    'dispatch: loop {
        match pc {
            0x82632D48 => {
    //   block [0x82632D48..0x82632DBC)
	// 82632D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632D54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632D58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632D5C: 390BEC58  addi r8, r11, -0x13a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5032;
	// 82632D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632D64: 392A4FB8  addi r9, r10, 0x4fb8
	ctx.r[9].s64 = ctx.r[10].s64 + 20408;
	// 82632D68: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632D6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82632D70: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632D74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632D8C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82632D90: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82632D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632D98: 386B7A74  addi r3, r11, 0x7a74
	ctx.r[3].s64 = ctx.r[11].s64 + 31348;
	// 82632D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82632DA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632DA8: 4BE34079  bl 0x82466e20
	ctx.lr = 0x82632DAC;
	sub_82466E20(ctx, base);
	// 82632DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632DC0 size=108
    let mut pc: u32 = 0x82632DC0;
    'dispatch: loop {
        match pc {
            0x82632DC0 => {
    //   block [0x82632DC0..0x82632E2C)
	// 82632DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632DCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632DD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632DD4: 38EBEC70  addi r7, r11, -0x1390
	ctx.r[7].s64 = ctx.r[11].s64 + -5008;
	// 82632DD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82632DDC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82632DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632DF0: 386A7AA4  addi r3, r10, 0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 31396;
	// 82632DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632E18: 4BE34009  bl 0x82466e20
	ctx.lr = 0x82632E1C;
	sub_82466E20(ctx, base);
	// 82632E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82632E30 size=24
    let mut pc: u32 = 0x82632E30;
    'dispatch: loop {
        match pc {
            0x82632E30 => {
    //   block [0x82632E30..0x82632E48)
	// 82632E30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632E34: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82632E38: 394A23F8  addi r10, r10, 0x23f8
	ctx.r[10].s64 = ctx.r[10].s64 + 9208;
	// 82632E3C: 816BECD0  lwz r11, -0x1330(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4912 as u32) ) } as u64;
	// 82632E40: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82632E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632E48 size=116
    let mut pc: u32 = 0x82632E48;
    'dispatch: loop {
        match pc {
            0x82632E48 => {
    //   block [0x82632E48..0x82632EBC)
	// 82632E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632E54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632E5C: 390B23F8  addi r8, r11, 0x23f8
	ctx.r[8].s64 = ctx.r[11].s64 + 9208;
	// 82632E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632E64: 392A5014  addi r9, r10, 0x5014
	ctx.r[9].s64 = ctx.r[10].s64 + 20500;
	// 82632E68: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632E6C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82632E70: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82632E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632E8C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82632E90: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 82632E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632E98: 386B7AD4  addi r3, r11, 0x7ad4
	ctx.r[3].s64 = ctx.r[11].s64 + 31444;
	// 82632E9C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82632EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632EA8: 4BE33F79  bl 0x82466e20
	ctx.lr = 0x82632EAC;
	sub_82466E20(ctx, base);
	// 82632EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632EC0 size=112
    let mut pc: u32 = 0x82632EC0;
    'dispatch: loop {
        match pc {
            0x82632EC0 => {
    //   block [0x82632EC0..0x82632F30)
	// 82632EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632ECC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632ED0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632ED4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82632ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632EDC: 390BECDC  addi r8, r11, -0x1324
	ctx.r[8].s64 = ctx.r[11].s64 + -4900;
	// 82632EE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632EE4: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 82632EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632EEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632EF8: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 82632EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632F1C: 4BE33F05  bl 0x82466e20
	ctx.lr = 0x82632F20;
	sub_82466E20(ctx, base);
	// 82632F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632F30 size=112
    let mut pc: u32 = 0x82632F30;
    'dispatch: loop {
        match pc {
            0x82632F30 => {
    //   block [0x82632F30..0x82632FA0)
	// 82632F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632F3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632F40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632F44: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82632F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632F4C: 390BECF4  addi r8, r11, -0x130c
	ctx.r[8].s64 = ctx.r[11].s64 + -4876;
	// 82632F50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82632F54: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82632F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632F68: 386A7B34  addi r3, r10, 0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + 31540;
	// 82632F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632F8C: 4BE33E95  bl 0x82466e20
	ctx.lr = 0x82632F90;
	sub_82466E20(ctx, base);
	// 82632F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632FA0 size=108
    let mut pc: u32 = 0x82632FA0;
    'dispatch: loop {
        match pc {
            0x82632FA0 => {
    //   block [0x82632FA0..0x8263300C)
	// 82632FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632FAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632FB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82632FB4: 38EBED28  addi r7, r11, -0x12d8
	ctx.r[7].s64 = ctx.r[11].s64 + -4824;
	// 82632FB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82632FBC: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 82632FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82632FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632FD0: 386A7B64  addi r3, r10, 0x7b64
	ctx.r[3].s64 = ctx.r[10].s64 + 31588;
	// 82632FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632FF8: 4BE33E29  bl 0x82466e20
	ctx.lr = 0x82632FFC;
	sub_82466E20(ctx, base);
	// 82632FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633010 size=108
    let mut pc: u32 = 0x82633010;
    'dispatch: loop {
        match pc {
            0x82633010 => {
    //   block [0x82633010..0x8263307C)
	// 82633010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263301C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633024: 38EBED70  addi r7, r11, -0x1290
	ctx.r[7].s64 = ctx.r[11].s64 + -4752;
	// 82633028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263302C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82633030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263303C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633040: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 82633044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263304C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263305C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633068: 4BE33DB9  bl 0x82466e20
	ctx.lr = 0x8263306C;
	sub_82466E20(ctx, base);
	// 8263306C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633080 size=112
    let mut pc: u32 = 0x82633080;
    'dispatch: loop {
        match pc {
            0x82633080 => {
    //   block [0x82633080..0x826330F0)
	// 82633080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263308C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633090: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633094: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263309C: 390BEDA0  addi r8, r11, -0x1260
	ctx.r[8].s64 = ctx.r[11].s64 + -4704;
	// 826330A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826330A4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826330A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826330AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826330B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826330B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826330B8: 386A7BC4  addi r3, r10, 0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31684;
	// 826330BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826330C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826330C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826330C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826330CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826330D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826330D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826330D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826330DC: 4BE33D45  bl 0x82466e20
	ctx.lr = 0x826330E0;
	sub_82466E20(ctx, base);
	// 826330E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826330E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826330E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826330EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826330F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826330F0 size=108
    let mut pc: u32 = 0x826330F0;
    'dispatch: loop {
        match pc {
            0x826330F0 => {
    //   block [0x826330F0..0x8263315C)
	// 826330F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826330F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826330F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826330FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633104: 38EBEDD0  addi r7, r11, -0x1230
	ctx.r[7].s64 = ctx.r[11].s64 + -4656;
	// 82633108: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263310C: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 82633110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263311C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633120: 386A7BF4  addi r3, r10, 0x7bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31732;
	// 82633124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263312C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263313C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633148: 4BE33CD9  bl 0x82466e20
	ctx.lr = 0x8263314C;
	sub_82466E20(ctx, base);
	// 8263314C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633160 size=108
    let mut pc: u32 = 0x82633160;
    'dispatch: loop {
        match pc {
            0x82633160 => {
    //   block [0x82633160..0x826331CC)
	// 82633160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263316C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633174: 38EBEE30  addi r7, r11, -0x11d0
	ctx.r[7].s64 = ctx.r[11].s64 + -4560;
	// 82633178: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263317C: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 82633180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263318C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633190: 386A7C24  addi r3, r10, 0x7c24
	ctx.r[3].s64 = ctx.r[10].s64 + 31780;
	// 82633194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263319C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826331A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826331A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826331A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826331AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826331B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826331B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826331B8: 4BE33C69  bl 0x82466e20
	ctx.lr = 0x826331BC;
	sub_82466E20(ctx, base);
	// 826331BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826331C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826331C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826331C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826331D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826331D0 size=116
    let mut pc: u32 = 0x826331D0;
    'dispatch: loop {
        match pc {
            0x826331D0 => {
    //   block [0x826331D0..0x82633244)
	// 826331D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826331D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826331D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826331DC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826331E0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826331E4: 390AEE78  addi r8, r10, -0x1188
	ctx.r[8].s64 = ctx.r[10].s64 + -4488;
	// 826331E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826331EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826331F0: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826331F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826331F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826331FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633204: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82633208: 396B5050  addi r11, r11, 0x5050
	ctx.r[11].s64 = ctx.r[11].s64 + 20560;
	// 8263320C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633214: 386A7C54  addi r3, r10, 0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + 31828;
	// 82633218: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263321C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633220: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82633224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263322C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633230: 4BE33BF1  bl 0x82466e20
	ctx.lr = 0x82633234;
	sub_82466E20(ctx, base);
	// 82633234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633248 size=112
    let mut pc: u32 = 0x82633248;
    'dispatch: loop {
        match pc {
            0x82633248 => {
    //   block [0x82633248..0x826332B8)
	// 82633248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263324C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633254: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633258: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263325C: 38AA7CB4  addi r5, r10, 0x7cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 31924;
	// 82633260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633264: 390BEF08  addi r8, r11, -0x10f8
	ctx.r[8].s64 = ctx.r[11].s64 + -4344;
	// 82633268: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263326C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82633270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263327C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633280: 386A7C84  addi r3, r10, 0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + 31876;
	// 82633284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263328C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263329C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826332A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826332A4: 4BE33B7D  bl 0x82466e20
	ctx.lr = 0x826332A8;
	sub_82466E20(ctx, base);
	// 826332A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826332AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826332B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826332B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826332B8 size=100
    let mut pc: u32 = 0x826332B8;
    'dispatch: loop {
        match pc {
            0x826332B8 => {
    //   block [0x826332B8..0x8263331C)
	// 826332B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826332BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826332C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826332C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826332C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826332CC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826332D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826332D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826332D8: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826332DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826332E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826332E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826332E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826332EC: 386A7CB4  addi r3, r10, 0x7cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 31924;
	// 826332F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826332F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826332F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826332FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82633304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633308: 4BE33B19  bl 0x82466e20
	ctx.lr = 0x8263330C;
	sub_82466E20(ctx, base);
	// 8263330C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82633320 size=24
    let mut pc: u32 = 0x82633320;
    'dispatch: loop {
        match pc {
            0x82633320 => {
    //   block [0x82633320..0x82633338)
	// 82633320: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633324: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82633328: 394A24B8  addi r10, r10, 0x24b8
	ctx.r[10].s64 = ctx.r[10].s64 + 9400;
	// 8263332C: 816BED24  lwz r11, -0x12dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4828 as u32) ) } as u64;
	// 82633330: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82633334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633338 size=116
    let mut pc: u32 = 0x82633338;
    'dispatch: loop {
        match pc {
            0x82633338 => {
    //   block [0x82633338..0x826333AC)
	// 82633338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263333C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633344: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633348: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263334C: 390B24B8  addi r8, r11, 0x24b8
	ctx.r[8].s64 = ctx.r[11].s64 + 9400;
	// 82633350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633354: 392A5094  addi r9, r10, 0x5094
	ctx.r[9].s64 = ctx.r[10].s64 + 20628;
	// 82633358: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263335C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82633360: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633364: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263336C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263337C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82633380: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82633384: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82633388: 386B7CE4  addi r3, r11, 0x7ce4
	ctx.r[3].s64 = ctx.r[11].s64 + 31972;
	// 8263338C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82633390: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633398: 4BE33A89  bl 0x82466e20
	ctx.lr = 0x8263339C;
	sub_82466E20(ctx, base);
	// 8263339C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826333A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826333A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826333A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826333B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826333B0 size=112
    let mut pc: u32 = 0x826333B0;
    'dispatch: loop {
        match pc {
            0x826333B0 => {
    //   block [0x826333B0..0x82633420)
	// 826333B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826333B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826333B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826333BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826333C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826333C4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826333C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826333CC: 390BEF80  addi r8, r11, -0x1080
	ctx.r[8].s64 = ctx.r[11].s64 + -4224;
	// 826333D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826333D4: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826333D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826333DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826333E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826333E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826333E8: 386A7D14  addi r3, r10, 0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + 32020;
	// 826333EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826333F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826333F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826333F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826333FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263340C: 4BE33A15  bl 0x82466e20
	ctx.lr = 0x82633410;
	sub_82466E20(ctx, base);
	// 82633410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633420 size=112
    let mut pc: u32 = 0x82633420;
    'dispatch: loop {
        match pc {
            0x82633420 => {
    //   block [0x82633420..0x82633490)
	// 82633420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263342C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633430: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633434: 38AA7C54  addi r5, r10, 0x7c54
	ctx.r[5].s64 = ctx.r[10].s64 + 31828;
	// 82633438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263343C: 390BEFC8  addi r8, r11, -0x1038
	ctx.r[8].s64 = ctx.r[11].s64 + -4152;
	// 82633440: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82633444: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82633448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263344C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633458: 386A7D44  addi r3, r10, 0x7d44
	ctx.r[3].s64 = ctx.r[10].s64 + 32068;
	// 8263345C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263346C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263347C: 4BE339A5  bl 0x82466e20
	ctx.lr = 0x82633480;
	sub_82466E20(ctx, base);
	// 82633480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633490 size=108
    let mut pc: u32 = 0x82633490;
    'dispatch: loop {
        match pc {
            0x82633490 => {
    //   block [0x82633490..0x826334FC)
	// 82633490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263349C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826334A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826334A4: 38EBF028  addi r7, r11, -0xfd8
	ctx.r[7].s64 = ctx.r[11].s64 + -4056;
	// 826334A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826334AC: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 826334B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826334B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826334B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826334BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826334C0: 386A7D74  addi r3, r10, 0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + 32116;
	// 826334C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826334C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826334CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826334D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826334D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826334D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826334DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826334E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826334E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826334E8: 4BE33939  bl 0x82466e20
	ctx.lr = 0x826334EC;
	sub_82466E20(ctx, base);
	// 826334EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826334F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826334F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826334F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633500 size=108
    let mut pc: u32 = 0x82633500;
    'dispatch: loop {
        match pc {
            0x82633500 => {
    //   block [0x82633500..0x8263356C)
	// 82633500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263350C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633514: 38EBF070  addi r7, r11, -0xf90
	ctx.r[7].s64 = ctx.r[11].s64 + -3984;
	// 82633518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263351C: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82633520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263352C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633530: 386A7DA4  addi r3, r10, 0x7da4
	ctx.r[3].s64 = ctx.r[10].s64 + 32164;
	// 82633534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263353C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633558: 4BE338C9  bl 0x82466e20
	ctx.lr = 0x8263355C;
	sub_82466E20(ctx, base);
	// 8263355C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633570 size=112
    let mut pc: u32 = 0x82633570;
    'dispatch: loop {
        match pc {
            0x82633570 => {
    //   block [0x82633570..0x826335E0)
	// 82633570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263357C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633580: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633584: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263358C: 390BF0B8  addi r8, r11, -0xf48
	ctx.r[8].s64 = ctx.r[11].s64 + -3912;
	// 82633590: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82633594: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82633598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263359C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826335A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826335A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826335A8: 386A7DD4  addi r3, r10, 0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32212;
	// 826335AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826335B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826335B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826335B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826335BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826335C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826335C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826335C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826335CC: 4BE33855  bl 0x82466e20
	ctx.lr = 0x826335D0;
	sub_82466E20(ctx, base);
	// 826335D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826335D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826335D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826335DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826335E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826335E0 size=112
    let mut pc: u32 = 0x826335E0;
    'dispatch: loop {
        match pc {
            0x826335E0 => {
    //   block [0x826335E0..0x82633650)
	// 826335E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826335E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826335E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826335EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826335F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826335F4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826335F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826335FC: 390BF160  addi r8, r11, -0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + -3744;
	// 82633600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82633604: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82633608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263360C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633618: 386A7E04  addi r3, r10, 0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + 32260;
	// 8263361C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263362C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263363C: 4BE337E5  bl 0x82466e20
	ctx.lr = 0x82633640;
	sub_82466E20(ctx, base);
	// 82633640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633650 size=108
    let mut pc: u32 = 0x82633650;
    'dispatch: loop {
        match pc {
            0x82633650 => {
    //   block [0x82633650..0x826336BC)
	// 82633650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263365C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633660: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633664: 38EBF1A8  addi r7, r11, -0xe58
	ctx.r[7].s64 = ctx.r[11].s64 + -3672;
	// 82633668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263366C: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82633670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263367C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633680: 386A7E34  addi r3, r10, 0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + 32308;
	// 82633684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263368C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263369C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826336A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826336A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826336A8: 4BE33779  bl 0x82466e20
	ctx.lr = 0x826336AC;
	sub_82466E20(ctx, base);
	// 826336AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826336B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826336B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826336B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826336C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826336C0 size=108
    let mut pc: u32 = 0x826336C0;
    'dispatch: loop {
        match pc {
            0x826336C0 => {
    //   block [0x826336C0..0x8263372C)
	// 826336C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826336C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826336C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826336CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826336D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826336D4: 38EBF1D8  addi r7, r11, -0xe28
	ctx.r[7].s64 = ctx.r[11].s64 + -3624;
	// 826336D8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826336DC: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 826336E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826336E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826336E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826336EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826336F0: 386A7E64  addi r3, r10, 0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + 32356;
	// 826336F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826336F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826336FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263370C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633718: 4BE33709  bl 0x82466e20
	ctx.lr = 0x8263371C;
	sub_82466E20(ctx, base);
	// 8263371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633730 size=112
    let mut pc: u32 = 0x82633730;
    'dispatch: loop {
        match pc {
            0x82633730 => {
    //   block [0x82633730..0x826337A0)
	// 82633730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263373C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633740: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633744: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263374C: 390BF268  addi r8, r11, -0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + -3480;
	// 82633750: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82633754: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82633758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263375C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633768: 386A7E94  addi r3, r10, 0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + 32404;
	// 8263376C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263377C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263378C: 4BE33695  bl 0x82466e20
	ctx.lr = 0x82633790;
	sub_82466E20(ctx, base);
	// 82633790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263379C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826337A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826337A0 size=112
    let mut pc: u32 = 0x826337A0;
    'dispatch: loop {
        match pc {
            0x826337A0 => {
    //   block [0x826337A0..0x82633810)
	// 826337A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826337A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826337A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826337AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826337B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826337B4: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826337B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826337BC: 390BF2F8  addi r8, r11, -0xd08
	ctx.r[8].s64 = ctx.r[11].s64 + -3336;
	// 826337C0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826337C4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826337C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826337CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826337D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826337D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826337D8: 386A7EC4  addi r3, r10, 0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 32452;
	// 826337DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826337E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826337E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826337E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826337EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826337F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826337F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826337F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826337FC: 4BE33625  bl 0x82466e20
	ctx.lr = 0x82633800;
	sub_82466E20(ctx, base);
	// 82633800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263380C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633810 size=100
    let mut pc: u32 = 0x82633810;
    'dispatch: loop {
        match pc {
            0x82633810 => {
    //   block [0x82633810..0x82633874)
	// 82633810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263381C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633824: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263382C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633830: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82633834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263383C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633844: 386A7EF4  addi r3, r10, 0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 32500;
	// 82633848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263384C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633850: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82633854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633858: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263385C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633860: 4BE335C1  bl 0x82466e20
	ctx.lr = 0x82633864;
	sub_82466E20(ctx, base);
	// 82633864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263386C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633878 size=112
    let mut pc: u32 = 0x82633878;
    'dispatch: loop {
        match pc {
            0x82633878 => {
    //   block [0x82633878..0x826338E8)
	// 82633878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263387C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633888: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263388C: 38AA7AD4  addi r5, r10, 0x7ad4
	ctx.r[5].s64 = ctx.r[10].s64 + 31444;
	// 82633890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633894: 390BF3B8  addi r8, r11, -0xc48
	ctx.r[8].s64 = ctx.r[11].s64 + -3144;
	// 82633898: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263389C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826338A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826338A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826338A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826338AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826338B0: 386A7F24  addi r3, r10, 0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + 32548;
	// 826338B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826338B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826338BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826338C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826338C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826338C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826338CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826338D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826338D4: 4BE3354D  bl 0x82466e20
	ctx.lr = 0x826338D8;
	sub_82466E20(ctx, base);
	// 826338D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826338DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826338E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826338E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826338E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826338E8 size=112
    let mut pc: u32 = 0x826338E8;
    'dispatch: loop {
        match pc {
            0x826338E8 => {
    //   block [0x826338E8..0x82633958)
	// 826338E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826338EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826338F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826338F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826338F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826338FC: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 82633900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633904: 390BF3E8  addi r8, r11, -0xc18
	ctx.r[8].s64 = ctx.r[11].s64 + -3096;
	// 82633908: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263390C: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82633910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633914: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263391C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633920: 386A7F54  addi r3, r10, 0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + 32596;
	// 82633924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263392C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263393C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633944: 4BE334DD  bl 0x82466e20
	ctx.lr = 0x82633948;
	sub_82466E20(ctx, base);
	// 82633948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263394C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633958 size=108
    let mut pc: u32 = 0x82633958;
    'dispatch: loop {
        match pc {
            0x82633958 => {
    //   block [0x82633958..0x826339C4)
	// 82633958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263395C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633964: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263396C: 38EBF400  addi r7, r11, -0xc00
	ctx.r[7].s64 = ctx.r[11].s64 + -3072;
	// 82633970: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82633974: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82633978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263397C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633988: 386A7F84  addi r3, r10, 0x7f84
	ctx.r[3].s64 = ctx.r[10].s64 + 32644;
	// 8263398C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263399C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826339A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826339A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826339A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826339AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826339B0: 4BE33471  bl 0x82466e20
	ctx.lr = 0x826339B4;
	sub_82466E20(ctx, base);
	// 826339B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826339B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826339BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826339C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826339C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826339C8 size=112
    let mut pc: u32 = 0x826339C8;
    'dispatch: loop {
        match pc {
            0x826339C8 => {
    //   block [0x826339C8..0x82633A38)
	// 826339C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826339CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826339D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826339D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826339D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826339DC: 38AA7EF4  addi r5, r10, 0x7ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 32500;
	// 826339E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826339E4: 390BF430  addi r8, r11, -0xbd0
	ctx.r[8].s64 = ctx.r[11].s64 + -3024;
	// 826339E8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826339EC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826339F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826339F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826339F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826339FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633A00: 386A7FB4  addi r3, r10, 0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32692;
	// 82633A04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633A24: 4BE333FD  bl 0x82466e20
	ctx.lr = 0x82633A28;
	sub_82466E20(ctx, base);
	// 82633A28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633A38 size=108
    let mut pc: u32 = 0x82633A38;
    'dispatch: loop {
        match pc {
            0x82633A38 => {
    //   block [0x82633A38..0x82633AA4)
	// 82633A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633A44: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633A48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633A4C: 38EBF4A8  addi r7, r11, -0xb58
	ctx.r[7].s64 = ctx.r[11].s64 + -2904;
	// 82633A50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82633A54: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 82633A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633A5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633A60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633A68: 386A7FE4  addi r3, r10, 0x7fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 32740;
	// 82633A6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633A90: 4BE33391  bl 0x82466e20
	ctx.lr = 0x82633A94;
	sub_82466E20(ctx, base);
	// 82633A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633AA8 size=108
    let mut pc: u32 = 0x82633AA8;
    'dispatch: loop {
        match pc {
            0x82633AA8 => {
    //   block [0x82633AA8..0x82633B14)
	// 82633AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633AB4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633ABC: 38EBF4F0  addi r7, r11, -0xb10
	ctx.r[7].s64 = ctx.r[11].s64 + -2832;
	// 82633AC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82633AC4: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 82633AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633ACC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633AD8: 386A8014  addi r3, r10, -0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + -32748;
	// 82633ADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633B00: 4BE33321  bl 0x82466e20
	ctx.lr = 0x82633B04;
	sub_82466E20(ctx, base);
	// 82633B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633B18 size=112
    let mut pc: u32 = 0x82633B18;
    'dispatch: loop {
        match pc {
            0x82633B18 => {
    //   block [0x82633B18..0x82633B88)
	// 82633B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633B28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633B2C: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633B34: 390BF520  addi r8, r11, -0xae0
	ctx.r[8].s64 = ctx.r[11].s64 + -2784;
	// 82633B38: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82633B3C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 82633B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633B50: 386A8044  addi r3, r10, -0x7fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -32700;
	// 82633B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633B74: 4BE332AD  bl 0x82466e20
	ctx.lr = 0x82633B78;
	sub_82466E20(ctx, base);
	// 82633B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633B88 size=108
    let mut pc: u32 = 0x82633B88;
    'dispatch: loop {
        match pc {
            0x82633B88 => {
    //   block [0x82633B88..0x82633BF4)
	// 82633B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633B9C: 38EBF5B0  addi r7, r11, -0xa50
	ctx.r[7].s64 = ctx.r[11].s64 + -2640;
	// 82633BA0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82633BA4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82633BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633BB8: 386A8074  addi r3, r10, -0x7f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32652;
	// 82633BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633BE0: 4BE33241  bl 0x82466e20
	ctx.lr = 0x82633BE4;
	sub_82466E20(ctx, base);
	// 82633BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633BF8 size=116
    let mut pc: u32 = 0x82633BF8;
    'dispatch: loop {
        match pc {
            0x82633BF8 => {
    //   block [0x82633BF8..0x82633C6C)
	// 82633BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633C04: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82633C08: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82633C0C: 390AF640  addi r8, r10, -0x9c0
	ctx.r[8].s64 = ctx.r[10].s64 + -2496;
	// 82633C10: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633C14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82633C18: 38AA7EF4  addi r5, r10, 0x7ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 32500;
	// 82633C1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633C20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82633C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633C2C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82633C30: 396B50A8  addi r11, r11, 0x50a8
	ctx.r[11].s64 = ctx.r[11].s64 + 20648;
	// 82633C34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633C3C: 386A80A4  addi r3, r10, -0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32604;
	// 82633C40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82633C44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633C48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82633C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633C58: 4BE331C9  bl 0x82466e20
	ctx.lr = 0x82633C5C;
	sub_82466E20(ctx, base);
	// 82633C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633C70 size=108
    let mut pc: u32 = 0x82633C70;
    'dispatch: loop {
        match pc {
            0x82633C70 => {
    //   block [0x82633C70..0x82633CDC)
	// 82633C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633C7C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633C84: 38EBF700  addi r7, r11, -0x900
	ctx.r[7].s64 = ctx.r[11].s64 + -2304;
	// 82633C88: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82633C8C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82633C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633C98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633CA0: 386A80D4  addi r3, r10, -0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32556;
	// 82633CA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633CC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633CC8: 4BE33159  bl 0x82466e20
	ctx.lr = 0x82633CCC;
	sub_82466E20(ctx, base);
	// 82633CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633CE0 size=112
    let mut pc: u32 = 0x82633CE0;
    'dispatch: loop {
        match pc {
            0x82633CE0 => {
    //   block [0x82633CE0..0x82633D50)
	// 82633CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633CEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633CF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633CF4: 38AA7EF4  addi r5, r10, 0x7ef4
	ctx.r[5].s64 = ctx.r[10].s64 + 32500;
	// 82633CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633CFC: 390BF748  addi r8, r11, -0x8b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2232;
	// 82633D00: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82633D04: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82633D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633D0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633D18: 386A8104  addi r3, r10, -0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + -32508;
	// 82633D1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633D3C: 4BE330E5  bl 0x82466e20
	ctx.lr = 0x82633D40;
	sub_82466E20(ctx, base);
	// 82633D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633D50 size=112
    let mut pc: u32 = 0x82633D50;
    'dispatch: loop {
        match pc {
            0x82633D50 => {
    //   block [0x82633D50..0x82633DC0)
	// 82633D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633D60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633D64: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633D6C: 390BF7A8  addi r8, r11, -0x858
	ctx.r[8].s64 = ctx.r[11].s64 + -2136;
	// 82633D70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82633D74: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82633D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633D7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633D88: 386A8134  addi r3, r10, -0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -32460;
	// 82633D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633DAC: 4BE33075  bl 0x82466e20
	ctx.lr = 0x82633DB0;
	sub_82466E20(ctx, base);
	// 82633DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633DC0 size=108
    let mut pc: u32 = 0x82633DC0;
    'dispatch: loop {
        match pc {
            0x82633DC0 => {
    //   block [0x82633DC0..0x82633E2C)
	// 82633DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633DCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633DD4: 38EBF7C0  addi r7, r11, -0x840
	ctx.r[7].s64 = ctx.r[11].s64 + -2112;
	// 82633DD8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82633DDC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82633DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82633DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633DF0: 386A8164  addi r3, r10, -0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32412;
	// 82633DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82633DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82633E18: 4BE33009  bl 0x82466e20
	ctx.lr = 0x82633E1C;
	sub_82466E20(ctx, base);
	// 82633E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633E30 size=112
    let mut pc: u32 = 0x82633E30;
    'dispatch: loop {
        match pc {
            0x82633E30 => {
    //   block [0x82633E30..0x82633EA0)
	// 82633E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633E40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633E44: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82633E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633E4C: 390BF838  addi r8, r11, -0x7c8
	ctx.r[8].s64 = ctx.r[11].s64 + -1992;
	// 82633E50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82633E54: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82633E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633E68: 386A8194  addi r3, r10, -0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32364;
	// 82633E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633E8C: 4BE32F95  bl 0x82466e20
	ctx.lr = 0x82633E90;
	sub_82466E20(ctx, base);
	// 82633E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633EA0 size=104
    let mut pc: u32 = 0x82633EA0;
    'dispatch: loop {
        match pc {
            0x82633EA0 => {
    //   block [0x82633EA0..0x82633F08)
	// 82633EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633EAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82633EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633EB4: 392A5100  addi r9, r10, 0x5100
	ctx.r[9].s64 = ctx.r[10].s64 + 20736;
	// 82633EB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82633EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633EC0: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82633EC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633ED4: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82633ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633EE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82633EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633EE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82633EEC: 386A81C4  addi r3, r10, -0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32316;
	// 82633EF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82633EF4: 4BE32F2D  bl 0x82466e20
	ctx.lr = 0x82633EF8;
	sub_82466E20(ctx, base);
	// 82633EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633F08 size=112
    let mut pc: u32 = 0x82633F08;
    'dispatch: loop {
        match pc {
            0x82633F08 => {
    //   block [0x82633F08..0x82633F78)
	// 82633F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633F14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633F18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633F1C: 38AA81C4  addi r5, r10, -0x7e3c
	ctx.r[5].s64 = ctx.r[10].s64 + -32316;
	// 82633F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633F24: 390BF86C  addi r8, r11, -0x794
	ctx.r[8].s64 = ctx.r[11].s64 + -1940;
	// 82633F28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82633F2C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82633F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633F40: 386A81F4  addi r3, r10, -0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32268;
	// 82633F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633F64: 4BE32EBD  bl 0x82466e20
	ctx.lr = 0x82633F68;
	sub_82466E20(ctx, base);
	// 82633F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633F78 size=112
    let mut pc: u32 = 0x82633F78;
    'dispatch: loop {
        match pc {
            0x82633F78 => {
    //   block [0x82633F78..0x82633FE8)
	// 82633F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633F84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633F88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633F8C: 38AA81F4  addi r5, r10, -0x7e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -32268;
	// 82633F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82633F94: 390BF8A0  addi r8, r11, -0x760
	ctx.r[8].s64 = ctx.r[11].s64 + -1888;
	// 82633F98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82633F9C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82633FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633FA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82633FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82633FB0: 386A8224  addi r3, r10, -0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -32220;
	// 82633FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82633FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82633FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82633FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82633FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82633FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82633FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82633FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82633FD4: 4BE32E4D  bl 0x82466e20
	ctx.lr = 0x82633FD8;
	sub_82466E20(ctx, base);
	// 82633FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633FE8 size=112
    let mut pc: u32 = 0x82633FE8;
    'dispatch: loop {
        match pc {
            0x82633FE8 => {
    //   block [0x82633FE8..0x82634058)
	// 82633FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82633FF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82633FFC: 38AA81F4  addi r5, r10, -0x7e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -32268;
	// 82634000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634004: 390BF900  addi r8, r11, -0x700
	ctx.r[8].s64 = ctx.r[11].s64 + -1792;
	// 82634008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263400C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82634010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263401C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634020: 386A8254  addi r3, r10, -0x7dac
	ctx.r[3].s64 = ctx.r[10].s64 + -32172;
	// 82634024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263402C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263403C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634044: 4BE32DDD  bl 0x82466e20
	ctx.lr = 0x82634048;
	sub_82466E20(ctx, base);
	// 82634048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263404C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634058 size=112
    let mut pc: u32 = 0x82634058;
    'dispatch: loop {
        match pc {
            0x82634058 => {
    //   block [0x82634058..0x826340C8)
	// 82634058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634068: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263406C: 38AA81F4  addi r5, r10, -0x7e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -32268;
	// 82634070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634074: 390BF930  addi r8, r11, -0x6d0
	ctx.r[8].s64 = ctx.r[11].s64 + -1744;
	// 82634078: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263407C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82634080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263408C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634090: 386A8284  addi r3, r10, -0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -32124;
	// 82634094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263409C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826340A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826340A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826340A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826340AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826340B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826340B4: 4BE32D6D  bl 0x82466e20
	ctx.lr = 0x826340B8;
	sub_82466E20(ctx, base);
	// 826340B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826340BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826340C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826340C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826340C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826340C8 size=112
    let mut pc: u32 = 0x826340C8;
    'dispatch: loop {
        match pc {
            0x826340C8 => {
    //   block [0x826340C8..0x82634138)
	// 826340C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826340CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826340D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826340D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826340D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826340DC: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 826340E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826340E4: 390BF978  addi r8, r11, -0x688
	ctx.r[8].s64 = ctx.r[11].s64 + -1672;
	// 826340E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826340EC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826340F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826340F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826340F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826340FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634100: 386A82B4  addi r3, r10, -0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32076;
	// 82634104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263410C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263411C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634124: 4BE32CFD  bl 0x82466e20
	ctx.lr = 0x82634128;
	sub_82466E20(ctx, base);
	// 82634128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263412C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634138 size=116
    let mut pc: u32 = 0x82634138;
    'dispatch: loop {
        match pc {
            0x82634138 => {
    //   block [0x82634138..0x826341AC)
	// 82634138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634144: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634148: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8263414C: 390AFA08  addi r8, r10, -0x5f8
	ctx.r[8].s64 = ctx.r[10].s64 + -1528;
	// 82634150: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634154: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82634158: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 8263415C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634160: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263416C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82634170: 396B5118  addi r11, r11, 0x5118
	ctx.r[11].s64 = ctx.r[11].s64 + 20760;
	// 82634174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263417C: 386A82E4  addi r3, r10, -0x7d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32028;
	// 82634180: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82634184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634188: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634198: 4BE32C89  bl 0x82466e20
	ctx.lr = 0x8263419C;
	sub_82466E20(ctx, base);
	// 8263419C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826341A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826341A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826341A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826341B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826341B0 size=100
    let mut pc: u32 = 0x826341B0;
    'dispatch: loop {
        match pc {
            0x826341B0 => {
    //   block [0x826341B0..0x82634214)
	// 826341B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826341B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826341B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826341BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826341C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826341C4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826341C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826341CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826341D0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826341D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826341D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826341DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826341E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826341E4: 386A8314  addi r3, r10, -0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + -31980;
	// 826341E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826341EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826341F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826341F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826341F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826341FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634200: 4BE32C21  bl 0x82466e20
	ctx.lr = 0x82634204;
	sub_82466E20(ctx, base);
	// 82634204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263420C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634218 size=100
    let mut pc: u32 = 0x82634218;
    'dispatch: loop {
        match pc {
            0x82634218 => {
    //   block [0x82634218..0x8263427C)
	// 82634218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263422C: 38AA83A4  addi r5, r10, -0x7c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -31836;
	// 82634230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634238: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8263423C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263424C: 386A8344  addi r3, r10, -0x7cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31932;
	// 82634250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634258: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263425C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634260: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634268: 4BE32BB9  bl 0x82466e20
	ctx.lr = 0x8263426C;
	sub_82466E20(ctx, base);
	// 8263426C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634280 size=100
    let mut pc: u32 = 0x82634280;
    'dispatch: loop {
        match pc {
            0x82634280 => {
    //   block [0x82634280..0x826342E4)
	// 82634280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263428C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634294: 38AA82E4  addi r5, r10, -0x7d1c
	ctx.r[5].s64 = ctx.r[10].s64 + -32028;
	// 82634298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263429C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826342A0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826342A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826342A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826342AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826342B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826342B4: 386A8374  addi r3, r10, -0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31884;
	// 826342B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826342BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826342C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826342C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826342C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826342CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826342D0: 4BE32B51  bl 0x82466e20
	ctx.lr = 0x826342D4;
	sub_82466E20(ctx, base);
	// 826342D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826342D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826342DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826342E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826342E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826342E8 size=104
    let mut pc: u32 = 0x826342E8;
    'dispatch: loop {
        match pc {
            0x826342E8 => {
    //   block [0x826342E8..0x82634350)
	// 826342E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826342EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826342F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826342F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826342F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826342FC: 392A5184  addi r9, r10, 0x5184
	ctx.r[9].s64 = ctx.r[10].s64 + 20868;
	// 82634300: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634308: 38AA8314  addi r5, r10, -0x7cec
	ctx.r[5].s64 = ctx.r[10].s64 + -31980;
	// 8263430C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263431C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82634320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634328: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263432C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634330: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634334: 386A83A4  addi r3, r10, -0x7c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -31836;
	// 82634338: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263433C: 4BE32AE5  bl 0x82466e20
	ctx.lr = 0x82634340;
	sub_82466E20(ctx, base);
	// 82634340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263434C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634350 size=108
    let mut pc: u32 = 0x82634350;
    'dispatch: loop {
        match pc {
            0x82634350 => {
    //   block [0x82634350..0x826343BC)
	// 82634350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263435C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634364: 38EBFBB8  addi r7, r11, -0x448
	ctx.r[7].s64 = ctx.r[11].s64 + -1096;
	// 82634368: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263436C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82634370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634378: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263437C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634380: 386A83D4  addi r3, r10, -0x7c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31788;
	// 82634384: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263438C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263439C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826343A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826343A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826343A8: 4BE32A79  bl 0x82466e20
	ctx.lr = 0x826343AC;
	sub_82466E20(ctx, base);
	// 826343AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826343B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826343B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826343B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826343C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826343C0 size=112
    let mut pc: u32 = 0x826343C0;
    'dispatch: loop {
        match pc {
            0x826343C0 => {
    //   block [0x826343C0..0x82634430)
	// 826343C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826343C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826343C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826343CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826343D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826343D4: 38AA83A4  addi r5, r10, -0x7c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -31836;
	// 826343D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826343DC: 390BFBE8  addi r8, r11, -0x418
	ctx.r[8].s64 = ctx.r[11].s64 + -1048;
	// 826343E0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826343E4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826343E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826343EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826343F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826343F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826343F8: 386A8404  addi r3, r10, -0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -31740;
	// 826343FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263440C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263441C: 4BE32A05  bl 0x82466e20
	ctx.lr = 0x82634420;
	sub_82466E20(ctx, base);
	// 82634420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263442C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634430 size=116
    let mut pc: u32 = 0x82634430;
    'dispatch: loop {
        match pc {
            0x82634430 => {
    //   block [0x82634430..0x826344A4)
	// 82634430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263443C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634440: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634444: 390BFC94  addi r8, r11, -0x36c
	ctx.r[8].s64 = ctx.r[11].s64 + -876;
	// 82634448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263444C: 392A51E8  addi r9, r10, 0x51e8
	ctx.r[9].s64 = ctx.r[10].s64 + 20968;
	// 82634450: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634454: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82634458: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8263445C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634464: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263446C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634474: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82634478: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8263447C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634480: 386B8434  addi r3, r11, -0x7bcc
	ctx.r[3].s64 = ctx.r[11].s64 + -31692;
	// 82634484: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634488: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263448C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634490: 4BE32991  bl 0x82466e20
	ctx.lr = 0x82634494;
	sub_82466E20(ctx, base);
	// 82634494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263449C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826344A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826344A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826344A8 size=112
    let mut pc: u32 = 0x826344A8;
    'dispatch: loop {
        match pc {
            0x826344A8 => {
    //   block [0x826344A8..0x82634518)
	// 826344A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826344AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826344B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826344B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826344B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826344BC: 38AA84F4  addi r5, r10, -0x7b0c
	ctx.r[5].s64 = ctx.r[10].s64 + -31500;
	// 826344C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826344C4: 390BFCAC  addi r8, r11, -0x354
	ctx.r[8].s64 = ctx.r[11].s64 + -852;
	// 826344C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826344CC: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826344D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826344D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826344D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826344DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826344E0: 386A8464  addi r3, r10, -0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -31644;
	// 826344E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826344E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826344EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826344F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826344F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826344F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826344FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634504: 4BE3291D  bl 0x82466e20
	ctx.lr = 0x82634508;
	sub_82466E20(ctx, base);
	// 82634508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263450C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634518 size=100
    let mut pc: u32 = 0x82634518;
    'dispatch: loop {
        match pc {
            0x82634518 => {
    //   block [0x82634518..0x8263457C)
	// 82634518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263451C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634524: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263452C: 38AA84C4  addi r5, r10, -0x7b3c
	ctx.r[5].s64 = ctx.r[10].s64 + -31548;
	// 82634530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634538: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8263453C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263454C: 386A8494  addi r3, r10, -0x7b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -31596;
	// 82634550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634558: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263455C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634560: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634568: 4BE328B9  bl 0x82466e20
	ctx.lr = 0x8263456C;
	sub_82466E20(ctx, base);
	// 8263456C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634580 size=112
    let mut pc: u32 = 0x82634580;
    'dispatch: loop {
        match pc {
            0x82634580 => {
    //   block [0x82634580..0x826345F0)
	// 82634580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263458C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634590: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634594: 38AA84F4  addi r5, r10, -0x7b0c
	ctx.r[5].s64 = ctx.r[10].s64 + -31500;
	// 82634598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263459C: 390BFCC4  addi r8, r11, -0x33c
	ctx.r[8].s64 = ctx.r[11].s64 + -828;
	// 826345A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826345A4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826345A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826345AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826345B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826345B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826345B8: 386A84C4  addi r3, r10, -0x7b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31548;
	// 826345BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826345C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826345C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826345C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826345CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826345D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826345D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826345D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826345DC: 4BE32845  bl 0x82466e20
	ctx.lr = 0x826345E0;
	sub_82466E20(ctx, base);
	// 826345E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826345E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826345E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826345EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826345F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826345F0 size=112
    let mut pc: u32 = 0x826345F0;
    'dispatch: loop {
        match pc {
            0x826345F0 => {
    //   block [0x826345F0..0x82634660)
	// 826345F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826345F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826345F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826345FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634600: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634604: 38AA8434  addi r5, r10, -0x7bcc
	ctx.r[5].s64 = ctx.r[10].s64 + -31692;
	// 82634608: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263460C: 390BFCF8  addi r8, r11, -0x308
	ctx.r[8].s64 = ctx.r[11].s64 + -776;
	// 82634610: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82634614: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82634618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263461C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634628: 386A84F4  addi r3, r10, -0x7b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31500;
	// 8263462C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263463C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263464C: 4BE327D5  bl 0x82466e20
	ctx.lr = 0x82634650;
	sub_82466E20(ctx, base);
	// 82634650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263465C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634660 size=100
    let mut pc: u32 = 0x82634660;
    'dispatch: loop {
        match pc {
            0x82634660 => {
    //   block [0x82634660..0x826346C4)
	// 82634660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263466C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634674: 38AA84F4  addi r5, r10, -0x7b0c
	ctx.r[5].s64 = ctx.r[10].s64 + -31500;
	// 82634678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263467C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634680: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82634684: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263468C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634694: 386A8524  addi r3, r10, -0x7adc
	ctx.r[3].s64 = ctx.r[10].s64 + -31452;
	// 82634698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263469C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826346A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826346A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826346A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826346AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826346B0: 4BE32771  bl 0x82466e20
	ctx.lr = 0x826346B4;
	sub_82466E20(ctx, base);
	// 826346B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826346B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826346BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826346C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826346C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826346C8 size=100
    let mut pc: u32 = 0x826346C8;
    'dispatch: loop {
        match pc {
            0x826346C8 => {
    //   block [0x826346C8..0x8263472C)
	// 826346C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826346CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826346D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826346D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826346D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826346DC: 38AA8464  addi r5, r10, -0x7b9c
	ctx.r[5].s64 = ctx.r[10].s64 + -31644;
	// 826346E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826346E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826346E8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826346EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826346F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826346F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826346F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826346FC: 386A8554  addi r3, r10, -0x7aac
	ctx.r[3].s64 = ctx.r[10].s64 + -31404;
	// 82634700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634708: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263470C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634710: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634718: 4BE32709  bl 0x82466e20
	ctx.lr = 0x8263471C;
	sub_82466E20(ctx, base);
	// 8263471C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634730 size=100
    let mut pc: u32 = 0x82634730;
    'dispatch: loop {
        match pc {
            0x82634730 => {
    //   block [0x82634730..0x82634794)
	// 82634730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263473C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634744: 38AA8524  addi r5, r10, -0x7adc
	ctx.r[5].s64 = ctx.r[10].s64 + -31452;
	// 82634748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263474C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634750: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82634754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263475C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634764: 386A8584  addi r3, r10, -0x7a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -31356;
	// 82634768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263476C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634770: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82634774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634778: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263477C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634780: 4BE326A1  bl 0x82466e20
	ctx.lr = 0x82634784;
	sub_82466E20(ctx, base);
	// 82634784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263478C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634798 size=100
    let mut pc: u32 = 0x82634798;
    'dispatch: loop {
        match pc {
            0x82634798 => {
    //   block [0x82634798..0x826347FC)
	// 82634798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826347A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826347A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826347A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826347AC: 38AA8464  addi r5, r10, -0x7b9c
	ctx.r[5].s64 = ctx.r[10].s64 + -31644;
	// 826347B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826347B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826347B8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826347BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826347C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826347C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826347C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826347CC: 386A85B4  addi r3, r10, -0x7a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31308;
	// 826347D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826347D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826347D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826347DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826347E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826347E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826347E8: 4BE32639  bl 0x82466e20
	ctx.lr = 0x826347EC;
	sub_82466E20(ctx, base);
	// 826347EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826347F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826347F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826347F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634800 size=112
    let mut pc: u32 = 0x82634800;
    'dispatch: loop {
        match pc {
            0x82634800 => {
    //   block [0x82634800..0x82634870)
	// 82634800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263480C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634810: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634814: 38AA8644  addi r5, r10, -0x79bc
	ctx.r[5].s64 = ctx.r[10].s64 + -31164;
	// 82634818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263481C: 390BFDA0  addi r8, r11, -0x260
	ctx.r[8].s64 = ctx.r[11].s64 + -608;
	// 82634820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82634824: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82634828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263482C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634838: 386A85E4  addi r3, r10, -0x7a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31260;
	// 8263483C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263484C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263485C: 4BE325C5  bl 0x82466e20
	ctx.lr = 0x82634860;
	sub_82466E20(ctx, base);
	// 82634860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263486C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634870 size=112
    let mut pc: u32 = 0x82634870;
    'dispatch: loop {
        match pc {
            0x82634870 => {
    //   block [0x82634870..0x826348E0)
	// 82634870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263487C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634880: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634884: 38AA8674  addi r5, r10, -0x798c
	ctx.r[5].s64 = ctx.r[10].s64 + -31116;
	// 82634888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263488C: 390BFDD0  addi r8, r11, -0x230
	ctx.r[8].s64 = ctx.r[11].s64 + -560;
	// 82634890: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634894: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82634898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263489C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826348A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826348A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826348A8: 386A8614  addi r3, r10, -0x79ec
	ctx.r[3].s64 = ctx.r[10].s64 + -31212;
	// 826348AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826348B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826348B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826348B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826348BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826348C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826348C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826348C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826348CC: 4BE32555  bl 0x82466e20
	ctx.lr = 0x826348D0;
	sub_82466E20(ctx, base);
	// 826348D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826348D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826348D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826348DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826348E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826348E0 size=112
    let mut pc: u32 = 0x826348E0;
    'dispatch: loop {
        match pc {
            0x826348E0 => {
    //   block [0x826348E0..0x82634950)
	// 826348E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826348E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826348E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826348EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826348F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826348F4: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 826348F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826348FC: 390BFDE8  addi r8, r11, -0x218
	ctx.r[8].s64 = ctx.r[11].s64 + -536;
	// 82634900: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82634904: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82634908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263490C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634918: 386A8644  addi r3, r10, -0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + -31164;
	// 8263491C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263492C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263493C: 4BE324E5  bl 0x82466e20
	ctx.lr = 0x82634940;
	sub_82466E20(ctx, base);
	// 82634940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263494C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634950 size=112
    let mut pc: u32 = 0x82634950;
    'dispatch: loop {
        match pc {
            0x82634950 => {
    //   block [0x82634950..0x826349C0)
	// 82634950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263495C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634960: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634964: 38AA8644  addi r5, r10, -0x79bc
	ctx.r[5].s64 = ctx.r[10].s64 + -31164;
	// 82634968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263496C: 390BFE18  addi r8, r11, -0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + -488;
	// 82634970: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634974: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82634978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263497C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634988: 386A8674  addi r3, r10, -0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + -31116;
	// 8263498C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263499C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826349A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826349A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826349A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826349AC: 4BE32475  bl 0x82466e20
	ctx.lr = 0x826349B0;
	sub_82466E20(ctx, base);
	// 826349B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826349B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826349B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826349BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826349C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826349C0 size=112
    let mut pc: u32 = 0x826349C0;
    'dispatch: loop {
        match pc {
            0x826349C0 => {
    //   block [0x826349C0..0x82634A30)
	// 826349C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826349C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826349C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826349CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826349D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826349D4: 38AA8674  addi r5, r10, -0x798c
	ctx.r[5].s64 = ctx.r[10].s64 + -31116;
	// 826349D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826349DC: 390BFE30  addi r8, r11, -0x1d0
	ctx.r[8].s64 = ctx.r[11].s64 + -464;
	// 826349E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826349E4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826349E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826349EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826349F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826349F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826349F8: 386A86A4  addi r3, r10, -0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + -31068;
	// 826349FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634A1C: 4BE32405  bl 0x82466e20
	ctx.lr = 0x82634A20;
	sub_82466E20(ctx, base);
	// 82634A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634A30 size=112
    let mut pc: u32 = 0x82634A30;
    'dispatch: loop {
        match pc {
            0x82634A30 => {
    //   block [0x82634A30..0x82634AA0)
	// 82634A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634A3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634A40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634A44: 392A5214  addi r9, r10, 0x5214
	ctx.r[9].s64 = ctx.r[10].s64 + 21012;
	// 82634A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634A4C: 390BFE48  addi r8, r11, -0x1b8
	ctx.r[8].s64 = ctx.r[11].s64 + -440;
	// 82634A50: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82634A54: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82634A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634A5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634A68: 386A86D4  addi r3, r10, -0x792c
	ctx.r[3].s64 = ctx.r[10].s64 + -31020;
	// 82634A6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634A70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634A8C: 4BE32395  bl 0x82466e20
	ctx.lr = 0x82634A90;
	sub_82466E20(ctx, base);
	// 82634A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634AA0 size=116
    let mut pc: u32 = 0x82634AA0;
    'dispatch: loop {
        match pc {
            0x82634AA0 => {
    //   block [0x82634AA0..0x82634B14)
	// 82634AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634AAC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634AB0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82634AB4: 390AFE78  addi r8, r10, -0x188
	ctx.r[8].s64 = ctx.r[10].s64 + -392;
	// 82634AB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634ABC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82634AC0: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634AC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634AC8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634AD4: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82634AD8: 396B5228  addi r11, r11, 0x5228
	ctx.r[11].s64 = ctx.r[11].s64 + 21032;
	// 82634ADC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634AE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634AE4: 386A8704  addi r3, r10, -0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30972;
	// 82634AE8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82634AEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634AF0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82634AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634B00: 4BE32321  bl 0x82466e20
	ctx.lr = 0x82634B04;
	sub_82466E20(ctx, base);
	// 82634B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82634B18 size=48
    let mut pc: u32 = 0x82634B18;
    'dispatch: loop {
        match pc {
            0x82634B18 => {
    //   block [0x82634B18..0x82634B48)
	// 82634B18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634B1C: 814BFF2C  lwz r10, -0xd4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82634B20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634B24: 396B2518  addi r11, r11, 0x2518
	ctx.r[11].s64 = ctx.r[11].s64 + 9496;
	// 82634B28: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82634B2C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634B30: 814AFF28  lwz r10, -0xd8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-216 as u32) ) } as u64;
	// 82634B34: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82634B38: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82634B3C: 814AFF24  lwz r10, -0xdc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-220 as u32) ) } as u64;
	// 82634B40: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 82634B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634B48 size=116
    let mut pc: u32 = 0x82634B48;
    'dispatch: loop {
        match pc {
            0x82634B48 => {
    //   block [0x82634B48..0x82634BBC)
	// 82634B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634B54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82634B58: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634B5C: 392B5328  addi r9, r11, 0x5328
	ctx.r[9].s64 = ctx.r[11].s64 + 21288;
	// 82634B60: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634B64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634B68: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82634B6C: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82634B70: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634B74: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82634B78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634B7C: 396B2518  addi r11, r11, 0x2518
	ctx.r[11].s64 = ctx.r[11].s64 + 9496;
	// 82634B80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82634B84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634B88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82634B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634B90: 386A8734  addi r3, r10, -0x78cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30924;
	// 82634B94: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82634B98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82634B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634BA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82634BA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634BA8: 4BE32279  bl 0x82466e20
	ctx.lr = 0x82634BAC;
	sub_82466E20(ctx, base);
	// 82634BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634BC0 size=116
    let mut pc: u32 = 0x82634BC0;
    'dispatch: loop {
        match pc {
            0x82634BC0 => {
    //   block [0x82634BC0..0x82634C34)
	// 82634BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634BCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634BD4: 390BFF38  addi r8, r11, -0xc8
	ctx.r[8].s64 = ctx.r[11].s64 + -200;
	// 82634BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634BDC: 392A5410  addi r9, r10, 0x5410
	ctx.r[9].s64 = ctx.r[10].s64 + 21520;
	// 82634BE0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634BE4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82634BE8: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634BEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634BF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634C04: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82634C08: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82634C0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82634C10: 386B8764  addi r3, r11, -0x789c
	ctx.r[3].s64 = ctx.r[11].s64 + -30876;
	// 82634C14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634C18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634C20: 4BE32201  bl 0x82466e20
	ctx.lr = 0x82634C24;
	sub_82466E20(ctx, base);
	// 82634C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634C38 size=112
    let mut pc: u32 = 0x82634C38;
    'dispatch: loop {
        match pc {
            0x82634C38 => {
    //   block [0x82634C38..0x82634CA8)
	// 82634C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634C44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634C48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634C4C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634C54: 390BFFC8  addi r8, r11, -0x38
	ctx.r[8].s64 = ctx.r[11].s64 + -56;
	// 82634C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634C5C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82634C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634C70: 386A8794  addi r3, r10, -0x786c
	ctx.r[3].s64 = ctx.r[10].s64 + -30828;
	// 82634C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634C94: 4BE3218D  bl 0x82466e20
	ctx.lr = 0x82634C98;
	sub_82466E20(ctx, base);
	// 82634C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634CA8 size=112
    let mut pc: u32 = 0x82634CA8;
    'dispatch: loop {
        match pc {
            0x82634CA8 => {
    //   block [0x82634CA8..0x82634D18)
	// 82634CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634CB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634CB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634CBC: 38AA6F64  addi r5, r10, 0x6f64
	ctx.r[5].s64 = ctx.r[10].s64 + 28516;
	// 82634CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634CC4: 390BFFE0  addi r8, r11, -0x20
	ctx.r[8].s64 = ctx.r[11].s64 + -32;
	// 82634CC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82634CCC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82634CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634CE0: 386A87C4  addi r3, r10, -0x783c
	ctx.r[3].s64 = ctx.r[10].s64 + -30780;
	// 82634CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634D04: 4BE3211D  bl 0x82466e20
	ctx.lr = 0x82634D08;
	sub_82466E20(ctx, base);
	// 82634D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D18 size=108
    let mut pc: u32 = 0x82634D18;
    'dispatch: loop {
        match pc {
            0x82634D18 => {
    //   block [0x82634D18..0x82634D84)
	// 82634D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634D2C: 38EBFFF8  addi r7, r11, -8
	ctx.r[7].s64 = ctx.r[11].s64 + -8;
	// 82634D30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634D34: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82634D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634D48: 386A87F4  addi r3, r10, -0x780c
	ctx.r[3].s64 = ctx.r[10].s64 + -30732;
	// 82634D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634D70: 4BE320B1  bl 0x82466e20
	ctx.lr = 0x82634D74;
	sub_82466E20(ctx, base);
	// 82634D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D88 size=112
    let mut pc: u32 = 0x82634D88;
    'dispatch: loop {
        match pc {
            0x82634D88 => {
    //   block [0x82634D88..0x82634DF8)
	// 82634D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634D98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634D9C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634DA4: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82634DA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82634DAC: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82634DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634DC0: 386A8824  addi r3, r10, -0x77dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30684;
	// 82634DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634DE4: 4BE3203D  bl 0x82466e20
	ctx.lr = 0x82634DE8;
	sub_82466E20(ctx, base);
	// 82634DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634DF8 size=108
    let mut pc: u32 = 0x82634DF8;
    'dispatch: loop {
        match pc {
            0x82634DF8 => {
    //   block [0x82634DF8..0x82634E64)
	// 82634DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634E04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634E0C: 38EB0058  addi r7, r11, 0x58
	ctx.r[7].s64 = ctx.r[11].s64 + 88;
	// 82634E10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634E14: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82634E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634E28: 386A8854  addi r3, r10, -0x77ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30636;
	// 82634E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634E50: 4BE31FD1  bl 0x82466e20
	ctx.lr = 0x82634E54;
	sub_82466E20(ctx, base);
	// 82634E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634E68 size=112
    let mut pc: u32 = 0x82634E68;
    'dispatch: loop {
        match pc {
            0x82634E68 => {
    //   block [0x82634E68..0x82634ED8)
	// 82634E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634E74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82634E78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634E7C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82634E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82634E84: 390B0070  addi r8, r11, 0x70
	ctx.r[8].s64 = ctx.r[11].s64 + 112;
	// 82634E88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82634E8C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82634E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634E94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634EA0: 386A8884  addi r3, r10, -0x777c
	ctx.r[3].s64 = ctx.r[10].s64 + -30588;
	// 82634EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634EC4: 4BE31F5D  bl 0x82466e20
	ctx.lr = 0x82634EC8;
	sub_82466E20(ctx, base);
	// 82634EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634ED8 size=112
    let mut pc: u32 = 0x82634ED8;
    'dispatch: loop {
        match pc {
            0x82634ED8 => {
    //   block [0x82634ED8..0x82634F48)
	// 82634ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634EE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634EE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634EEC: 38AA8944  addi r5, r10, -0x76bc
	ctx.r[5].s64 = ctx.r[10].s64 + -30396;
	// 82634EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634EF4: 390B00A0  addi r8, r11, 0xa0
	ctx.r[8].s64 = ctx.r[11].s64 + 160;
	// 82634EF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82634EFC: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82634F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634F04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82634F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634F10: 386A88B4  addi r3, r10, -0x774c
	ctx.r[3].s64 = ctx.r[10].s64 + -30540;
	// 82634F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82634F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634F34: 4BE31EED  bl 0x82466e20
	ctx.lr = 0x82634F38;
	sub_82466E20(ctx, base);
	// 82634F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634F48 size=108
    let mut pc: u32 = 0x82634F48;
    'dispatch: loop {
        match pc {
            0x82634F48 => {
    //   block [0x82634F48..0x82634FB4)
	// 82634F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634F54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634F5C: 38EB0118  addi r7, r11, 0x118
	ctx.r[7].s64 = ctx.r[11].s64 + 280;
	// 82634F60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82634F64: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82634F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634F6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634F78: 386A88E4  addi r3, r10, -0x771c
	ctx.r[3].s64 = ctx.r[10].s64 + -30492;
	// 82634F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82634F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82634F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82634F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82634F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634FA0: 4BE31E81  bl 0x82466e20
	ctx.lr = 0x82634FA4;
	sub_82466E20(ctx, base);
	// 82634FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82634FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634FB8 size=108
    let mut pc: u32 = 0x82634FB8;
    'dispatch: loop {
        match pc {
            0x82634FB8 => {
    //   block [0x82634FB8..0x82635024)
	// 82634FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634FC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82634FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82634FCC: 38EB0160  addi r7, r11, 0x160
	ctx.r[7].s64 = ctx.r[11].s64 + 352;
	// 82634FD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82634FD4: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82634FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634FDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82634FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82634FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82634FE8: 386A8914  addi r3, r10, -0x76ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30444;
	// 82634FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82634FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82634FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82634FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263500C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635010: 4BE31E11  bl 0x82466e20
	ctx.lr = 0x82635014;
	sub_82466E20(ctx, base);
	// 82635014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263501C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635028 size=116
    let mut pc: u32 = 0x82635028;
    'dispatch: loop {
        match pc {
            0x82635028 => {
    //   block [0x82635028..0x8263509C)
	// 82635028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263502C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635034: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82635038: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263503C: 390A01A8  addi r8, r10, 0x1a8
	ctx.r[8].s64 = ctx.r[10].s64 + 424;
	// 82635040: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635044: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82635048: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 8263504C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635050: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82635054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263505C: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82635060: 396B5424  addi r11, r11, 0x5424
	ctx.r[11].s64 = ctx.r[11].s64 + 21540;
	// 82635064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635068: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263506C: 386A8944  addi r3, r10, -0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30396;
	// 82635070: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82635074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635078: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263507C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635088: 4BE31D99  bl 0x82466e20
	ctx.lr = 0x8263508C;
	sub_82466E20(ctx, base);
	// 8263508C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826350A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826350A0 size=108
    let mut pc: u32 = 0x826350A0;
    'dispatch: loop {
        match pc {
            0x826350A0 => {
    //   block [0x826350A0..0x8263510C)
	// 826350A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826350A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826350A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826350AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826350B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826350B4: 38EB0280  addi r7, r11, 0x280
	ctx.r[7].s64 = ctx.r[11].s64 + 640;
	// 826350B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826350BC: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 826350C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826350C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826350C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826350CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826350D0: 386A8974  addi r3, r10, -0x768c
	ctx.r[3].s64 = ctx.r[10].s64 + -30348;
	// 826350D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826350D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826350DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826350E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826350E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826350E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826350EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826350F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826350F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826350F8: 4BE31D29  bl 0x82466e20
	ctx.lr = 0x826350FC;
	sub_82466E20(ctx, base);
	// 826350FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635110 size=108
    let mut pc: u32 = 0x82635110;
    'dispatch: loop {
        match pc {
            0x82635110 => {
    //   block [0x82635110..0x8263517C)
	// 82635110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263511C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82635124: 38EB02B0  addi r7, r11, 0x2b0
	ctx.r[7].s64 = ctx.r[11].s64 + 688;
	// 82635128: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263512C: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 82635130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635134: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263513C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635140: 386A89A4  addi r3, r10, -0x765c
	ctx.r[3].s64 = ctx.r[10].s64 + -30300;
	// 82635144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263514C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263515C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635168: 4BE31CB9  bl 0x82466e20
	ctx.lr = 0x8263516C;
	sub_82466E20(ctx, base);
	// 8263516C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635180 size=112
    let mut pc: u32 = 0x82635180;
    'dispatch: loop {
        match pc {
            0x82635180 => {
    //   block [0x82635180..0x826351F0)
	// 82635180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263518C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635190: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635194: 38AA7A74  addi r5, r10, 0x7a74
	ctx.r[5].s64 = ctx.r[10].s64 + 31348;
	// 82635198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263519C: 390B02E0  addi r8, r11, 0x2e0
	ctx.r[8].s64 = ctx.r[11].s64 + 736;
	// 826351A0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826351A4: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826351A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826351AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826351B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826351B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826351B8: 386A89D4  addi r3, r10, -0x762c
	ctx.r[3].s64 = ctx.r[10].s64 + -30252;
	// 826351BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826351C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826351C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826351C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826351CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826351D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826351D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826351D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826351DC: 4BE31C45  bl 0x82466e20
	ctx.lr = 0x826351E0;
	sub_82466E20(ctx, base);
	// 826351E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826351E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826351E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826351EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826351F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826351F0 size=112
    let mut pc: u32 = 0x826351F0;
    'dispatch: loop {
        match pc {
            0x826351F0 => {
    //   block [0x826351F0..0x82635260)
	// 826351F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826351F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826351F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826351FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635200: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635204: 38AA7984  addi r5, r10, 0x7984
	ctx.r[5].s64 = ctx.r[10].s64 + 31108;
	// 82635208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263520C: 390B03B8  addi r8, r11, 0x3b8
	ctx.r[8].s64 = ctx.r[11].s64 + 952;
	// 82635210: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82635214: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82635218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263521C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635228: 386A8A04  addi r3, r10, -0x75fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30204;
	// 8263522C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263523C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263524C: 4BE31BD5  bl 0x82466e20
	ctx.lr = 0x82635250;
	sub_82466E20(ctx, base);
	// 82635250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263525C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635260 size=112
    let mut pc: u32 = 0x82635260;
    'dispatch: loop {
        match pc {
            0x82635260 => {
    //   block [0x82635260..0x826352D0)
	// 82635260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263526C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635270: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635274: 38AA7984  addi r5, r10, 0x7984
	ctx.r[5].s64 = ctx.r[10].s64 + 31108;
	// 82635278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263527C: 390B0400  addi r8, r11, 0x400
	ctx.r[8].s64 = ctx.r[11].s64 + 1024;
	// 82635280: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82635284: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82635288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263528C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635298: 386A8A34  addi r3, r10, -0x75cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30156;
	// 8263529C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826352A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826352A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826352A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826352AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826352B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826352B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826352B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826352BC: 4BE31B65  bl 0x82466e20
	ctx.lr = 0x826352C0;
	sub_82466E20(ctx, base);
	// 826352C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826352C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826352C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826352CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826352D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826352D0 size=112
    let mut pc: u32 = 0x826352D0;
    'dispatch: loop {
        match pc {
            0x826352D0 => {
    //   block [0x826352D0..0x82635340)
	// 826352D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826352D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826352D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826352DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826352E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826352E4: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 826352E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826352EC: 390B0460  addi r8, r11, 0x460
	ctx.r[8].s64 = ctx.r[11].s64 + 1120;
	// 826352F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826352F4: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826352F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826352FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635308: 386A8A64  addi r3, r10, -0x759c
	ctx.r[3].s64 = ctx.r[10].s64 + -30108;
	// 8263530C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263531C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263532C: 4BE31AF5  bl 0x82466e20
	ctx.lr = 0x82635330;
	sub_82466E20(ctx, base);
	// 82635330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263533C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635340 size=112
    let mut pc: u32 = 0x82635340;
    'dispatch: loop {
        match pc {
            0x82635340 => {
    //   block [0x82635340..0x826353B0)
	// 82635340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263534C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635350: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635354: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 82635358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263535C: 390B04C0  addi r8, r11, 0x4c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1216;
	// 82635360: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82635364: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82635368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263536C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635378: 386A8A94  addi r3, r10, -0x756c
	ctx.r[3].s64 = ctx.r[10].s64 + -30060;
	// 8263537C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263538C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263539C: 4BE31A85  bl 0x82466e20
	ctx.lr = 0x826353A0;
	sub_82466E20(ctx, base);
	// 826353A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826353A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826353A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826353AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826353B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826353B0 size=112
    let mut pc: u32 = 0x826353B0;
    'dispatch: loop {
        match pc {
            0x826353B0 => {
    //   block [0x826353B0..0x82635420)
	// 826353B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826353B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826353B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826353BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826353C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826353C4: 38AA7984  addi r5, r10, 0x7984
	ctx.r[5].s64 = ctx.r[10].s64 + 31108;
	// 826353C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826353CC: 390B0520  addi r8, r11, 0x520
	ctx.r[8].s64 = ctx.r[11].s64 + 1312;
	// 826353D0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826353D4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826353D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826353DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826353E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826353E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826353E8: 386A8AC4  addi r3, r10, -0x753c
	ctx.r[3].s64 = ctx.r[10].s64 + -30012;
	// 826353EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826353F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826353F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826353F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826353FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263540C: 4BE31A15  bl 0x82466e20
	ctx.lr = 0x82635410;
	sub_82466E20(ctx, base);
	// 82635410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263541C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635420 size=108
    let mut pc: u32 = 0x82635420;
    'dispatch: loop {
        match pc {
            0x82635420 => {
    //   block [0x82635420..0x8263548C)
	// 82635420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263542C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635434: 38EB05E0  addi r7, r11, 0x5e0
	ctx.r[7].s64 = ctx.r[11].s64 + 1504;
	// 82635438: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8263543C: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82635440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263544C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635450: 386A8AF4  addi r3, r10, -0x750c
	ctx.r[3].s64 = ctx.r[10].s64 + -29964;
	// 82635454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263545C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263546C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635478: 4BE319A9  bl 0x82466e20
	ctx.lr = 0x8263547C;
	sub_82466E20(ctx, base);
	// 8263547C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635490 size=112
    let mut pc: u32 = 0x82635490;
    'dispatch: loop {
        match pc {
            0x82635490 => {
    //   block [0x82635490..0x82635500)
	// 82635490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263549C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826354A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826354A4: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 826354A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826354AC: 390B0778  addi r8, r11, 0x778
	ctx.r[8].s64 = ctx.r[11].s64 + 1912;
	// 826354B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826354B4: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826354B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826354BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826354C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826354C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826354C8: 386A8B24  addi r3, r10, -0x74dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29916;
	// 826354CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826354D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826354D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826354D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826354DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826354E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826354E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826354E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826354EC: 4BE31935  bl 0x82466e20
	ctx.lr = 0x826354F0;
	sub_82466E20(ctx, base);
	// 826354F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826354F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826354F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826354FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635500 size=112
    let mut pc: u32 = 0x82635500;
    'dispatch: loop {
        match pc {
            0x82635500 => {
    //   block [0x82635500..0x82635570)
	// 82635500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263550C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635510: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635514: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82635518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263551C: 390B0790  addi r8, r11, 0x790
	ctx.r[8].s64 = ctx.r[11].s64 + 1936;
	// 82635520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635524: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82635528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263552C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635538: 386A8B54  addi r3, r10, -0x74ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29868;
	// 8263553C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263554C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82635550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263555C: 4BE318C5  bl 0x82466e20
	ctx.lr = 0x82635560;
	sub_82466E20(ctx, base);
	// 82635560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635570 size=112
    let mut pc: u32 = 0x82635570;
    'dispatch: loop {
        match pc {
            0x82635570 => {
    //   block [0x82635570..0x826355E0)
	// 82635570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263557C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635580: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635584: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82635588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263558C: 390B07A8  addi r8, r11, 0x7a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1960;
	// 82635590: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82635594: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82635598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263559C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826355A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826355A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826355A8: 386A8B84  addi r3, r10, -0x747c
	ctx.r[3].s64 = ctx.r[10].s64 + -29820;
	// 826355AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826355B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826355B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826355B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826355BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826355C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826355C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826355C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826355CC: 4BE31855  bl 0x82466e20
	ctx.lr = 0x826355D0;
	sub_82466E20(ctx, base);
	// 826355D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826355D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826355D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826355DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826355E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826355E0 size=108
    let mut pc: u32 = 0x826355E0;
    'dispatch: loop {
        match pc {
            0x826355E0 => {
    //   block [0x826355E0..0x8263564C)
	// 826355E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826355E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826355E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826355EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826355F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826355F4: 38EB07D8  addi r7, r11, 0x7d8
	ctx.r[7].s64 = ctx.r[11].s64 + 2008;
	// 826355F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826355FC: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82635600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263560C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635610: 386A8BB4  addi r3, r10, -0x744c
	ctx.r[3].s64 = ctx.r[10].s64 + -29772;
	// 82635614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263561C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263562C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635638: 4BE317E9  bl 0x82466e20
	ctx.lr = 0x8263563C;
	sub_82466E20(ctx, base);
	// 8263563C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635650 size=112
    let mut pc: u32 = 0x82635650;
    'dispatch: loop {
        match pc {
            0x82635650 => {
    //   block [0x82635650..0x826356C0)
	// 82635650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263565C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635660: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635664: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82635668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263566C: 390B0808  addi r8, r11, 0x808
	ctx.r[8].s64 = ctx.r[11].s64 + 2056;
	// 82635670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635674: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82635678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263567C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635688: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 8263568C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263569C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826356A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826356A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826356A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826356AC: 4BE31775  bl 0x82466e20
	ctx.lr = 0x826356B0;
	sub_82466E20(ctx, base);
	// 826356B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826356B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826356B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826356BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826356C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826356C0 size=108
    let mut pc: u32 = 0x826356C0;
    'dispatch: loop {
        match pc {
            0x826356C0 => {
    //   block [0x826356C0..0x8263572C)
	// 826356C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826356C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826356C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826356CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826356D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826356D4: 38EB0820  addi r7, r11, 0x820
	ctx.r[7].s64 = ctx.r[11].s64 + 2080;
	// 826356D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826356DC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826356E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826356E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826356E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826356EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826356F0: 386A8C14  addi r3, r10, -0x73ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29676;
	// 826356F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826356F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826356FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263570C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635718: 4BE31709  bl 0x82466e20
	ctx.lr = 0x8263571C;
	sub_82466E20(ctx, base);
	// 8263571C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635730 size=108
    let mut pc: u32 = 0x82635730;
    'dispatch: loop {
        match pc {
            0x82635730 => {
    //   block [0x82635730..0x8263579C)
	// 82635730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263573C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635744: 38EB0850  addi r7, r11, 0x850
	ctx.r[7].s64 = ctx.r[11].s64 + 2128;
	// 82635748: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263574C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82635750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263575C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635760: 386A8C44  addi r3, r10, -0x73bc
	ctx.r[3].s64 = ctx.r[10].s64 + -29628;
	// 82635764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263576C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263577C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635788: 4BE31699  bl 0x82466e20
	ctx.lr = 0x8263578C;
	sub_82466E20(ctx, base);
	// 8263578C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826357A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826357A0 size=112
    let mut pc: u32 = 0x826357A0;
    'dispatch: loop {
        match pc {
            0x826357A0 => {
    //   block [0x826357A0..0x82635810)
	// 826357A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826357A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826357A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826357AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826357B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826357B4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826357B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826357BC: 390B0898  addi r8, r11, 0x898
	ctx.r[8].s64 = ctx.r[11].s64 + 2200;
	// 826357C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826357C4: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826357C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826357CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826357D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826357D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826357D8: 386A8C74  addi r3, r10, -0x738c
	ctx.r[3].s64 = ctx.r[10].s64 + -29580;
	// 826357DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826357E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826357E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826357E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826357EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826357F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826357F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826357F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826357FC: 4BE31625  bl 0x82466e20
	ctx.lr = 0x82635800;
	sub_82466E20(ctx, base);
	// 82635800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263580C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635810 size=112
    let mut pc: u32 = 0x82635810;
    'dispatch: loop {
        match pc {
            0x82635810 => {
    //   block [0x82635810..0x82635880)
	// 82635810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263581C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635820: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635824: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 82635828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263582C: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 82635830: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82635834: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82635838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263583C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635848: 386A8CA4  addi r3, r10, -0x735c
	ctx.r[3].s64 = ctx.r[10].s64 + -29532;
	// 8263584C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263585C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263586C: 4BE315B5  bl 0x82466e20
	ctx.lr = 0x82635870;
	sub_82466E20(ctx, base);
	// 82635870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263587C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635880 size=108
    let mut pc: u32 = 0x82635880;
    'dispatch: loop {
        match pc {
            0x82635880 => {
    //   block [0x82635880..0x826358EC)
	// 82635880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263588C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635894: 38EB0970  addi r7, r11, 0x970
	ctx.r[7].s64 = ctx.r[11].s64 + 2416;
	// 82635898: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263589C: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826358A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826358A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826358A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826358AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826358B0: 386A8CD4  addi r3, r10, -0x732c
	ctx.r[3].s64 = ctx.r[10].s64 + -29484;
	// 826358B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826358B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826358BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826358C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826358C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826358C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826358CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826358D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826358D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826358D8: 4BE31549  bl 0x82466e20
	ctx.lr = 0x826358DC;
	sub_82466E20(ctx, base);
	// 826358DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826358E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826358E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826358E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826358F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826358F0 size=108
    let mut pc: u32 = 0x826358F0;
    'dispatch: loop {
        match pc {
            0x826358F0 => {
    //   block [0x826358F0..0x8263595C)
	// 826358F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826358F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826358F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826358FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635904: 38EB09B8  addi r7, r11, 0x9b8
	ctx.r[7].s64 = ctx.r[11].s64 + 2488;
	// 82635908: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263590C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82635910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635918: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263591C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635920: 386A8D04  addi r3, r10, -0x72fc
	ctx.r[3].s64 = ctx.r[10].s64 + -29436;
	// 82635924: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263592C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263593C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635944: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635948: 4BE314D9  bl 0x82466e20
	ctx.lr = 0x8263594C;
	sub_82466E20(ctx, base);
	// 8263594C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635960 size=108
    let mut pc: u32 = 0x82635960;
    'dispatch: loop {
        match pc {
            0x82635960 => {
    //   block [0x82635960..0x826359CC)
	// 82635960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263596C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635974: 38EB09E8  addi r7, r11, 0x9e8
	ctx.r[7].s64 = ctx.r[11].s64 + 2536;
	// 82635978: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263597C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82635980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263598C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635990: 386A8D34  addi r3, r10, -0x72cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29388;
	// 82635994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263599C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826359A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826359A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826359A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826359AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826359B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826359B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826359B8: 4BE31469  bl 0x82466e20
	ctx.lr = 0x826359BC;
	sub_82466E20(ctx, base);
	// 826359BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826359C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826359C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826359C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826359D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826359D0 size=112
    let mut pc: u32 = 0x826359D0;
    'dispatch: loop {
        match pc {
            0x826359D0 => {
    //   block [0x826359D0..0x82635A40)
	// 826359D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826359D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826359D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826359DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826359E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826359E4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826359E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826359EC: 390B0A18  addi r8, r11, 0xa18
	ctx.r[8].s64 = ctx.r[11].s64 + 2584;
	// 826359F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826359F4: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826359F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826359FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635A08: 386A8D64  addi r3, r10, -0x729c
	ctx.r[3].s64 = ctx.r[10].s64 + -29340;
	// 82635A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635A2C: 4BE313F5  bl 0x82466e20
	ctx.lr = 0x82635A30;
	sub_82466E20(ctx, base);
	// 82635A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635A40 size=112
    let mut pc: u32 = 0x82635A40;
    'dispatch: loop {
        match pc {
            0x82635A40 => {
    //   block [0x82635A40..0x82635AB0)
	// 82635A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635A50: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635A54: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635A5C: 390B0A48  addi r8, r11, 0xa48
	ctx.r[8].s64 = ctx.r[11].s64 + 2632;
	// 82635A60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635A64: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82635A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635A6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635A78: 386A8D94  addi r3, r10, -0x726c
	ctx.r[3].s64 = ctx.r[10].s64 + -29292;
	// 82635A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635A9C: 4BE31385  bl 0x82466e20
	ctx.lr = 0x82635AA0;
	sub_82466E20(ctx, base);
	// 82635AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635AB0 size=112
    let mut pc: u32 = 0x82635AB0;
    'dispatch: loop {
        match pc {
            0x82635AB0 => {
    //   block [0x82635AB0..0x82635B20)
	// 82635AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635ABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635AC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635AC4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635ACC: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 82635AD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635AD4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82635AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635ADC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635AE8: 386A8DC4  addi r3, r10, -0x723c
	ctx.r[3].s64 = ctx.r[10].s64 + -29244;
	// 82635AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635B0C: 4BE31315  bl 0x82466e20
	ctx.lr = 0x82635B10;
	sub_82466E20(ctx, base);
	// 82635B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635B20 size=108
    let mut pc: u32 = 0x82635B20;
    'dispatch: loop {
        match pc {
            0x82635B20 => {
    //   block [0x82635B20..0x82635B8C)
	// 82635B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635B2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635B34: 38EB0A78  addi r7, r11, 0xa78
	ctx.r[7].s64 = ctx.r[11].s64 + 2680;
	// 82635B38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82635B3C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82635B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635B44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635B50: 386A8DF4  addi r3, r10, -0x720c
	ctx.r[3].s64 = ctx.r[10].s64 + -29196;
	// 82635B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635B78: 4BE312A9  bl 0x82466e20
	ctx.lr = 0x82635B7C;
	sub_82466E20(ctx, base);
	// 82635B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635B90 size=112
    let mut pc: u32 = 0x82635B90;
    'dispatch: loop {
        match pc {
            0x82635B90 => {
    //   block [0x82635B90..0x82635C00)
	// 82635B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635B9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635BA0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635BA4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635BAC: 390B0AA8  addi r8, r11, 0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + 2728;
	// 82635BB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82635BB4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82635BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635BBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635BC8: 386A8E24  addi r3, r10, -0x71dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29148;
	// 82635BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635BEC: 4BE31235  bl 0x82466e20
	ctx.lr = 0x82635BF0;
	sub_82466E20(ctx, base);
	// 82635BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635C00 size=108
    let mut pc: u32 = 0x82635C00;
    'dispatch: loop {
        match pc {
            0x82635C00 => {
    //   block [0x82635C00..0x82635C6C)
	// 82635C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635C0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635C14: 38EB0AC0  addi r7, r11, 0xac0
	ctx.r[7].s64 = ctx.r[11].s64 + 2752;
	// 82635C18: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82635C1C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82635C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635C30: 386A8E54  addi r3, r10, -0x71ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29100;
	// 82635C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635C58: 4BE311C9  bl 0x82466e20
	ctx.lr = 0x82635C5C;
	sub_82466E20(ctx, base);
	// 82635C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635C70 size=112
    let mut pc: u32 = 0x82635C70;
    'dispatch: loop {
        match pc {
            0x82635C70 => {
    //   block [0x82635C70..0x82635CE0)
	// 82635C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635C80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635C84: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635C8C: 390B0B98  addi r8, r11, 0xb98
	ctx.r[8].s64 = ctx.r[11].s64 + 2968;
	// 82635C90: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82635C94: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82635C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635C9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635CA8: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 82635CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635CCC: 4BE31155  bl 0x82466e20
	ctx.lr = 0x82635CD0;
	sub_82466E20(ctx, base);
	// 82635CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635CE0 size=108
    let mut pc: u32 = 0x82635CE0;
    'dispatch: loop {
        match pc {
            0x82635CE0 => {
    //   block [0x82635CE0..0x82635D4C)
	// 82635CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635CEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635CF4: 38EB0D48  addi r7, r11, 0xd48
	ctx.r[7].s64 = ctx.r[11].s64 + 3400;
	// 82635CF8: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82635CFC: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82635D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635D08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635D10: 386A8EB4  addi r3, r10, -0x714c
	ctx.r[3].s64 = ctx.r[10].s64 + -29004;
	// 82635D14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635D38: 4BE310E9  bl 0x82466e20
	ctx.lr = 0x82635D3C;
	sub_82466E20(ctx, base);
	// 82635D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635D50 size=112
    let mut pc: u32 = 0x82635D50;
    'dispatch: loop {
        match pc {
            0x82635D50 => {
    //   block [0x82635D50..0x82635DC0)
	// 82635D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635D60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635D64: 38AA79B4  addi r5, r10, 0x79b4
	ctx.r[5].s64 = ctx.r[10].s64 + 31156;
	// 82635D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635D6C: 390B0EE0  addi r8, r11, 0xee0
	ctx.r[8].s64 = ctx.r[11].s64 + 3808;
	// 82635D70: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 82635D74: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82635D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635D88: 386A8EE4  addi r3, r10, -0x711c
	ctx.r[3].s64 = ctx.r[10].s64 + -28956;
	// 82635D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635DAC: 4BE31075  bl 0x82466e20
	ctx.lr = 0x82635DB0;
	sub_82466E20(ctx, base);
	// 82635DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635DC0 size=100
    let mut pc: u32 = 0x82635DC0;
    'dispatch: loop {
        match pc {
            0x82635DC0 => {
    //   block [0x82635DC0..0x82635E24)
	// 82635DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635DD4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635DE0: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82635DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635DF4: 386A8F14  addi r3, r10, -0x70ec
	ctx.r[3].s64 = ctx.r[10].s64 + -28908;
	// 82635DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635DFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635E00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82635E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635E08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82635E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635E10: 4BE31011  bl 0x82466e20
	ctx.lr = 0x82635E14;
	sub_82466E20(ctx, base);
	// 82635E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635E28 size=112
    let mut pc: u32 = 0x82635E28;
    'dispatch: loop {
        match pc {
            0x82635E28 => {
    //   block [0x82635E28..0x82635E98)
	// 82635E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635E34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635E38: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635E3C: 38AA8F14  addi r5, r10, -0x70ec
	ctx.r[5].s64 = ctx.r[10].s64 + -28908;
	// 82635E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635E44: 390B1138  addi r8, r11, 0x1138
	ctx.r[8].s64 = ctx.r[11].s64 + 4408;
	// 82635E48: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82635E4C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82635E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635E54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635E60: 386A8F44  addi r3, r10, -0x70bc
	ctx.r[3].s64 = ctx.r[10].s64 + -28860;
	// 82635E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635E84: 4BE30F9D  bl 0x82466e20
	ctx.lr = 0x82635E88;
	sub_82466E20(ctx, base);
	// 82635E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635E98 size=100
    let mut pc: u32 = 0x82635E98;
    'dispatch: loop {
        match pc {
            0x82635E98 => {
    //   block [0x82635E98..0x82635EFC)
	// 82635E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635EAC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635EB8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82635EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635ECC: 386A8F74  addi r3, r10, -0x708c
	ctx.r[3].s64 = ctx.r[10].s64 + -28812;
	// 82635ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82635EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82635EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635EE8: 4BE30F39  bl 0x82466e20
	ctx.lr = 0x82635EEC;
	sub_82466E20(ctx, base);
	// 82635EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635F00 size=108
    let mut pc: u32 = 0x82635F00;
    'dispatch: loop {
        match pc {
            0x82635F00 => {
    //   block [0x82635F00..0x82635F6C)
	// 82635F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635F0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635F14: 38EB11B0  addi r7, r11, 0x11b0
	ctx.r[7].s64 = ctx.r[11].s64 + 4528;
	// 82635F18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82635F1C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82635F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635F24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82635F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635F30: 386A8FA4  addi r3, r10, -0x705c
	ctx.r[3].s64 = ctx.r[10].s64 + -28764;
	// 82635F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82635F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82635F58: 4BE30EC9  bl 0x82466e20
	ctx.lr = 0x82635F5C;
	sub_82466E20(ctx, base);
	// 82635F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635F70 size=112
    let mut pc: u32 = 0x82635F70;
    'dispatch: loop {
        match pc {
            0x82635F70 => {
    //   block [0x82635F70..0x82635FE0)
	// 82635F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635F7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635F80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82635F84: 38AA8F74  addi r5, r10, -0x708c
	ctx.r[5].s64 = ctx.r[10].s64 + -28812;
	// 82635F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635F8C: 390B11F8  addi r8, r11, 0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + 4600;
	// 82635F90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82635F94: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82635F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635F9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82635FA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82635FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82635FA8: 386A8FD4  addi r3, r10, -0x702c
	ctx.r[3].s64 = ctx.r[10].s64 + -28716;
	// 82635FAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82635FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82635FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82635FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82635FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82635FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82635FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82635FCC: 4BE30E55  bl 0x82466e20
	ctx.lr = 0x82635FD0;
	sub_82466E20(ctx, base);
	// 82635FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82635FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635FE0 size=100
    let mut pc: u32 = 0x82635FE0;
    'dispatch: loop {
        match pc {
            0x82635FE0 => {
    //   block [0x82635FE0..0x82636044)
	// 82635FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635FEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82635FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635FF4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82635FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82635FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636000: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 82636004: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263600C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636014: 386A9004  addi r3, r10, -0x6ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -28668;
	// 82636018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263601C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636020: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82636024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636028: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636030: 4BE30DF1  bl 0x82466e20
	ctx.lr = 0x82636034;
	sub_82466E20(ctx, base);
	// 82636034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636048 size=100
    let mut pc: u32 = 0x82636048;
    'dispatch: loop {
        match pc {
            0x82636048 => {
    //   block [0x82636048..0x826360AC)
	// 82636048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263605C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636068: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8263606C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263607C: 386A9034  addi r3, r10, -0x6fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -28620;
	// 82636080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263608C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82636094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636098: 4BE30D89  bl 0x82466e20
	ctx.lr = 0x8263609C;
	sub_82466E20(ctx, base);
	// 8263609C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826360A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826360A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826360A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826360B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826360B0 size=112
    let mut pc: u32 = 0x826360B0;
    'dispatch: loop {
        match pc {
            0x826360B0 => {
    //   block [0x826360B0..0x82636120)
	// 826360B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826360B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826360B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826360BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826360C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826360C4: 38AA9004  addi r5, r10, -0x6ffc
	ctx.r[5].s64 = ctx.r[10].s64 + -28668;
	// 826360C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826360CC: 390B1228  addi r8, r11, 0x1228
	ctx.r[8].s64 = ctx.r[11].s64 + 4648;
	// 826360D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826360D4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826360D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826360DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826360E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826360E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826360E8: 386A9064  addi r3, r10, -0x6f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -28572;
	// 826360EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826360F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826360F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826360F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826360FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263610C: 4BE30D15  bl 0x82466e20
	ctx.lr = 0x82636110;
	sub_82466E20(ctx, base);
	// 82636110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263611C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636120 size=112
    let mut pc: u32 = 0x82636120;
    'dispatch: loop {
        match pc {
            0x82636120 => {
    //   block [0x82636120..0x82636190)
	// 82636120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263612C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636130: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636134: 38AA9034  addi r5, r10, -0x6fcc
	ctx.r[5].s64 = ctx.r[10].s64 + -28620;
	// 82636138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263613C: 390B1288  addi r8, r11, 0x1288
	ctx.r[8].s64 = ctx.r[11].s64 + 4744;
	// 82636140: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82636144: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82636148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263614C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636158: 386A9094  addi r3, r10, -0x6f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -28524;
	// 8263615C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263616C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263617C: 4BE30CA5  bl 0x82466e20
	ctx.lr = 0x82636180;
	sub_82466E20(ctx, base);
	// 82636180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263618C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636190 size=100
    let mut pc: u32 = 0x82636190;
    'dispatch: loop {
        match pc {
            0x82636190 => {
    //   block [0x82636190..0x826361F4)
	// 82636190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263619C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826361A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826361A4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826361A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826361AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826361B0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826361B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826361B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826361BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826361C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826361C4: 386A90C4  addi r3, r10, -0x6f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -28476;
	// 826361C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826361CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826361D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826361D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826361D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826361DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826361E0: 4BE30C41  bl 0x82466e20
	ctx.lr = 0x826361E4;
	sub_82466E20(ctx, base);
	// 826361E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826361E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826361EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826361F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826361F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826361F8 size=112
    let mut pc: u32 = 0x826361F8;
    'dispatch: loop {
        match pc {
            0x826361F8 => {
    //   block [0x826361F8..0x82636268)
	// 826361F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826361FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636208: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263620C: 38AA90C4  addi r5, r10, -0x6f3c
	ctx.r[5].s64 = ctx.r[10].s64 + -28476;
	// 82636210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636214: 390B12E8  addi r8, r11, 0x12e8
	ctx.r[8].s64 = ctx.r[11].s64 + 4840;
	// 82636218: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8263621C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82636220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263622C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636230: 386A90F4  addi r3, r10, -0x6f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -28428;
	// 82636234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263623C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263624C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636254: 4BE30BCD  bl 0x82466e20
	ctx.lr = 0x82636258;
	sub_82466E20(ctx, base);
	// 82636258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263625C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636268 size=108
    let mut pc: u32 = 0x82636268;
    'dispatch: loop {
        match pc {
            0x82636268 => {
    //   block [0x82636268..0x826362D4)
	// 82636268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636274: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263627C: 38EB13D8  addi r7, r11, 0x13d8
	ctx.r[7].s64 = ctx.r[11].s64 + 5080;
	// 82636280: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82636284: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82636288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263628C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636298: 386A9124  addi r3, r10, -0x6edc
	ctx.r[3].s64 = ctx.r[10].s64 + -28380;
	// 8263629C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826362A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826362A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826362A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826362AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826362B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826362B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826362B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826362BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826362C0: 4BE30B61  bl 0x82466e20
	ctx.lr = 0x826362C4;
	sub_82466E20(ctx, base);
	// 826362C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826362C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826362CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826362D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826362D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826362D8 size=108
    let mut pc: u32 = 0x826362D8;
    'dispatch: loop {
        match pc {
            0x826362D8 => {
    //   block [0x826362D8..0x82636344)
	// 826362D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826362DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826362E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826362E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826362E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826362EC: 38EB14C8  addi r7, r11, 0x14c8
	ctx.r[7].s64 = ctx.r[11].s64 + 5320;
	// 826362F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826362F4: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826362F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826362FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636308: 386A9154  addi r3, r10, -0x6eac
	ctx.r[3].s64 = ctx.r[10].s64 + -28332;
	// 8263630C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263631C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263632C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636330: 4BE30AF1  bl 0x82466e20
	ctx.lr = 0x82636334;
	sub_82466E20(ctx, base);
	// 82636334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263633C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636348 size=108
    let mut pc: u32 = 0x82636348;
    'dispatch: loop {
        match pc {
            0x82636348 => {
    //   block [0x82636348..0x826363B4)
	// 82636348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636354: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263635C: 38EB1510  addi r7, r11, 0x1510
	ctx.r[7].s64 = ctx.r[11].s64 + 5392;
	// 82636360: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82636364: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82636368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263636C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636378: 386A9184  addi r3, r10, -0x6e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -28284;
	// 8263637C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263638C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263639C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826363A0: 4BE30A81  bl 0x82466e20
	ctx.lr = 0x826363A4;
	sub_82466E20(ctx, base);
	// 826363A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826363A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826363AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826363B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826363B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826363B8 size=108
    let mut pc: u32 = 0x826363B8;
    'dispatch: loop {
        match pc {
            0x826363B8 => {
    //   block [0x826363B8..0x82636424)
	// 826363B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826363BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826363C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826363C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826363C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826363CC: 38EB15E8  addi r7, r11, 0x15e8
	ctx.r[7].s64 = ctx.r[11].s64 + 5608;
	// 826363D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826363D4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826363D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826363DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826363E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826363E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826363E8: 386A91B4  addi r3, r10, -0x6e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -28236;
	// 826363EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826363F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826363F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826363F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826363FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263640C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636410: 4BE30A11  bl 0x82466e20
	ctx.lr = 0x82636414;
	sub_82466E20(ctx, base);
	// 82636414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263641C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636428 size=100
    let mut pc: u32 = 0x82636428;
    'dispatch: loop {
        match pc {
            0x82636428 => {
    //   block [0x82636428..0x8263648C)
	// 82636428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263643C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636448: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8263644C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263645C: 386A91E4  addi r3, r10, -0x6e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -28188;
	// 82636460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636468: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263646C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636470: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82636474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636478: 4BE309A9  bl 0x82466e20
	ctx.lr = 0x8263647C;
	sub_82466E20(ctx, base);
	// 8263647C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636490 size=112
    let mut pc: u32 = 0x82636490;
    'dispatch: loop {
        match pc {
            0x82636490 => {
    //   block [0x82636490..0x82636500)
	// 82636490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263649C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826364A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826364A4: 38AA91E4  addi r5, r10, -0x6e1c
	ctx.r[5].s64 = ctx.r[10].s64 + -28188;
	// 826364A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826364AC: 390B1600  addi r8, r11, 0x1600
	ctx.r[8].s64 = ctx.r[11].s64 + 5632;
	// 826364B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826364B4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826364B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826364BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826364C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826364C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826364C8: 386A9214  addi r3, r10, -0x6dec
	ctx.r[3].s64 = ctx.r[10].s64 + -28140;
	// 826364CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826364D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826364D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826364D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826364DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826364E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826364E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826364E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826364EC: 4BE30935  bl 0x82466e20
	ctx.lr = 0x826364F0;
	sub_82466E20(ctx, base);
	// 826364F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826364F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826364F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826364FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636500 size=108
    let mut pc: u32 = 0x82636500;
    'dispatch: loop {
        match pc {
            0x82636500 => {
    //   block [0x82636500..0x8263656C)
	// 82636500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263650C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636514: 38EB1648  addi r7, r11, 0x1648
	ctx.r[7].s64 = ctx.r[11].s64 + 5704;
	// 82636518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263651C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82636520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636524: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263652C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636530: 386A9244  addi r3, r10, -0x6dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28092;
	// 82636534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263653C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263654C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636558: 4BE308C9  bl 0x82466e20
	ctx.lr = 0x8263655C;
	sub_82466E20(ctx, base);
	// 8263655C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636570 size=112
    let mut pc: u32 = 0x82636570;
    'dispatch: loop {
        match pc {
            0x82636570 => {
    //   block [0x82636570..0x826365E0)
	// 82636570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263657C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636580: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636584: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263658C: 390B1690  addi r8, r11, 0x1690
	ctx.r[8].s64 = ctx.r[11].s64 + 5776;
	// 82636590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82636594: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82636598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263659C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826365A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826365A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826365A8: 386A9274  addi r3, r10, -0x6d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28044;
	// 826365AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826365B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826365B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826365B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826365BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826365C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826365C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826365C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826365CC: 4BE30855  bl 0x82466e20
	ctx.lr = 0x826365D0;
	sub_82466E20(ctx, base);
	// 826365D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826365D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826365D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826365DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826365E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826365E0 size=108
    let mut pc: u32 = 0x826365E0;
    'dispatch: loop {
        match pc {
            0x826365E0 => {
    //   block [0x826365E0..0x8263664C)
	// 826365E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826365E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826365E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826365EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826365F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826365F4: 38EB16A8  addi r7, r11, 0x16a8
	ctx.r[7].s64 = ctx.r[11].s64 + 5800;
	// 826365F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826365FC: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82636600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263660C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636610: 386A92A4  addi r3, r10, -0x6d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27996;
	// 82636614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263661C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263662C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636638: 4BE307E9  bl 0x82466e20
	ctx.lr = 0x8263663C;
	sub_82466E20(ctx, base);
	// 8263663C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636650 size=112
    let mut pc: u32 = 0x82636650;
    'dispatch: loop {
        match pc {
            0x82636650 => {
    //   block [0x82636650..0x826366C0)
	// 82636650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263665C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636660: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636664: 38AA9274  addi r5, r10, -0x6d8c
	ctx.r[5].s64 = ctx.r[10].s64 + -28044;
	// 82636668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263666C: 390B16F0  addi r8, r11, 0x16f0
	ctx.r[8].s64 = ctx.r[11].s64 + 5872;
	// 82636670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82636674: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82636678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263667C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636688: 386A92D4  addi r3, r10, -0x6d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27948;
	// 8263668C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826366A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826366A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826366A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826366AC: 4BE30775  bl 0x82466e20
	ctx.lr = 0x826366B0;
	sub_82466E20(ctx, base);
	// 826366B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826366B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826366B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826366BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826366C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826366C0 size=100
    let mut pc: u32 = 0x826366C0;
    'dispatch: loop {
        match pc {
            0x826366C0 => {
    //   block [0x826366C0..0x82636724)
	// 826366C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826366C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826366C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826366CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826366D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826366D4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826366D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826366DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826366E0: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826366E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826366E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826366EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826366F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826366F4: 386A9304  addi r3, r10, -0x6cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -27900;
	// 826366F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826366FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636700: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82636704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636708: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263670C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636710: 4BE30711  bl 0x82466e20
	ctx.lr = 0x82636714;
	sub_82466E20(ctx, base);
	// 82636714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263671C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636728 size=112
    let mut pc: u32 = 0x82636728;
    'dispatch: loop {
        match pc {
            0x82636728 => {
    //   block [0x82636728..0x82636798)
	// 82636728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636738: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263673C: 38AA9304  addi r5, r10, -0x6cfc
	ctx.r[5].s64 = ctx.r[10].s64 + -27900;
	// 82636740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636744: 390B1708  addi r8, r11, 0x1708
	ctx.r[8].s64 = ctx.r[11].s64 + 5896;
	// 82636748: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263674C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82636750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263675C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636760: 386A9334  addi r3, r10, -0x6ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -27852;
	// 82636764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263676C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263677C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636784: 4BE3069D  bl 0x82466e20
	ctx.lr = 0x82636788;
	sub_82466E20(ctx, base);
	// 82636788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263678C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636798 size=108
    let mut pc: u32 = 0x82636798;
    'dispatch: loop {
        match pc {
            0x82636798 => {
    //   block [0x82636798..0x82636804)
	// 82636798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263679C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826367A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826367A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826367A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826367AC: 38EB17B0  addi r7, r11, 0x17b0
	ctx.r[7].s64 = ctx.r[11].s64 + 6064;
	// 826367B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826367B4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826367B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826367BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826367C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826367C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826367C8: 386A9364  addi r3, r10, -0x6c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -27804;
	// 826367CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826367D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826367D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826367D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826367DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826367E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826367E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826367E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826367EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826367F0: 4BE30631  bl 0x82466e20
	ctx.lr = 0x826367F4;
	sub_82466E20(ctx, base);
	// 826367F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826367F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826367FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636808 size=112
    let mut pc: u32 = 0x82636808;
    'dispatch: loop {
        match pc {
            0x82636808 => {
    //   block [0x82636808..0x82636878)
	// 82636808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263680C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636818: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263681C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636824: 390B17E0  addi r8, r11, 0x17e0
	ctx.r[8].s64 = ctx.r[11].s64 + 6112;
	// 82636828: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263682C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82636830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636834: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263683C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636840: 386A9394  addi r3, r10, -0x6c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -27756;
	// 82636844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263684C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263685C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636864: 4BE305BD  bl 0x82466e20
	ctx.lr = 0x82636868;
	sub_82466E20(ctx, base);
	// 82636868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263686C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636878 size=112
    let mut pc: u32 = 0x82636878;
    'dispatch: loop {
        match pc {
            0x82636878 => {
    //   block [0x82636878..0x826368E8)
	// 82636878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263687C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636888: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263688C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636894: 390B1828  addi r8, r11, 0x1828
	ctx.r[8].s64 = ctx.r[11].s64 + 6184;
	// 82636898: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263689C: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826368A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826368A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826368A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826368AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826368B0: 386A93C4  addi r3, r10, -0x6c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -27708;
	// 826368B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826368B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826368BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826368C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826368C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826368C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826368CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826368D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826368D4: 4BE3054D  bl 0x82466e20
	ctx.lr = 0x826368D8;
	sub_82466E20(ctx, base);
	// 826368D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826368DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826368E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826368E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826368E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826368E8 size=100
    let mut pc: u32 = 0x826368E8;
    'dispatch: loop {
        match pc {
            0x826368E8 => {
    //   block [0x826368E8..0x8263694C)
	// 826368E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826368EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826368F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826368F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826368F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826368FC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636908: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8263690C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263691C: 386A93F4  addi r3, r10, -0x6c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27660;
	// 82636920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263692C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82636934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636938: 4BE304E9  bl 0x82466e20
	ctx.lr = 0x8263693C;
	sub_82466E20(ctx, base);
	// 8263693C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636950 size=112
    let mut pc: u32 = 0x82636950;
    'dispatch: loop {
        match pc {
            0x82636950 => {
    //   block [0x82636950..0x826369C0)
	// 82636950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263695C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636960: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636964: 38AA93F4  addi r5, r10, -0x6c0c
	ctx.r[5].s64 = ctx.r[10].s64 + -27660;
	// 82636968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263696C: 390B1870  addi r8, r11, 0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + 6256;
	// 82636970: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82636974: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82636978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263697C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636988: 386A9424  addi r3, r10, -0x6bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -27612;
	// 8263698C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263699C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826369A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826369A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826369A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826369AC: 4BE30475  bl 0x82466e20
	ctx.lr = 0x826369B0;
	sub_82466E20(ctx, base);
	// 826369B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826369B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826369B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826369BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826369C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826369C0 size=112
    let mut pc: u32 = 0x826369C0;
    'dispatch: loop {
        match pc {
            0x826369C0 => {
    //   block [0x826369C0..0x82636A30)
	// 826369C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826369C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826369C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826369CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826369D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826369D4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826369D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826369DC: 390B18B8  addi r8, r11, 0x18b8
	ctx.r[8].s64 = ctx.r[11].s64 + 6328;
	// 826369E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826369E4: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826369E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826369EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826369F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826369F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826369F8: 386A9454  addi r3, r10, -0x6bac
	ctx.r[3].s64 = ctx.r[10].s64 + -27564;
	// 826369FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636A1C: 4BE30405  bl 0x82466e20
	ctx.lr = 0x82636A20;
	sub_82466E20(ctx, base);
	// 82636A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636A30 size=112
    let mut pc: u32 = 0x82636A30;
    'dispatch: loop {
        match pc {
            0x82636A30 => {
    //   block [0x82636A30..0x82636AA0)
	// 82636A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636A3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82636A40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636A44: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82636A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636A4C: 390B18D0  addi r8, r11, 0x18d0
	ctx.r[8].s64 = ctx.r[11].s64 + 6352;
	// 82636A50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82636A54: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82636A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636A5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636A68: 386A9484  addi r3, r10, -0x6b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -27516;
	// 82636A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636A7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82636A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636A8C: 4BE30395  bl 0x82466e20
	ctx.lr = 0x82636A90;
	sub_82466E20(ctx, base);
	// 82636A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636AA0 size=112
    let mut pc: u32 = 0x82636AA0;
    'dispatch: loop {
        match pc {
            0x82636AA0 => {
    //   block [0x82636AA0..0x82636B10)
	// 82636AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636AAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636AB0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636AB4: 38AA9454  addi r5, r10, -0x6bac
	ctx.r[5].s64 = ctx.r[10].s64 + -27564;
	// 82636AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636ABC: 390B18E8  addi r8, r11, 0x18e8
	ctx.r[8].s64 = ctx.r[11].s64 + 6376;
	// 82636AC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82636AC4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82636AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636ACC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636AD8: 386A94B4  addi r3, r10, -0x6b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27468;
	// 82636ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636AFC: 4BE30325  bl 0x82466e20
	ctx.lr = 0x82636B00;
	sub_82466E20(ctx, base);
	// 82636B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636B10 size=72
    let mut pc: u32 = 0x82636B10;
    'dispatch: loop {
        match pc {
            0x82636B10 => {
    //   block [0x82636B10..0x82636B58)
	// 82636B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636B1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82636B20: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82636B24: 38CB46C8  addi r6, r11, 0x46c8
	ctx.r[6].s64 = ctx.r[11].s64 + 18120;
	// 82636B28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82636B2C: 388B5450  addi r4, r11, 0x5450
	ctx.r[4].s64 = ctx.r[11].s64 + 21584;
	// 82636B30: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82636B34: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 82636B38: 4BE44F51  bl 0x8247ba88
	ctx.lr = 0x82636B3C;
	sub_8247BA88(ctx, base);
	// 82636B3C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82636B40: 386BCD80  addi r3, r11, -0x3280
	ctx.r[3].s64 = ctx.r[11].s64 + -12928;
	// 82636B44: 4BEFBFF5  bl 0x82532b38
	ctx.lr = 0x82636B48;
	sub_82532B38(ctx, base);
	// 82636B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636B58 size=108
    let mut pc: u32 = 0x82636B58;
    'dispatch: loop {
        match pc {
            0x82636B58 => {
    //   block [0x82636B58..0x82636BC4)
	// 82636B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636B64: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636B6C: 38EB27C0  addi r7, r11, 0x27c0
	ctx.r[7].s64 = ctx.r[11].s64 + 10176;
	// 82636B70: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82636B74: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82636B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636B7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636B88: 386A94FC  addi r3, r10, -0x6b04
	ctx.r[3].s64 = ctx.r[10].s64 + -27396;
	// 82636B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636BB0: 4BE30271  bl 0x82466e20
	ctx.lr = 0x82636BB4;
	sub_82466E20(ctx, base);
	// 82636BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82636BC8 size=24
    let mut pc: u32 = 0x82636BC8;
    'dispatch: loop {
        match pc {
            0x82636BC8 => {
    //   block [0x82636BC8..0x82636BE0)
	// 82636BC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636BCC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82636BD0: 394AA8E8  addi r10, r10, -0x5718
	ctx.r[10].s64 = ctx.r[10].s64 + -22296;
	// 82636BD4: 816B2838  lwz r11, 0x2838(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10296 as u32) ) } as u64;
	// 82636BD8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82636BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636BE0 size=112
    let mut pc: u32 = 0x82636BE0;
    'dispatch: loop {
        match pc {
            0x82636BE0 => {
    //   block [0x82636BE0..0x82636C50)
	// 82636BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636BEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82636BF0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82636BF4: 392A5D34  addi r9, r10, 0x5d34
	ctx.r[9].s64 = ctx.r[10].s64 + 23860;
	// 82636BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636BFC: 390BA8E8  addi r8, r11, -0x5718
	ctx.r[8].s64 = ctx.r[11].s64 + -22296;
	// 82636C00: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82636C04: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82636C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636C0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636C18: 386A952C  addi r3, r10, -0x6ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -27348;
	// 82636C1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82636C20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82636C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636C3C: 4BE301E5  bl 0x82466e20
	ctx.lr = 0x82636C40;
	sub_82466E20(ctx, base);
	// 82636C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636C50 size=108
    let mut pc: u32 = 0x82636C50;
    'dispatch: loop {
        match pc {
            0x82636C50 => {
    //   block [0x82636C50..0x82636CBC)
	// 82636C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636C5C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636C64: 38EB283C  addi r7, r11, 0x283c
	ctx.r[7].s64 = ctx.r[11].s64 + 10300;
	// 82636C68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82636C6C: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82636C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636C78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636C80: 386A955C  addi r3, r10, -0x6aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -27300;
	// 82636C84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636CA8: 4BE30179  bl 0x82466e20
	ctx.lr = 0x82636CAC;
	sub_82466E20(ctx, base);
	// 82636CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636CC0 size=108
    let mut pc: u32 = 0x82636CC0;
    'dispatch: loop {
        match pc {
            0x82636CC0 => {
    //   block [0x82636CC0..0x82636D2C)
	// 82636CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636CCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636CD4: 38EB286C  addi r7, r11, 0x286c
	ctx.r[7].s64 = ctx.r[11].s64 + 10348;
	// 82636CD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82636CDC: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82636CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636CF0: 386A958C  addi r3, r10, -0x6a74
	ctx.r[3].s64 = ctx.r[10].s64 + -27252;
	// 82636CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636D18: 4BE30109  bl 0x82466e20
	ctx.lr = 0x82636D1C;
	sub_82466E20(ctx, base);
	// 82636D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82636D30 size=24
    let mut pc: u32 = 0x82636D30;
    'dispatch: loop {
        match pc {
            0x82636D30 => {
    //   block [0x82636D30..0x82636D48)
	// 82636D30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636D34: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82636D38: 394AA930  addi r10, r10, -0x56d0
	ctx.r[10].s64 = ctx.r[10].s64 + -22224;
	// 82636D3C: 816B289C  lwz r11, 0x289c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10396 as u32) ) } as u64;
	// 82636D40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82636D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636D48 size=116
    let mut pc: u32 = 0x82636D48;
    'dispatch: loop {
        match pc {
            0x82636D48 => {
    //   block [0x82636D48..0x82636DBC)
	// 82636D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636D54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82636D58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82636D5C: 390BA930  addi r8, r11, -0x56d0
	ctx.r[8].s64 = ctx.r[11].s64 + -22224;
	// 82636D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636D64: 392A5D68  addi r9, r10, 0x5d68
	ctx.r[9].s64 = ctx.r[10].s64 + 23912;
	// 82636D68: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636D6C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82636D70: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82636D74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636D8C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82636D90: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82636D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82636D98: 386B95BC  addi r3, r11, -0x6a44
	ctx.r[3].s64 = ctx.r[11].s64 + -27204;
	// 82636D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82636DA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636DA8: 4BE30079  bl 0x82466e20
	ctx.lr = 0x82636DAC;
	sub_82466E20(ctx, base);
	// 82636DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636DC0 size=108
    let mut pc: u32 = 0x82636DC0;
    'dispatch: loop {
        match pc {
            0x82636DC0 => {
    //   block [0x82636DC0..0x82636E2C)
	// 82636DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636DCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636DD4: 38EB28A0  addi r7, r11, 0x28a0
	ctx.r[7].s64 = ctx.r[11].s64 + 10400;
	// 82636DD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82636DDC: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82636DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636DF0: 386A95EC  addi r3, r10, -0x6a14
	ctx.r[3].s64 = ctx.r[10].s64 + -27156;
	// 82636DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636E18: 4BE30009  bl 0x82466e20
	ctx.lr = 0x82636E1C;
	sub_82466E20(ctx, base);
	// 82636E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636E30 size=112
    let mut pc: u32 = 0x82636E30;
    'dispatch: loop {
        match pc {
            0x82636E30 => {
    //   block [0x82636E30..0x82636EA0)
	// 82636E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636E3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636E40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636E44: 38AA95BC  addi r5, r10, -0x6a44
	ctx.r[5].s64 = ctx.r[10].s64 + -27204;
	// 82636E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636E4C: 390B2930  addi r8, r11, 0x2930
	ctx.r[8].s64 = ctx.r[11].s64 + 10544;
	// 82636E50: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82636E54: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82636E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636E68: 386A961C  addi r3, r10, -0x69e4
	ctx.r[3].s64 = ctx.r[10].s64 + -27108;
	// 82636E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636E8C: 4BE2FF95  bl 0x82466e20
	ctx.lr = 0x82636E90;
	sub_82466E20(ctx, base);
	// 82636E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636EA0 size=112
    let mut pc: u32 = 0x82636EA0;
    'dispatch: loop {
        match pc {
            0x82636EA0 => {
    //   block [0x82636EA0..0x82636F10)
	// 82636EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636EAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636EB0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636EB4: 38AA95BC  addi r5, r10, -0x6a44
	ctx.r[5].s64 = ctx.r[10].s64 + -27204;
	// 82636EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636EBC: 390B2A50  addi r8, r11, 0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + 10832;
	// 82636EC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82636EC4: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82636EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636ED8: 386A964C  addi r3, r10, -0x69b4
	ctx.r[3].s64 = ctx.r[10].s64 + -27060;
	// 82636EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636EFC: 4BE2FF25  bl 0x82466e20
	ctx.lr = 0x82636F00;
	sub_82466E20(ctx, base);
	// 82636F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636F10 size=108
    let mut pc: u32 = 0x82636F10;
    'dispatch: loop {
        match pc {
            0x82636F10 => {
    //   block [0x82636F10..0x82636F7C)
	// 82636F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636F1C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636F24: 38EB2A68  addi r7, r11, 0x2a68
	ctx.r[7].s64 = ctx.r[11].s64 + 10856;
	// 82636F28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82636F2C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82636F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82636F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636F40: 386A967C  addi r3, r10, -0x6984
	ctx.r[3].s64 = ctx.r[10].s64 + -27012;
	// 82636F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82636F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82636F68: 4BE2FEB9  bl 0x82466e20
	ctx.lr = 0x82636F6C;
	sub_82466E20(ctx, base);
	// 82636F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636F80 size=112
    let mut pc: u32 = 0x82636F80;
    'dispatch: loop {
        match pc {
            0x82636F80 => {
    //   block [0x82636F80..0x82636FF0)
	// 82636F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636F90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82636F94: 38AA95BC  addi r5, r10, -0x6a44
	ctx.r[5].s64 = ctx.r[10].s64 + -27204;
	// 82636F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82636F9C: 390B2AF8  addi r8, r11, 0x2af8
	ctx.r[8].s64 = ctx.r[11].s64 + 11000;
	// 82636FA0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82636FA4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82636FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636FAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82636FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82636FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82636FB8: 386A96AC  addi r3, r10, -0x6954
	ctx.r[3].s64 = ctx.r[10].s64 + -26964;
	// 82636FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82636FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82636FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82636FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82636FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82636FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82636FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82636FDC: 4BE2FE45  bl 0x82466e20
	ctx.lr = 0x82636FE0;
	sub_82466E20(ctx, base);
	// 82636FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82636FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636FF0 size=108
    let mut pc: u32 = 0x82636FF0;
    'dispatch: loop {
        match pc {
            0x82636FF0 => {
    //   block [0x82636FF0..0x8263705C)
	// 82636FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636FFC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637004: 38EB2BE8  addi r7, r11, 0x2be8
	ctx.r[7].s64 = ctx.r[11].s64 + 11240;
	// 82637008: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263700C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82637010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263701C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637020: 386A96DC  addi r3, r10, -0x6924
	ctx.r[3].s64 = ctx.r[10].s64 + -26916;
	// 82637024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263702C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263703C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637048: 4BE2FDD9  bl 0x82466e20
	ctx.lr = 0x8263704C;
	sub_82466E20(ctx, base);
	// 8263704C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637060 size=108
    let mut pc: u32 = 0x82637060;
    'dispatch: loop {
        match pc {
            0x82637060 => {
    //   block [0x82637060..0x826370CC)
	// 82637060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263706C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637074: 38EB2C00  addi r7, r11, 0x2c00
	ctx.r[7].s64 = ctx.r[11].s64 + 11264;
	// 82637078: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263707C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82637080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263708C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637090: 386A970C  addi r3, r10, -0x68f4
	ctx.r[3].s64 = ctx.r[10].s64 + -26868;
	// 82637094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263709C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826370A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826370A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826370A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826370AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826370B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826370B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826370B8: 4BE2FD69  bl 0x82466e20
	ctx.lr = 0x826370BC;
	sub_82466E20(ctx, base);
	// 826370BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826370C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826370C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826370C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826370D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826370D0 size=116
    let mut pc: u32 = 0x826370D0;
    'dispatch: loop {
        match pc {
            0x826370D0 => {
    //   block [0x826370D0..0x82637144)
	// 826370D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826370D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826370D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826370DC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826370E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826370E4: 390B2C64  addi r8, r11, 0x2c64
	ctx.r[8].s64 = ctx.r[11].s64 + 11364;
	// 826370E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826370EC: 392A5D94  addi r9, r10, 0x5d94
	ctx.r[9].s64 = ctx.r[10].s64 + 23956;
	// 826370F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826370F4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826370F8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826370FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637104: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263710C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637114: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82637118: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8263711C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82637120: 386B973C  addi r3, r11, -0x68c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26820;
	// 82637124: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82637128: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263712C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637130: 4BE2FCF1  bl 0x82466e20
	ctx.lr = 0x82637134;
	sub_82466E20(ctx, base);
	// 82637134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263713C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637148 size=108
    let mut pc: u32 = 0x82637148;
    'dispatch: loop {
        match pc {
            0x82637148 => {
    //   block [0x82637148..0x826371B4)
	// 82637148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263714C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637154: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263715C: 38EB2C80  addi r7, r11, 0x2c80
	ctx.r[7].s64 = ctx.r[11].s64 + 11392;
	// 82637160: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82637164: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82637168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263716C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82637174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637178: 386A976C  addi r3, r10, -0x6894
	ctx.r[3].s64 = ctx.r[10].s64 + -26772;
	// 8263717C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263719C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826371A0: 4BE2FC81  bl 0x82466e20
	ctx.lr = 0x826371A4;
	sub_82466E20(ctx, base);
	// 826371A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826371A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826371AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826371B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826371B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826371B8 size=108
    let mut pc: u32 = 0x826371B8;
    'dispatch: loop {
        match pc {
            0x826371B8 => {
    //   block [0x826371B8..0x82637224)
	// 826371B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826371BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826371C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826371C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826371C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826371CC: 38EB2CC8  addi r7, r11, 0x2cc8
	ctx.r[7].s64 = ctx.r[11].s64 + 11464;
	// 826371D0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826371D4: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826371D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826371DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826371E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826371E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826371E8: 386A979C  addi r3, r10, -0x6864
	ctx.r[3].s64 = ctx.r[10].s64 + -26724;
	// 826371EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826371F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826371F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826371F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826371FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263720C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637210: 4BE2FC11  bl 0x82466e20
	ctx.lr = 0x82637214;
	sub_82466E20(ctx, base);
	// 82637214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263721C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637228 size=108
    let mut pc: u32 = 0x82637228;
    'dispatch: loop {
        match pc {
            0x82637228 => {
    //   block [0x82637228..0x82637294)
	// 82637228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637234: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263723C: 38EB2D58  addi r7, r11, 0x2d58
	ctx.r[7].s64 = ctx.r[11].s64 + 11608;
	// 82637240: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82637244: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82637248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263724C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82637254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637258: 386A97CC  addi r3, r10, -0x6834
	ctx.r[3].s64 = ctx.r[10].s64 + -26676;
	// 8263725C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263726C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263727C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637280: 4BE2FBA1  bl 0x82466e20
	ctx.lr = 0x82637284;
	sub_82466E20(ctx, base);
	// 82637284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263728C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637298 size=100
    let mut pc: u32 = 0x82637298;
    'dispatch: loop {
        match pc {
            0x82637298 => {
    //   block [0x82637298..0x826372FC)
	// 82637298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263729C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826372A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826372A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826372A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826372AC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826372B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826372B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826372B8: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826372BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826372C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826372C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826372C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826372CC: 386A97FC  addi r3, r10, -0x6804
	ctx.r[3].s64 = ctx.r[10].s64 + -26628;
	// 826372D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826372D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826372D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826372DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826372E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826372E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826372E8: 4BE2FB39  bl 0x82466e20
	ctx.lr = 0x826372EC;
	sub_82466E20(ctx, base);
	// 826372EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826372F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826372F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826372F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637300 size=112
    let mut pc: u32 = 0x82637300;
    'dispatch: loop {
        match pc {
            0x82637300 => {
    //   block [0x82637300..0x82637370)
	// 82637300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263730C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637310: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637314: 38AA97FC  addi r5, r10, -0x6804
	ctx.r[5].s64 = ctx.r[10].s64 + -26628;
	// 82637318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263731C: 390B2DE8  addi r8, r11, 0x2de8
	ctx.r[8].s64 = ctx.r[11].s64 + 11752;
	// 82637320: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82637324: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82637328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263732C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637338: 386A982C  addi r3, r10, -0x67d4
	ctx.r[3].s64 = ctx.r[10].s64 + -26580;
	// 8263733C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263734C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263735C: 4BE2FAC5  bl 0x82466e20
	ctx.lr = 0x82637360;
	sub_82466E20(ctx, base);
	// 82637360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263736C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637370 size=108
    let mut pc: u32 = 0x82637370;
    'dispatch: loop {
        match pc {
            0x82637370 => {
    //   block [0x82637370..0x826373DC)
	// 82637370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263737C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637384: 38EB2E48  addi r7, r11, 0x2e48
	ctx.r[7].s64 = ctx.r[11].s64 + 11848;
	// 82637388: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263738C: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82637390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263739C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826373A0: 386A985C  addi r3, r10, -0x67a4
	ctx.r[3].s64 = ctx.r[10].s64 + -26532;
	// 826373A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826373A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826373AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826373B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826373B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826373B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826373BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826373C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826373C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826373C8: 4BE2FA59  bl 0x82466e20
	ctx.lr = 0x826373CC;
	sub_82466E20(ctx, base);
	// 826373CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826373D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826373D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826373D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826373E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826373E0 size=108
    let mut pc: u32 = 0x826373E0;
    'dispatch: loop {
        match pc {
            0x826373E0 => {
    //   block [0x826373E0..0x8263744C)
	// 826373E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826373E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826373E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826373EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826373F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826373F4: 38EB2E78  addi r7, r11, 0x2e78
	ctx.r[7].s64 = ctx.r[11].s64 + 11896;
	// 826373F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826373FC: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82637400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263740C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637410: 386A988C  addi r3, r10, -0x6774
	ctx.r[3].s64 = ctx.r[10].s64 + -26484;
	// 82637414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263741C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263742C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637438: 4BE2F9E9  bl 0x82466e20
	ctx.lr = 0x8263743C;
	sub_82466E20(ctx, base);
	// 8263743C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637450 size=108
    let mut pc: u32 = 0x82637450;
    'dispatch: loop {
        match pc {
            0x82637450 => {
    //   block [0x82637450..0x826374BC)
	// 82637450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263745C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637464: 38EB2ED8  addi r7, r11, 0x2ed8
	ctx.r[7].s64 = ctx.r[11].s64 + 11992;
	// 82637468: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263746C: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82637470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637478: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263747C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637480: 386A98BC  addi r3, r10, -0x6744
	ctx.r[3].s64 = ctx.r[10].s64 + -26436;
	// 82637484: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263748C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263749C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826374A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826374A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826374A8: 4BE2F979  bl 0x82466e20
	ctx.lr = 0x826374AC;
	sub_82466E20(ctx, base);
	// 826374AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826374B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826374B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826374B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826374C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826374C0 size=24
    let mut pc: u32 = 0x826374C0;
    'dispatch: loop {
        match pc {
            0x826374C0 => {
    //   block [0x826374C0..0x826374D8)
	// 826374C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826374C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826374C8: 394AA9A8  addi r10, r10, -0x5658
	ctx.r[10].s64 = ctx.r[10].s64 + -22104;
	// 826374CC: 816B2C7C  lwz r11, 0x2c7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11388 as u32) ) } as u64;
	// 826374D0: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 826374D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826374D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826374D8 size=116
    let mut pc: u32 = 0x826374D8;
    'dispatch: loop {
        match pc {
            0x826374D8 => {
    //   block [0x826374D8..0x8263754C)
	// 826374D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826374DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826374E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826374E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826374E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826374EC: 390BA9A8  addi r8, r11, -0x5658
	ctx.r[8].s64 = ctx.r[11].s64 + -22104;
	// 826374F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826374F4: 392A5DC8  addi r9, r10, 0x5dc8
	ctx.r[9].s64 = ctx.r[10].s64 + 24008;
	// 826374F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826374FC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82637500: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82637504: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263750C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263751C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82637520: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82637524: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82637528: 386B98EC  addi r3, r11, -0x6714
	ctx.r[3].s64 = ctx.r[11].s64 + -26388;
	// 8263752C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82637530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637538: 4BE2F8E9  bl 0x82466e20
	ctx.lr = 0x8263753C;
	sub_82466E20(ctx, base);
	// 8263753C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637550 size=112
    let mut pc: u32 = 0x82637550;
    'dispatch: loop {
        match pc {
            0x82637550 => {
    //   block [0x82637550..0x826375C0)
	// 82637550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263755C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637560: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637564: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82637568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263756C: 390B2F38  addi r8, r11, 0x2f38
	ctx.r[8].s64 = ctx.r[11].s64 + 12088;
	// 82637570: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82637574: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 82637578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263757C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637588: 386A991C  addi r3, r10, -0x66e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26340;
	// 8263758C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263759C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826375A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826375A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826375A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826375AC: 4BE2F875  bl 0x82466e20
	ctx.lr = 0x826375B0;
	sub_82466E20(ctx, base);
	// 826375B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826375B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826375B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826375BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826375C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826375C0 size=112
    let mut pc: u32 = 0x826375C0;
    'dispatch: loop {
        match pc {
            0x826375C0 => {
    //   block [0x826375C0..0x82637630)
	// 826375C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826375C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826375C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826375CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826375D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826375D4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826375D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826375DC: 390B2F80  addi r8, r11, 0x2f80
	ctx.r[8].s64 = ctx.r[11].s64 + 12160;
	// 826375E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826375E4: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 826375E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826375EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826375F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826375F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826375F8: 386A994C  addi r3, r10, -0x66b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26292;
	// 826375FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263760C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263761C: 4BE2F805  bl 0x82466e20
	ctx.lr = 0x82637620;
	sub_82466E20(ctx, base);
	// 82637620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263762C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637630 size=112
    let mut pc: u32 = 0x82637630;
    'dispatch: loop {
        match pc {
            0x82637630 => {
    //   block [0x82637630..0x826376A0)
	// 82637630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263763C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637640: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637644: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263764C: 390B2FC8  addi r8, r11, 0x2fc8
	ctx.r[8].s64 = ctx.r[11].s64 + 12232;
	// 82637650: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82637654: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82637658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263765C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637668: 386A997C  addi r3, r10, -0x6684
	ctx.r[3].s64 = ctx.r[10].s64 + -26244;
	// 8263766C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263767C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263768C: 4BE2F795  bl 0x82466e20
	ctx.lr = 0x82637690;
	sub_82466E20(ctx, base);
	// 82637690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263769C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826376A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826376A0 size=108
    let mut pc: u32 = 0x826376A0;
    'dispatch: loop {
        match pc {
            0x826376A0 => {
    //   block [0x826376A0..0x8263770C)
	// 826376A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826376A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826376A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826376AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826376B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826376B4: 38EB30B8  addi r7, r11, 0x30b8
	ctx.r[7].s64 = ctx.r[11].s64 + 12472;
	// 826376B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826376BC: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826376C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826376C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826376C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826376CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826376D0: 386A99AC  addi r3, r10, -0x6654
	ctx.r[3].s64 = ctx.r[10].s64 + -26196;
	// 826376D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826376D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826376DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826376E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826376E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826376E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826376EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826376F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826376F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826376F8: 4BE2F729  bl 0x82466e20
	ctx.lr = 0x826376FC;
	sub_82466E20(ctx, base);
	// 826376FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637710 size=108
    let mut pc: u32 = 0x82637710;
    'dispatch: loop {
        match pc {
            0x82637710 => {
    //   block [0x82637710..0x8263777C)
	// 82637710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263771C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637724: 38EB3118  addi r7, r11, 0x3118
	ctx.r[7].s64 = ctx.r[11].s64 + 12568;
	// 82637728: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263772C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 82637730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263773C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637740: 386A99DC  addi r3, r10, -0x6624
	ctx.r[3].s64 = ctx.r[10].s64 + -26148;
	// 82637744: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263774C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263775C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637768: 4BE2F6B9  bl 0x82466e20
	ctx.lr = 0x8263776C;
	sub_82466E20(ctx, base);
	// 8263776C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637780 size=112
    let mut pc: u32 = 0x82637780;
    'dispatch: loop {
        match pc {
            0x82637780 => {
    //   block [0x82637780..0x826377F0)
	// 82637780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263778C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82637790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637794: 392B5DFC  addi r9, r11, 0x5dfc
	ctx.r[9].s64 = ctx.r[11].s64 + 24060;
	// 82637798: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 8263779C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826377A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826377A4: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826377A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826377AC: 396B31C8  addi r11, r11, 0x31c8
	ctx.r[11].s64 = ctx.r[11].s64 + 12744;
	// 826377B0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826377B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826377B8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826377BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826377C0: 386A9A0C  addi r3, r10, -0x65f4
	ctx.r[3].s64 = ctx.r[10].s64 + -26100;
	// 826377C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826377C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826377CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826377D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826377D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826377D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826377DC: 4BE2F645  bl 0x82466e20
	ctx.lr = 0x826377E0;
	sub_82466E20(ctx, base);
	// 826377E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826377E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826377E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826377EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826377F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826377F0 size=112
    let mut pc: u32 = 0x826377F0;
    'dispatch: loop {
        match pc {
            0x826377F0 => {
    //   block [0x826377F0..0x82637860)
	// 826377F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826377F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826377F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826377FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637800: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637804: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82637808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263780C: 390B3318  addi r8, r11, 0x3318
	ctx.r[8].s64 = ctx.r[11].s64 + 13080;
	// 82637810: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82637814: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82637818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263781C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637828: 386A9A3C  addi r3, r10, -0x65c4
	ctx.r[3].s64 = ctx.r[10].s64 + -26052;
	// 8263782C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263783C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263784C: 4BE2F5D5  bl 0x82466e20
	ctx.lr = 0x82637850;
	sub_82466E20(ctx, base);
	// 82637850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263785C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637860 size=112
    let mut pc: u32 = 0x82637860;
    'dispatch: loop {
        match pc {
            0x82637860 => {
    //   block [0x82637860..0x826378D0)
	// 82637860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263786C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637870: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637874: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82637878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263787C: 390B33C0  addi r8, r11, 0x33c0
	ctx.r[8].s64 = ctx.r[11].s64 + 13248;
	// 82637880: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82637884: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 82637888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263788C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637898: 386A9A6C  addi r3, r10, -0x6594
	ctx.r[3].s64 = ctx.r[10].s64 + -26004;
	// 8263789C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826378A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826378A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826378A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826378AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826378B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826378B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826378B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826378BC: 4BE2F565  bl 0x82466e20
	ctx.lr = 0x826378C0;
	sub_82466E20(ctx, base);
	// 826378C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826378C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826378C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826378CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826378D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826378D0 size=112
    let mut pc: u32 = 0x826378D0;
    'dispatch: loop {
        match pc {
            0x826378D0 => {
    //   block [0x826378D0..0x82637940)
	// 826378D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826378D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826378D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826378DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826378E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826378E4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826378E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826378EC: 390B3450  addi r8, r11, 0x3450
	ctx.r[8].s64 = ctx.r[11].s64 + 13392;
	// 826378F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826378F4: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826378F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826378FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637908: 386A9A9C  addi r3, r10, -0x6564
	ctx.r[3].s64 = ctx.r[10].s64 + -25956;
	// 8263790C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263791C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263792C: 4BE2F4F5  bl 0x82466e20
	ctx.lr = 0x82637930;
	sub_82466E20(ctx, base);
	// 82637930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263793C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637940 size=108
    let mut pc: u32 = 0x82637940;
    'dispatch: loop {
        match pc {
            0x82637940 => {
    //   block [0x82637940..0x826379AC)
	// 82637940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263794C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637954: 38EB34C8  addi r7, r11, 0x34c8
	ctx.r[7].s64 = ctx.r[11].s64 + 13512;
	// 82637958: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263795C: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 82637960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263796C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637970: 386A9ACC  addi r3, r10, -0x6534
	ctx.r[3].s64 = ctx.r[10].s64 + -25908;
	// 82637974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263797C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263798C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637998: 4BE2F489  bl 0x82466e20
	ctx.lr = 0x8263799C;
	sub_82466E20(ctx, base);
	// 8263799C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826379A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826379A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826379A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826379B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826379B0 size=112
    let mut pc: u32 = 0x826379B0;
    'dispatch: loop {
        match pc {
            0x826379B0 => {
    //   block [0x826379B0..0x82637A20)
	// 826379B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826379B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826379B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826379BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826379C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826379C4: 392A5E58  addi r9, r10, 0x5e58
	ctx.r[9].s64 = ctx.r[10].s64 + 24152;
	// 826379C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826379CC: 390B3558  addi r8, r11, 0x3558
	ctx.r[8].s64 = ctx.r[11].s64 + 13656;
	// 826379D0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826379D4: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826379D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826379DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826379E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826379E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826379E8: 386A9AFC  addi r3, r10, -0x6504
	ctx.r[3].s64 = ctx.r[10].s64 + -25860;
	// 826379EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826379F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826379F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826379F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826379FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637A04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637A0C: 4BE2F415  bl 0x82466e20
	ctx.lr = 0x82637A10;
	sub_82466E20(ctx, base);
	// 82637A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637A20 size=100
    let mut pc: u32 = 0x82637A20;
    'dispatch: loop {
        match pc {
            0x82637A20 => {
    //   block [0x82637A20..0x82637A84)
	// 82637A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637A2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637A34: 38AAA2DC  addi r5, r10, -0x5d24
	ctx.r[5].s64 = ctx.r[10].s64 + -23844;
	// 82637A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637A40: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82637A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637A54: 386A9B2C  addi r3, r10, -0x64d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25812;
	// 82637A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637A5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637A60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82637A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637A68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82637A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637A70: 4BE2F3B1  bl 0x82466e20
	ctx.lr = 0x82637A74;
	sub_82466E20(ctx, base);
	// 82637A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637A88 size=108
    let mut pc: u32 = 0x82637A88;
    'dispatch: loop {
        match pc {
            0x82637A88 => {
    //   block [0x82637A88..0x82637AF4)
	// 82637A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637A94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637A9C: 38EB358C  addi r7, r11, 0x358c
	ctx.r[7].s64 = ctx.r[11].s64 + 13708;
	// 82637AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82637AA4: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 82637AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637AAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82637AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637AB8: 386A9B5C  addi r3, r10, -0x64a4
	ctx.r[3].s64 = ctx.r[10].s64 + -25764;
	// 82637ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637AE0: 4BE2F341  bl 0x82466e20
	ctx.lr = 0x82637AE4;
	sub_82466E20(ctx, base);
	// 82637AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637AF8 size=112
    let mut pc: u32 = 0x82637AF8;
    'dispatch: loop {
        match pc {
            0x82637AF8 => {
    //   block [0x82637AF8..0x82637B68)
	// 82637AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637B04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82637B08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637B0C: 392A5ED0  addi r9, r10, 0x5ed0
	ctx.r[9].s64 = ctx.r[10].s64 + 24272;
	// 82637B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637B14: 390B35C0  addi r8, r11, 0x35c0
	ctx.r[8].s64 = ctx.r[11].s64 + 13760;
	// 82637B18: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82637B1C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82637B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637B30: 386A9B8C  addi r3, r10, -0x6474
	ctx.r[3].s64 = ctx.r[10].s64 + -25716;
	// 82637B34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82637B38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82637B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637B54: 4BE2F2CD  bl 0x82466e20
	ctx.lr = 0x82637B58;
	sub_82466E20(ctx, base);
	// 82637B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637B68 size=112
    let mut pc: u32 = 0x82637B68;
    'dispatch: loop {
        match pc {
            0x82637B68 => {
    //   block [0x82637B68..0x82637BD8)
	// 82637B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637B74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637B78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637B7C: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637B84: 390B3638  addi r8, r11, 0x3638
	ctx.r[8].s64 = ctx.r[11].s64 + 13880;
	// 82637B88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82637B8C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 82637B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637BA0: 386A9BBC  addi r3, r10, -0x6444
	ctx.r[3].s64 = ctx.r[10].s64 + -25668;
	// 82637BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637BC4: 4BE2F25D  bl 0x82466e20
	ctx.lr = 0x82637BC8;
	sub_82466E20(ctx, base);
	// 82637BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637BD8 size=116
    let mut pc: u32 = 0x82637BD8;
    'dispatch: loop {
        match pc {
            0x82637BD8 => {
    //   block [0x82637BD8..0x82637C4C)
	// 82637BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637BE4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82637BE8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82637BEC: 390A3668  addi r8, r10, 0x3668
	ctx.r[8].s64 = ctx.r[10].s64 + 13928;
	// 82637BF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637BF4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82637BF8: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637BFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637C00: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82637C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637C0C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82637C10: 396B5EE4  addi r11, r11, 0x5ee4
	ctx.r[11].s64 = ctx.r[11].s64 + 24292;
	// 82637C14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637C18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637C1C: 386A9BEC  addi r3, r10, -0x6414
	ctx.r[3].s64 = ctx.r[10].s64 + -25620;
	// 82637C20: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82637C24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637C28: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82637C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637C38: 4BE2F1E9  bl 0x82466e20
	ctx.lr = 0x82637C3C;
	sub_82466E20(ctx, base);
	// 82637C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637C50 size=100
    let mut pc: u32 = 0x82637C50;
    'dispatch: loop {
        match pc {
            0x82637C50 => {
    //   block [0x82637C50..0x82637CB4)
	// 82637C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637C5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637C64: 38AA9BEC  addi r5, r10, -0x6414
	ctx.r[5].s64 = ctx.r[10].s64 + -25620;
	// 82637C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637C70: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82637C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637C84: 386A9C1C  addi r3, r10, -0x63e4
	ctx.r[3].s64 = ctx.r[10].s64 + -25572;
	// 82637C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637C8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637C90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82637C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637C98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82637C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637CA0: 4BE2F181  bl 0x82466e20
	ctx.lr = 0x82637CA4;
	sub_82466E20(ctx, base);
	// 82637CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82637CB8 size=24
    let mut pc: u32 = 0x82637CB8;
    'dispatch: loop {
        match pc {
            0x82637CB8 => {
    //   block [0x82637CB8..0x82637CD0)
	// 82637CB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637CBC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82637CC0: 394AAAE0  addi r10, r10, -0x5520
	ctx.r[10].s64 = ctx.r[10].s64 + -21792;
	// 82637CC4: 816B35BC  lwz r11, 0x35bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13756 as u32) ) } as u64;
	// 82637CC8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82637CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637CD0 size=116
    let mut pc: u32 = 0x82637CD0;
    'dispatch: loop {
        match pc {
            0x82637CD0 => {
    //   block [0x82637CD0..0x82637D44)
	// 82637CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637CDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82637CE0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637CE4: 392B5F20  addi r9, r11, 0x5f20
	ctx.r[9].s64 = ctx.r[11].s64 + 24352;
	// 82637CE8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82637CEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637CF0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82637CF4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82637CF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82637CFC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82637D00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637D04: 396BAAE0  addi r11, r11, -0x5520
	ctx.r[11].s64 = ctx.r[11].s64 + -21792;
	// 82637D08: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82637D0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637D10: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82637D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637D18: 386A9C4C  addi r3, r10, -0x63b4
	ctx.r[3].s64 = ctx.r[10].s64 + -25524;
	// 82637D1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82637D20: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82637D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637D28: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82637D2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82637D30: 4BE2F0F1  bl 0x82466e20
	ctx.lr = 0x82637D34;
	sub_82466E20(ctx, base);
	// 82637D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637D48 size=116
    let mut pc: u32 = 0x82637D48;
    'dispatch: loop {
        match pc {
            0x82637D48 => {
    //   block [0x82637D48..0x82637DBC)
	// 82637D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637D54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82637D58: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637D5C: 392B5F64  addi r9, r11, 0x5f64
	ctx.r[9].s64 = ctx.r[11].s64 + 24420;
	// 82637D60: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637D64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637D68: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82637D6C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82637D70: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637D74: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82637D78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637D7C: 396B3718  addi r11, r11, 0x3718
	ctx.r[11].s64 = ctx.r[11].s64 + 14104;
	// 82637D80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82637D84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637D88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82637D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637D90: 386A9C7C  addi r3, r10, -0x6384
	ctx.r[3].s64 = ctx.r[10].s64 + -25476;
	// 82637D94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82637D98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82637D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637DA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82637DA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82637DA8: 4BE2F079  bl 0x82466e20
	ctx.lr = 0x82637DAC;
	sub_82466E20(ctx, base);
	// 82637DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637DC0 size=108
    let mut pc: u32 = 0x82637DC0;
    'dispatch: loop {
        match pc {
            0x82637DC0 => {
    //   block [0x82637DC0..0x82637E2C)
	// 82637DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637DCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637DD4: 38EB37C0  addi r7, r11, 0x37c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14272;
	// 82637DD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82637DDC: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82637DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82637DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637DF0: 386A9CAC  addi r3, r10, -0x6354
	ctx.r[3].s64 = ctx.r[10].s64 + -25428;
	// 82637DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82637DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82637E18: 4BE2F009  bl 0x82466e20
	ctx.lr = 0x82637E1C;
	sub_82466E20(ctx, base);
	// 82637E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82637E30 size=24
    let mut pc: u32 = 0x82637E30;
    'dispatch: loop {
        match pc {
            0x82637E30 => {
    //   block [0x82637E30..0x82637E48)
	// 82637E30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637E34: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82637E38: 394AAB58  addi r10, r10, -0x54a8
	ctx.r[10].s64 = ctx.r[10].s64 + -21672;
	// 82637E3C: 816B3714  lwz r11, 0x3714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14100 as u32) ) } as u64;
	// 82637E40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82637E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637E48 size=116
    let mut pc: u32 = 0x82637E48;
    'dispatch: loop {
        match pc {
            0x82637E48 => {
    //   block [0x82637E48..0x82637EBC)
	// 82637E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637E54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82637E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82637E5C: 390BAB58  addi r8, r11, -0x54a8
	ctx.r[8].s64 = ctx.r[11].s64 + -21672;
	// 82637E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637E64: 392A5FD0  addi r9, r10, 0x5fd0
	ctx.r[9].s64 = ctx.r[10].s64 + 24528;
	// 82637E68: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637E6C: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 82637E70: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637E8C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82637E90: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 82637E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82637E98: 386B9CDC  addi r3, r11, -0x6324
	ctx.r[3].s64 = ctx.r[11].s64 + -25380;
	// 82637E9C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82637EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637EA8: 4BE2EF79  bl 0x82466e20
	ctx.lr = 0x82637EAC;
	sub_82466E20(ctx, base);
	// 82637EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637EC0 size=112
    let mut pc: u32 = 0x82637EC0;
    'dispatch: loop {
        match pc {
            0x82637EC0 => {
    //   block [0x82637EC0..0x82637F30)
	// 82637EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637ED0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637ED4: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637EDC: 390B3824  addi r8, r11, 0x3824
	ctx.r[8].s64 = ctx.r[11].s64 + 14372;
	// 82637EE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82637EE4: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82637EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637EF8: 386A9D0C  addi r3, r10, -0x62f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25332;
	// 82637EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82637F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82637F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637F1C: 4BE2EF05  bl 0x82466e20
	ctx.lr = 0x82637F20;
	sub_82466E20(ctx, base);
	// 82637F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82637F30 size=24
    let mut pc: u32 = 0x82637F30;
    'dispatch: loop {
        match pc {
            0x82637F30 => {
    //   block [0x82637F30..0x82637F48)
	// 82637F30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637F34: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82637F38: 394AACF0  addi r10, r10, -0x5310
	ctx.r[10].s64 = ctx.r[10].s64 + -21264;
	// 82637F3C: 816B3854  lwz r11, 0x3854(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14420 as u32) ) } as u64;
	// 82637F40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82637F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637F48 size=116
    let mut pc: u32 = 0x82637F48;
    'dispatch: loop {
        match pc {
            0x82637F48 => {
    //   block [0x82637F48..0x82637FBC)
	// 82637F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637F54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82637F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82637F5C: 390BACF0  addi r8, r11, -0x5310
	ctx.r[8].s64 = ctx.r[11].s64 + -21264;
	// 82637F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637F64: 392A6008  addi r9, r10, 0x6008
	ctx.r[9].s64 = ctx.r[10].s64 + 24584;
	// 82637F68: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637F6C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82637F70: 38AA9C7C  addi r5, r10, -0x6384
	ctx.r[5].s64 = ctx.r[10].s64 + -25476;
	// 82637F74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82637F7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82637F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82637F8C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82637F90: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 82637F94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82637F98: 386B9D3C  addi r3, r11, -0x62c4
	ctx.r[3].s64 = ctx.r[11].s64 + -25284;
	// 82637F9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82637FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82637FA8: 4BE2EE79  bl 0x82466e20
	ctx.lr = 0x82637FAC;
	sub_82466E20(ctx, base);
	// 82637FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82637FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82637FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637FC0 size=112
    let mut pc: u32 = 0x82637FC0;
    'dispatch: loop {
        match pc {
            0x82637FC0 => {
    //   block [0x82637FC0..0x82638030)
	// 82637FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637FCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637FD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82637FD4: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82637FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82637FDC: 390B3858  addi r8, r11, 0x3858
	ctx.r[8].s64 = ctx.r[11].s64 + 14424;
	// 82637FE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82637FE4: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82637FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637FEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82637FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82637FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82637FF8: 386A9D6C  addi r3, r10, -0x6294
	ctx.r[3].s64 = ctx.r[10].s64 + -25236;
	// 82637FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263800C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263801C: 4BE2EE05  bl 0x82466e20
	ctx.lr = 0x82638020;
	sub_82466E20(ctx, base);
	// 82638020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638030 size=100
    let mut pc: u32 = 0x82638030;
    'dispatch: loop {
        match pc {
            0x82638030 => {
    //   block [0x82638030..0x82638094)
	// 82638030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263803C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638044: 38AAA2DC  addi r5, r10, -0x5d24
	ctx.r[5].s64 = ctx.r[10].s64 + -23844;
	// 82638048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263804C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638050: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 82638054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263805C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638064: 386A9D9C  addi r3, r10, -0x6264
	ctx.r[3].s64 = ctx.r[10].s64 + -25188;
	// 82638068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263806C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638070: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82638074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638078: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263807C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638080: 4BE2EDA1  bl 0x82466e20
	ctx.lr = 0x82638084;
	sub_82466E20(ctx, base);
	// 82638084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263808C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638098 size=108
    let mut pc: u32 = 0x82638098;
    'dispatch: loop {
        match pc {
            0x82638098 => {
    //   block [0x82638098..0x82638104)
	// 82638098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263809C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826380A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826380A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826380A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826380AC: 38EB3870  addi r7, r11, 0x3870
	ctx.r[7].s64 = ctx.r[11].s64 + 14448;
	// 826380B0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826380B4: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826380B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826380BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826380C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826380C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826380C8: 386A9DCC  addi r3, r10, -0x6234
	ctx.r[3].s64 = ctx.r[10].s64 + -25140;
	// 826380CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826380D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826380D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826380D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826380DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826380E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826380E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826380E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826380EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826380F0: 4BE2ED31  bl 0x82466e20
	ctx.lr = 0x826380F4;
	sub_82466E20(ctx, base);
	// 826380F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826380F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826380FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638108 size=112
    let mut pc: u32 = 0x82638108;
    'dispatch: loop {
        match pc {
            0x82638108 => {
    //   block [0x82638108..0x82638178)
	// 82638108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263810C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638114: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638118: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263811C: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638124: 390B3948  addi r8, r11, 0x3948
	ctx.r[8].s64 = ctx.r[11].s64 + 14664;
	// 82638128: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263812C: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82638130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638134: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263813C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638140: 386A9DFC  addi r3, r10, -0x6204
	ctx.r[3].s64 = ctx.r[10].s64 + -25092;
	// 82638144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263814C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263815C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638164: 4BE2ECBD  bl 0x82466e20
	ctx.lr = 0x82638168;
	sub_82466E20(ctx, base);
	// 82638168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263816C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638178 size=108
    let mut pc: u32 = 0x82638178;
    'dispatch: loop {
        match pc {
            0x82638178 => {
    //   block [0x82638178..0x826381E4)
	// 82638178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263817C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638184: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263818C: 38EB3978  addi r7, r11, 0x3978
	ctx.r[7].s64 = ctx.r[11].s64 + 14712;
	// 82638190: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82638194: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 82638198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263819C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826381A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826381A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826381A8: 386A9E2C  addi r3, r10, -0x61d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25044;
	// 826381AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826381B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826381B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826381B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826381BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826381C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826381C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826381C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826381CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826381D0: 4BE2EC51  bl 0x82466e20
	ctx.lr = 0x826381D4;
	sub_82466E20(ctx, base);
	// 826381D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826381D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826381DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826381E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826381E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826381E8 size=112
    let mut pc: u32 = 0x826381E8;
    'dispatch: loop {
        match pc {
            0x826381E8 => {
    //   block [0x826381E8..0x82638258)
	// 826381E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826381EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826381F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826381F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826381F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826381FC: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638204: 390B39A8  addi r8, r11, 0x39a8
	ctx.r[8].s64 = ctx.r[11].s64 + 14760;
	// 82638208: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263820C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82638210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638214: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263821C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638220: 386A9E5C  addi r3, r10, -0x61a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24996;
	// 82638224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263822C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263823C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638244: 4BE2EBDD  bl 0x82466e20
	ctx.lr = 0x82638248;
	sub_82466E20(ctx, base);
	// 82638248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263824C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638258 size=112
    let mut pc: u32 = 0x82638258;
    'dispatch: loop {
        match pc {
            0x82638258 => {
    //   block [0x82638258..0x826382C8)
	// 82638258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263825C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638264: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82638268: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8263826C: 38EA39C0  addi r7, r10, 0x39c0
	ctx.r[7].s64 = ctx.r[10].s64 + 14784;
	// 82638270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638274: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82638278: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8263827C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638280: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82638284: 396B601C  addi r11, r11, 0x601c
	ctx.r[11].s64 = ctx.r[11].s64 + 24604;
	// 82638288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263828C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638294: 386A9E8C  addi r3, r10, -0x6174
	ctx.r[3].s64 = ctx.r[10].s64 + -24948;
	// 82638298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263829C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826382A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826382A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826382A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826382AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826382B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826382B4: 4BE2EB6D  bl 0x82466e20
	ctx.lr = 0x826382B8;
	sub_82466E20(ctx, base);
	// 826382B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826382BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826382C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826382C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826382C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826382C8 size=108
    let mut pc: u32 = 0x826382C8;
    'dispatch: loop {
        match pc {
            0x826382C8 => {
    //   block [0x826382C8..0x82638334)
	// 826382C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826382CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826382D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826382D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826382D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826382DC: 38EB3A98  addi r7, r11, 0x3a98
	ctx.r[7].s64 = ctx.r[11].s64 + 15000;
	// 826382E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826382E4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826382E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826382EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826382F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826382F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826382F8: 386A9EBC  addi r3, r10, -0x6144
	ctx.r[3].s64 = ctx.r[10].s64 + -24900;
	// 826382FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82638300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263830C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263831C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82638320: 4BE2EB01  bl 0x82466e20
	ctx.lr = 0x82638324;
	sub_82466E20(ctx, base);
	// 82638324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263832C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638338 size=108
    let mut pc: u32 = 0x82638338;
    'dispatch: loop {
        match pc {
            0x82638338 => {
    //   block [0x82638338..0x826383A4)
	// 82638338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263833C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638344: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263834C: 38EB3AB0  addi r7, r11, 0x3ab0
	ctx.r[7].s64 = ctx.r[11].s64 + 15024;
	// 82638350: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82638354: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82638358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263835C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82638364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638368: 386A9EEC  addi r3, r10, -0x6114
	ctx.r[3].s64 = ctx.r[10].s64 + -24852;
	// 8263836C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82638370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263837C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263838C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82638390: 4BE2EA91  bl 0x82466e20
	ctx.lr = 0x82638394;
	sub_82466E20(ctx, base);
	// 82638394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263839C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826383A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826383A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826383A8 size=108
    let mut pc: u32 = 0x826383A8;
    'dispatch: loop {
        match pc {
            0x826383A8 => {
    //   block [0x826383A8..0x82638414)
	// 826383A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826383AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826383B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826383B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826383B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826383BC: 38EB3BB8  addi r7, r11, 0x3bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 15288;
	// 826383C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826383C4: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826383C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826383CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826383D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826383D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826383D8: 386A9F1C  addi r3, r10, -0x60e4
	ctx.r[3].s64 = ctx.r[10].s64 + -24804;
	// 826383DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826383E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826383E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826383E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826383EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826383F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826383F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826383F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826383FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82638400: 4BE2EA21  bl 0x82466e20
	ctx.lr = 0x82638404;
	sub_82466E20(ctx, base);
	// 82638404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263840C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638418 size=112
    let mut pc: u32 = 0x82638418;
    'dispatch: loop {
        match pc {
            0x82638418 => {
    //   block [0x82638418..0x82638488)
	// 82638418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263841C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638428: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263842C: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638434: 390B3C18  addi r8, r11, 0x3c18
	ctx.r[8].s64 = ctx.r[11].s64 + 15384;
	// 82638438: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8263843C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82638440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263844C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638450: 386A9F4C  addi r3, r10, -0x60b4
	ctx.r[3].s64 = ctx.r[10].s64 + -24756;
	// 82638454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263845C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263846C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638474: 4BE2E9AD  bl 0x82466e20
	ctx.lr = 0x82638478;
	sub_82466E20(ctx, base);
	// 82638478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263847C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638488 size=112
    let mut pc: u32 = 0x82638488;
    'dispatch: loop {
        match pc {
            0x82638488 => {
    //   block [0x82638488..0x826384F8)
	// 82638488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263848C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638498: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263849C: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 826384A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826384A4: 390B3D38  addi r8, r11, 0x3d38
	ctx.r[8].s64 = ctx.r[11].s64 + 15672;
	// 826384A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826384AC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826384B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826384B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826384B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826384BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826384C0: 386A9F7C  addi r3, r10, -0x6084
	ctx.r[3].s64 = ctx.r[10].s64 + -24708;
	// 826384C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826384C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826384CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826384D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826384D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826384D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826384DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826384E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826384E4: 4BE2E93D  bl 0x82466e20
	ctx.lr = 0x826384E8;
	sub_82466E20(ctx, base);
	// 826384E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826384EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826384F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826384F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826384F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826384F8 size=116
    let mut pc: u32 = 0x826384F8;
    'dispatch: loop {
        match pc {
            0x826384F8 => {
    //   block [0x826384F8..0x8263856C)
	// 826384F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826384FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638504: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82638508: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8263850C: 390A3D50  addi r8, r10, 0x3d50
	ctx.r[8].s64 = ctx.r[10].s64 + 15696;
	// 82638510: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638514: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82638518: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 8263851C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638520: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82638524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263852C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82638530: 396B604C  addi r11, r11, 0x604c
	ctx.r[11].s64 = ctx.r[11].s64 + 24652;
	// 82638534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638538: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263853C: 386A9FAC  addi r3, r10, -0x6054
	ctx.r[3].s64 = ctx.r[10].s64 + -24660;
	// 82638540: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82638544: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638548: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263854C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638558: 4BE2E8C9  bl 0x82466e20
	ctx.lr = 0x8263855C;
	sub_82466E20(ctx, base);
	// 8263855C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638570 size=108
    let mut pc: u32 = 0x82638570;
    'dispatch: loop {
        match pc {
            0x82638570 => {
    //   block [0x82638570..0x826385DC)
	// 82638570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263857C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638584: 38EB3DB0  addi r7, r11, 0x3db0
	ctx.r[7].s64 = ctx.r[11].s64 + 15792;
	// 82638588: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263858C: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 82638590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638594: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263859C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826385A0: 386A9FDC  addi r3, r10, -0x6024
	ctx.r[3].s64 = ctx.r[10].s64 + -24612;
	// 826385A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826385A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826385AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826385B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826385B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826385B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826385BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826385C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826385C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826385C8: 4BE2E859  bl 0x82466e20
	ctx.lr = 0x826385CC;
	sub_82466E20(ctx, base);
	// 826385CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826385D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826385D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826385D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826385E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826385E0 size=108
    let mut pc: u32 = 0x826385E0;
    'dispatch: loop {
        match pc {
            0x826385E0 => {
    //   block [0x826385E0..0x8263864C)
	// 826385E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826385E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826385E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826385EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826385F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826385F4: 38EB3DF8  addi r7, r11, 0x3df8
	ctx.r[7].s64 = ctx.r[11].s64 + 15864;
	// 826385F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826385FC: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 82638600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263860C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638610: 386AA00C  addi r3, r10, -0x5ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -24564;
	// 82638614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82638618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263861C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263862C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82638638: 4BE2E7E9  bl 0x82466e20
	ctx.lr = 0x8263863C;
	sub_82466E20(ctx, base);
	// 8263863C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638650 size=112
    let mut pc: u32 = 0x82638650;
    'dispatch: loop {
        match pc {
            0x82638650 => {
    //   block [0x82638650..0x826386C0)
	// 82638650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263865C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638660: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638664: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263866C: 390B3E40  addi r8, r11, 0x3e40
	ctx.r[8].s64 = ctx.r[11].s64 + 15936;
	// 82638670: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82638674: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82638678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263867C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638688: 386AA03C  addi r3, r10, -0x5fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -24516;
	// 8263868C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263869C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826386A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826386A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826386A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826386AC: 4BE2E775  bl 0x82466e20
	ctx.lr = 0x826386B0;
	sub_82466E20(ctx, base);
	// 826386B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826386B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826386B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826386BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826386C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826386C0 size=112
    let mut pc: u32 = 0x826386C0;
    'dispatch: loop {
        match pc {
            0x826386C0 => {
    //   block [0x826386C0..0x82638730)
	// 826386C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826386C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826386C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826386CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826386D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826386D4: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 826386D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826386DC: 390B3F48  addi r8, r11, 0x3f48
	ctx.r[8].s64 = ctx.r[11].s64 + 16200;
	// 826386E0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826386E4: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826386E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826386EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826386F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826386F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826386F8: 386AA06C  addi r3, r10, -0x5f94
	ctx.r[3].s64 = ctx.r[10].s64 + -24468;
	// 826386FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263870C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263871C: 4BE2E705  bl 0x82466e20
	ctx.lr = 0x82638720;
	sub_82466E20(ctx, base);
	// 82638720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263872C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638730 size=112
    let mut pc: u32 = 0x82638730;
    'dispatch: loop {
        match pc {
            0x82638730 => {
    //   block [0x82638730..0x826387A0)
	// 82638730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263873C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638740: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638744: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263874C: 390B4050  addi r8, r11, 0x4050
	ctx.r[8].s64 = ctx.r[11].s64 + 16464;
	// 82638750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82638754: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82638758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263875C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638768: 386AA09C  addi r3, r10, -0x5f64
	ctx.r[3].s64 = ctx.r[10].s64 + -24420;
	// 8263876C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263877C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263878C: 4BE2E695  bl 0x82466e20
	ctx.lr = 0x82638790;
	sub_82466E20(ctx, base);
	// 82638790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263879C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826387A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826387A0 size=112
    let mut pc: u32 = 0x826387A0;
    'dispatch: loop {
        match pc {
            0x826387A0 => {
    //   block [0x826387A0..0x82638810)
	// 826387A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826387A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826387A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826387AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826387B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826387B4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826387B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826387BC: 390B4068  addi r8, r11, 0x4068
	ctx.r[8].s64 = ctx.r[11].s64 + 16488;
	// 826387C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826387C4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826387C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826387CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826387D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826387D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826387D8: 386AA0CC  addi r3, r10, -0x5f34
	ctx.r[3].s64 = ctx.r[10].s64 + -24372;
	// 826387DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826387E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826387E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826387E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826387EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826387F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826387F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826387F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826387FC: 4BE2E625  bl 0x82466e20
	ctx.lr = 0x82638800;
	sub_82466E20(ctx, base);
	// 82638800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263880C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638810 size=108
    let mut pc: u32 = 0x82638810;
    'dispatch: loop {
        match pc {
            0x82638810 => {
    //   block [0x82638810..0x8263887C)
	// 82638810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263881C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638824: 38EB4098  addi r7, r11, 0x4098
	ctx.r[7].s64 = ctx.r[11].s64 + 16536;
	// 82638828: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263882C: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82638830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638834: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263883C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638840: 386AA0FC  addi r3, r10, -0x5f04
	ctx.r[3].s64 = ctx.r[10].s64 + -24324;
	// 82638844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82638848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263885C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82638868: 4BE2E5B9  bl 0x82466e20
	ctx.lr = 0x8263886C;
	sub_82466E20(ctx, base);
	// 8263886C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82638880 size=24
    let mut pc: u32 = 0x82638880;
    'dispatch: loop {
        match pc {
            0x82638880 => {
    //   block [0x82638880..0x82638898)
	// 82638880: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638884: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82638888: 394AADE0  addi r10, r10, -0x5220
	ctx.r[10].s64 = ctx.r[10].s64 + -21024;
	// 8263888C: 816B4110  lwz r11, 0x4110(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16656 as u32) ) } as u64;
	// 82638890: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82638894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638898 size=116
    let mut pc: u32 = 0x82638898;
    'dispatch: loop {
        match pc {
            0x82638898 => {
    //   block [0x82638898..0x8263890C)
	// 82638898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263889C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826388A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826388A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826388A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826388AC: 390BADE0  addi r8, r11, -0x5220
	ctx.r[8].s64 = ctx.r[11].s64 + -21024;
	// 826388B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826388B4: 392A6078  addi r9, r10, 0x6078
	ctx.r[9].s64 = ctx.r[10].s64 + 24696;
	// 826388B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826388BC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826388C0: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 826388C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826388C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826388CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826388D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826388D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826388D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826388DC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826388E0: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826388E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826388E8: 386BA12C  addi r3, r11, -0x5ed4
	ctx.r[3].s64 = ctx.r[11].s64 + -24276;
	// 826388EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826388F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826388F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826388F8: 4BE2E529  bl 0x82466e20
	ctx.lr = 0x826388FC;
	sub_82466E20(ctx, base);
	// 826388FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638910 size=112
    let mut pc: u32 = 0x82638910;
    'dispatch: loop {
        match pc {
            0x82638910 => {
    //   block [0x82638910..0x82638980)
	// 82638910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263891C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638920: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638924: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263892C: 390B4114  addi r8, r11, 0x4114
	ctx.r[8].s64 = ctx.r[11].s64 + 16660;
	// 82638930: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82638934: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82638938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263893C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638948: 386AA15C  addi r3, r10, -0x5ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -24228;
	// 8263894C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263895C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263896C: 4BE2E4B5  bl 0x82466e20
	ctx.lr = 0x82638970;
	sub_82466E20(ctx, base);
	// 82638970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263897C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638980 size=116
    let mut pc: u32 = 0x82638980;
    'dispatch: loop {
        match pc {
            0x82638980 => {
    //   block [0x82638980..0x826389F4)
	// 82638980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263898C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82638990: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82638994: 390A4148  addi r8, r10, 0x4148
	ctx.r[8].s64 = ctx.r[10].s64 + 16712;
	// 82638998: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263899C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826389A0: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 826389A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826389A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826389AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826389B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826389B4: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826389B8: 396B608C  addi r11, r11, 0x608c
	ctx.r[11].s64 = ctx.r[11].s64 + 24716;
	// 826389BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826389C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826389C4: 386AA18C  addi r3, r10, -0x5e74
	ctx.r[3].s64 = ctx.r[10].s64 + -24180;
	// 826389C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826389CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826389D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826389D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826389D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826389DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826389E0: 4BE2E441  bl 0x82466e20
	ctx.lr = 0x826389E4;
	sub_82466E20(ctx, base);
	// 826389E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826389E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826389EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826389F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826389F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826389F8 size=112
    let mut pc: u32 = 0x826389F8;
    'dispatch: loop {
        match pc {
            0x826389F8 => {
    //   block [0x826389F8..0x82638A68)
	// 826389F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826389FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638A04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638A08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638A0C: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638A14: 390B4208  addi r8, r11, 0x4208
	ctx.r[8].s64 = ctx.r[11].s64 + 16904;
	// 82638A18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82638A1C: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82638A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638A24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638A30: 386AA1BC  addi r3, r10, -0x5e44
	ctx.r[3].s64 = ctx.r[10].s64 + -24132;
	// 82638A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638A54: 4BE2E3CD  bl 0x82466e20
	ctx.lr = 0x82638A58;
	sub_82466E20(ctx, base);
	// 82638A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638A68 size=108
    let mut pc: u32 = 0x82638A68;
    'dispatch: loop {
        match pc {
            0x82638A68 => {
    //   block [0x82638A68..0x82638AD4)
	// 82638A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638A74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638A7C: 38EB4220  addi r7, r11, 0x4220
	ctx.r[7].s64 = ctx.r[11].s64 + 16928;
	// 82638A80: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82638A84: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 82638A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638A8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82638A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638A98: 386AA1EC  addi r3, r10, -0x5e14
	ctx.r[3].s64 = ctx.r[10].s64 + -24084;
	// 82638A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82638AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82638AC0: 4BE2E361  bl 0x82466e20
	ctx.lr = 0x82638AC4;
	sub_82466E20(ctx, base);
	// 82638AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638AD8 size=116
    let mut pc: u32 = 0x82638AD8;
    'dispatch: loop {
        match pc {
            0x82638AD8 => {
    //   block [0x82638AD8..0x82638B4C)
	// 82638AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638AE4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82638AE8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82638AEC: 390A4358  addi r8, r10, 0x4358
	ctx.r[8].s64 = ctx.r[10].s64 + 17240;
	// 82638AF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638AF4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82638AF8: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638AFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638B00: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82638B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638B0C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82638B10: 396B60B0  addi r11, r11, 0x60b0
	ctx.r[11].s64 = ctx.r[11].s64 + 24752;
	// 82638B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638B1C: 386AA21C  addi r3, r10, -0x5de4
	ctx.r[3].s64 = ctx.r[10].s64 + -24036;
	// 82638B20: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82638B24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638B28: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82638B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638B38: 4BE2E2E9  bl 0x82466e20
	ctx.lr = 0x82638B3C;
	sub_82466E20(ctx, base);
	// 82638B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638B50 size=112
    let mut pc: u32 = 0x82638B50;
    'dispatch: loop {
        match pc {
            0x82638B50 => {
    //   block [0x82638B50..0x82638BC0)
	// 82638B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638B5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638B60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638B64: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638B6C: 390B4400  addi r8, r11, 0x4400
	ctx.r[8].s64 = ctx.r[11].s64 + 17408;
	// 82638B70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82638B74: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 82638B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638B7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638B88: 386AA24C  addi r3, r10, -0x5db4
	ctx.r[3].s64 = ctx.r[10].s64 + -23988;
	// 82638B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638BAC: 4BE2E275  bl 0x82466e20
	ctx.lr = 0x82638BB0;
	sub_82466E20(ctx, base);
	// 82638BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638BC0 size=112
    let mut pc: u32 = 0x82638BC0;
    'dispatch: loop {
        match pc {
            0x82638BC0 => {
    //   block [0x82638BC0..0x82638C30)
	// 82638BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638BCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638BD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638BD4: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638BDC: 390B4418  addi r8, r11, 0x4418
	ctx.r[8].s64 = ctx.r[11].s64 + 17432;
	// 82638BE0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82638BE4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82638BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638BEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638BF8: 386AA27C  addi r3, r10, -0x5d84
	ctx.r[3].s64 = ctx.r[10].s64 + -23940;
	// 82638BFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638C1C: 4BE2E205  bl 0x82466e20
	ctx.lr = 0x82638C20;
	sub_82466E20(ctx, base);
	// 82638C20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638C30 size=112
    let mut pc: u32 = 0x82638C30;
    'dispatch: loop {
        match pc {
            0x82638C30 => {
    //   block [0x82638C30..0x82638CA0)
	// 82638C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638C3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638C40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638C44: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638C4C: 390B4538  addi r8, r11, 0x4538
	ctx.r[8].s64 = ctx.r[11].s64 + 17720;
	// 82638C50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82638C54: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 82638C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638C5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638C60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638C68: 386AA2AC  addi r3, r10, -0x5d54
	ctx.r[3].s64 = ctx.r[10].s64 + -23892;
	// 82638C6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638C8C: 4BE2E195  bl 0x82466e20
	ctx.lr = 0x82638C90;
	sub_82466E20(ctx, base);
	// 82638C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638CA0 size=116
    let mut pc: u32 = 0x82638CA0;
    'dispatch: loop {
        match pc {
            0x82638CA0 => {
    //   block [0x82638CA0..0x82638D14)
	// 82638CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638CAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638CB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82638CB4: 390B4550  addi r8, r11, 0x4550
	ctx.r[8].s64 = ctx.r[11].s64 + 17744;
	// 82638CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638CBC: 392A60E8  addi r9, r10, 0x60e8
	ctx.r[9].s64 = ctx.r[10].s64 + 24808;
	// 82638CC0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638CC4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82638CC8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638CCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638CD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638CE4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82638CE8: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82638CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82638CF0: 386BA2DC  addi r3, r11, -0x5d24
	ctx.r[3].s64 = ctx.r[11].s64 + -23844;
	// 82638CF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82638CF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638D00: 4BE2E121  bl 0x82466e20
	ctx.lr = 0x82638D04;
	sub_82466E20(ctx, base);
	// 82638D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638D18 size=100
    let mut pc: u32 = 0x82638D18;
    'dispatch: loop {
        match pc {
            0x82638D18 => {
    //   block [0x82638D18..0x82638D7C)
	// 82638D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638D24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638D2C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638D38: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82638D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638D4C: 386AA30C  addi r3, r10, -0x5cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -23796;
	// 82638D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638D54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638D58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82638D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638D60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82638D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638D68: 4BE2E0B9  bl 0x82466e20
	ctx.lr = 0x82638D6C;
	sub_82466E20(ctx, base);
	// 82638D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638D80 size=112
    let mut pc: u32 = 0x82638D80;
    'dispatch: loop {
        match pc {
            0x82638D80 => {
    //   block [0x82638D80..0x82638DF0)
	// 82638D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638D90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638D94: 38AAA30C  addi r5, r10, -0x5cf4
	ctx.r[5].s64 = ctx.r[10].s64 + -23796;
	// 82638D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638D9C: 390B4580  addi r8, r11, 0x4580
	ctx.r[8].s64 = ctx.r[11].s64 + 17792;
	// 82638DA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82638DA4: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82638DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638DAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638DB8: 386AA33C  addi r3, r10, -0x5cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -23748;
	// 82638DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638DDC: 4BE2E045  bl 0x82466e20
	ctx.lr = 0x82638DE0;
	sub_82466E20(ctx, base);
	// 82638DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638DF0 size=112
    let mut pc: u32 = 0x82638DF0;
    'dispatch: loop {
        match pc {
            0x82638DF0 => {
    //   block [0x82638DF0..0x82638E60)
	// 82638DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638E00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638E04: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638E0C: 390B4598  addi r8, r11, 0x4598
	ctx.r[8].s64 = ctx.r[11].s64 + 17816;
	// 82638E10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82638E14: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82638E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638E28: 386AA36C  addi r3, r10, -0x5c94
	ctx.r[3].s64 = ctx.r[10].s64 + -23700;
	// 82638E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638E4C: 4BE2DFD5  bl 0x82466e20
	ctx.lr = 0x82638E50;
	sub_82466E20(ctx, base);
	// 82638E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638E60 size=112
    let mut pc: u32 = 0x82638E60;
    'dispatch: loop {
        match pc {
            0x82638E60 => {
    //   block [0x82638E60..0x82638ED0)
	// 82638E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638E6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638E70: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638E74: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638E7C: 390B4640  addi r8, r11, 0x4640
	ctx.r[8].s64 = ctx.r[11].s64 + 17984;
	// 82638E80: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82638E84: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 82638E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638E8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638E98: 386AA39C  addi r3, r10, -0x5c64
	ctx.r[3].s64 = ctx.r[10].s64 + -23652;
	// 82638E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638EBC: 4BE2DF65  bl 0x82466e20
	ctx.lr = 0x82638EC0;
	sub_82466E20(ctx, base);
	// 82638EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638ED0 size=112
    let mut pc: u32 = 0x82638ED0;
    'dispatch: loop {
        match pc {
            0x82638ED0 => {
    //   block [0x82638ED0..0x82638F40)
	// 82638ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638EE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82638EE4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638EEC: 390B4688  addi r8, r11, 0x4688
	ctx.r[8].s64 = ctx.r[11].s64 + 18056;
	// 82638EF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82638EF4: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82638EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638EFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638F08: 386AA3CC  addi r3, r10, -0x5c34
	ctx.r[3].s64 = ctx.r[10].s64 + -23604;
	// 82638F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82638F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638F2C: 4BE2DEF5  bl 0x82466e20
	ctx.lr = 0x82638F30;
	sub_82466E20(ctx, base);
	// 82638F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82638F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638F40 size=116
    let mut pc: u32 = 0x82638F40;
    'dispatch: loop {
        match pc {
            0x82638F40 => {
    //   block [0x82638F40..0x82638FB4)
	// 82638F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638F4C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82638F50: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82638F54: 390A46B8  addi r8, r10, 0x46b8
	ctx.r[8].s64 = ctx.r[10].s64 + 18104;
	// 82638F58: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638F5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82638F60: 38AA9D9C  addi r5, r10, -0x6264
	ctx.r[5].s64 = ctx.r[10].s64 + -25188;
	// 82638F64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638F68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82638F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82638F74: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 82638F78: 396B60FC  addi r11, r11, 0x60fc
	ctx.r[11].s64 = ctx.r[11].s64 + 24828;
	// 82638F7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638F80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638F84: 386AA3FC  addi r3, r10, -0x5c04
	ctx.r[3].s64 = ctx.r[10].s64 + -23556;
	// 82638F88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82638F8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638F90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82638F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82638FA0: 4BE2DE81  bl 0x82466e20
	ctx.lr = 0x82638FA4;
	sub_82466E20(ctx, base);
	// 82638FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82638FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


