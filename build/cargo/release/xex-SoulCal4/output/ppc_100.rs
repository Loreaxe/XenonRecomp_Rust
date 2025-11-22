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


pub fn sub_82638FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638FB8 size=100
    let mut pc: u32 = 0x82638FB8;
    'dispatch: loop {
        match pc {
            0x82638FB8 => {
    //   block [0x82638FB8..0x8263901C)
	// 82638FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638FC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638FCC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638FD8: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82638FDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638FEC: 386AA42C  addi r3, r10, -0x5bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -23508;
	// 82638FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638FF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638FF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82638FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639000: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82639004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639008: 4BE2DE19  bl 0x82466e20
	ctx.lr = 0x8263900C;
	sub_82466E20(ctx, base);
	// 8263900C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639020 size=108
    let mut pc: u32 = 0x82639020;
    'dispatch: loop {
        match pc {
            0x82639020 => {
    //   block [0x82639020..0x8263908C)
	// 82639020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263902C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639034: 38EB4778  addi r7, r11, 0x4778
	ctx.r[7].s64 = ctx.r[11].s64 + 18296;
	// 82639038: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263903C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82639040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263904C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639050: 386AA45C  addi r3, r10, -0x5ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -23460;
	// 82639054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263905C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263906C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639078: 4BE2DDA9  bl 0x82466e20
	ctx.lr = 0x8263907C;
	sub_82466E20(ctx, base);
	// 8263907C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639090 size=112
    let mut pc: u32 = 0x82639090;
    'dispatch: loop {
        match pc {
            0x82639090 => {
    //   block [0x82639090..0x82639100)
	// 82639090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263909C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826390A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826390A4: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 826390A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826390AC: 390B47A8  addi r8, r11, 0x47a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18344;
	// 826390B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826390B4: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826390B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826390BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826390C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826390C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826390C8: 386AA48C  addi r3, r10, -0x5b74
	ctx.r[3].s64 = ctx.r[10].s64 + -23412;
	// 826390CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826390D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826390D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826390D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826390DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826390E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826390E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826390E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826390EC: 4BE2DD35  bl 0x82466e20
	ctx.lr = 0x826390F0;
	sub_82466E20(ctx, base);
	// 826390F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826390F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826390F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826390FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639100 size=108
    let mut pc: u32 = 0x82639100;
    'dispatch: loop {
        match pc {
            0x82639100 => {
    //   block [0x82639100..0x8263916C)
	// 82639100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263910C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639114: 38EB47D8  addi r7, r11, 0x47d8
	ctx.r[7].s64 = ctx.r[11].s64 + 18392;
	// 82639118: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263911C: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82639120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263912C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639130: 386AA4BC  addi r3, r10, -0x5b44
	ctx.r[3].s64 = ctx.r[10].s64 + -23364;
	// 82639134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263913C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263914C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639158: 4BE2DCC9  bl 0x82466e20
	ctx.lr = 0x8263915C;
	sub_82466E20(ctx, base);
	// 8263915C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639170 size=112
    let mut pc: u32 = 0x82639170;
    'dispatch: loop {
        match pc {
            0x82639170 => {
    //   block [0x82639170..0x826391E0)
	// 82639170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263917C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639180: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639184: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 82639188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263918C: 390B4808  addi r8, r11, 0x4808
	ctx.r[8].s64 = ctx.r[11].s64 + 18440;
	// 82639190: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82639194: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82639198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263919C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826391A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826391A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826391A8: 386AA4EC  addi r3, r10, -0x5b14
	ctx.r[3].s64 = ctx.r[10].s64 + -23316;
	// 826391AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826391B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826391B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826391B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826391BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826391C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826391C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826391C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826391CC: 4BE2DC55  bl 0x82466e20
	ctx.lr = 0x826391D0;
	sub_82466E20(ctx, base);
	// 826391D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826391D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826391D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826391DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826391E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826391E0 size=108
    let mut pc: u32 = 0x826391E0;
    'dispatch: loop {
        match pc {
            0x826391E0 => {
    //   block [0x826391E0..0x8263924C)
	// 826391E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826391E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826391E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826391EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826391F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826391F4: 38EB4850  addi r7, r11, 0x4850
	ctx.r[7].s64 = ctx.r[11].s64 + 18512;
	// 826391F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826391FC: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82639200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263920C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639210: 386AA51C  addi r3, r10, -0x5ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -23268;
	// 82639214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263921C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263922C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639238: 4BE2DBE9  bl 0x82466e20
	ctx.lr = 0x8263923C;
	sub_82466E20(ctx, base);
	// 8263923C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639250 size=112
    let mut pc: u32 = 0x82639250;
    'dispatch: loop {
        match pc {
            0x82639250 => {
    //   block [0x82639250..0x826392C0)
	// 82639250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263925C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639260: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639264: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 82639268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263926C: 390B4880  addi r8, r11, 0x4880
	ctx.r[8].s64 = ctx.r[11].s64 + 18560;
	// 82639270: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82639274: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82639278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263927C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639288: 386AA54C  addi r3, r10, -0x5ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -23220;
	// 8263928C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263929C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826392A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826392A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826392A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826392AC: 4BE2DB75  bl 0x82466e20
	ctx.lr = 0x826392B0;
	sub_82466E20(ctx, base);
	// 826392B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826392B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826392B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826392BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826392C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826392C0 size=108
    let mut pc: u32 = 0x826392C0;
    'dispatch: loop {
        match pc {
            0x826392C0 => {
    //   block [0x826392C0..0x8263932C)
	// 826392C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826392C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826392C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826392CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826392D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826392D4: 38EB48C8  addi r7, r11, 0x48c8
	ctx.r[7].s64 = ctx.r[11].s64 + 18632;
	// 826392D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826392DC: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 826392E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826392E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826392E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826392EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826392F0: 386AA57C  addi r3, r10, -0x5a84
	ctx.r[3].s64 = ctx.r[10].s64 + -23172;
	// 826392F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826392F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826392FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263930C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639318: 4BE2DB09  bl 0x82466e20
	ctx.lr = 0x8263931C;
	sub_82466E20(ctx, base);
	// 8263931C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639330 size=112
    let mut pc: u32 = 0x82639330;
    'dispatch: loop {
        match pc {
            0x82639330 => {
    //   block [0x82639330..0x826393A0)
	// 82639330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263933C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639340: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639344: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 82639348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263934C: 390B48F8  addi r8, r11, 0x48f8
	ctx.r[8].s64 = ctx.r[11].s64 + 18680;
	// 82639350: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82639354: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82639358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263935C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639368: 386AA5AC  addi r3, r10, -0x5a54
	ctx.r[3].s64 = ctx.r[10].s64 + -23124;
	// 8263936C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263937C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263938C: 4BE2DA95  bl 0x82466e20
	ctx.lr = 0x82639390;
	sub_82466E20(ctx, base);
	// 82639390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263939C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826393A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826393A0 size=108
    let mut pc: u32 = 0x826393A0;
    'dispatch: loop {
        match pc {
            0x826393A0 => {
    //   block [0x826393A0..0x8263940C)
	// 826393A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826393A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826393A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826393AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826393B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826393B4: 38EB4948  addi r7, r11, 0x4948
	ctx.r[7].s64 = ctx.r[11].s64 + 18760;
	// 826393B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826393BC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826393C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826393C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826393C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826393CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826393D0: 386AA5DC  addi r3, r10, -0x5a24
	ctx.r[3].s64 = ctx.r[10].s64 + -23076;
	// 826393D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826393D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826393DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826393E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826393E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826393E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826393EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826393F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826393F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826393F8: 4BE2DA29  bl 0x82466e20
	ctx.lr = 0x826393FC;
	sub_82466E20(ctx, base);
	// 826393FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639410 size=112
    let mut pc: u32 = 0x82639410;
    'dispatch: loop {
        match pc {
            0x82639410 => {
    //   block [0x82639410..0x82639480)
	// 82639410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263941C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639420: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639424: 392A61B8  addi r9, r10, 0x61b8
	ctx.r[9].s64 = ctx.r[10].s64 + 25016;
	// 82639428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263942C: 390B49A8  addi r8, r11, 0x49a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18856;
	// 82639430: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82639434: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82639438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263943C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639448: 386AA60C  addi r3, r10, -0x59f4
	ctx.r[3].s64 = ctx.r[10].s64 + -23028;
	// 8263944C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639450: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263945C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263946C: 4BE2D9B5  bl 0x82466e20
	ctx.lr = 0x82639470;
	sub_82466E20(ctx, base);
	// 82639470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263947C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639480 size=108
    let mut pc: u32 = 0x82639480;
    'dispatch: loop {
        match pc {
            0x82639480 => {
    //   block [0x82639480..0x826394EC)
	// 82639480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263948C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639494: 38EB4A68  addi r7, r11, 0x4a68
	ctx.r[7].s64 = ctx.r[11].s64 + 19048;
	// 82639498: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263949C: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826394A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826394A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826394A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826394AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826394B0: 386AA63C  addi r3, r10, -0x59c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22980;
	// 826394B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826394B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826394BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826394C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826394C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826394C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826394CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826394D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826394D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826394D8: 4BE2D949  bl 0x82466e20
	ctx.lr = 0x826394DC;
	sub_82466E20(ctx, base);
	// 826394DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826394E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826394E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826394E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826394F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826394F0 size=108
    let mut pc: u32 = 0x826394F0;
    'dispatch: loop {
        match pc {
            0x826394F0 => {
    //   block [0x826394F0..0x8263955C)
	// 826394F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826394F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826394F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826394FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639504: 38EB4AB0  addi r7, r11, 0x4ab0
	ctx.r[7].s64 = ctx.r[11].s64 + 19120;
	// 82639508: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263950C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82639510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639514: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263951C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639520: 386AA66C  addi r3, r10, -0x5994
	ctx.r[3].s64 = ctx.r[10].s64 + -22932;
	// 82639524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263952C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263953C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639548: 4BE2D8D9  bl 0x82466e20
	ctx.lr = 0x8263954C;
	sub_82466E20(ctx, base);
	// 8263954C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639560 size=24
    let mut pc: u32 = 0x82639560;
    'dispatch: loop {
        match pc {
            0x82639560 => {
    //   block [0x82639560..0x82639578)
	// 82639560: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639564: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639568: 394AAEE8  addi r10, r10, -0x5118
	ctx.r[10].s64 = ctx.r[10].s64 + -20760;
	// 8263956C: 816B4940  lwz r11, 0x4940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18752 as u32) ) } as u64;
	// 82639570: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82639574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639578 size=116
    let mut pc: u32 = 0x82639578;
    'dispatch: loop {
        match pc {
            0x82639578 => {
    //   block [0x82639578..0x826395EC)
	// 82639578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639584: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82639588: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263958C: 392B6144  addi r9, r11, 0x6144
	ctx.r[9].s64 = ctx.r[11].s64 + 24900;
	// 82639590: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82639594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639598: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 8263959C: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 826395A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826395A4: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826395A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826395AC: 396BAEE8  addi r11, r11, -0x5118
	ctx.r[11].s64 = ctx.r[11].s64 + -20760;
	// 826395B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826395B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826395B8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826395BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826395C0: 386AA69C  addi r3, r10, -0x5964
	ctx.r[3].s64 = ctx.r[10].s64 + -22884;
	// 826395C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826395C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826395CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826395D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826395D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826395D8: 4BE2D849  bl 0x82466e20
	ctx.lr = 0x826395DC;
	sub_82466E20(ctx, base);
	// 826395DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826395E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826395E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826395E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826395F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826395F0 size=100
    let mut pc: u32 = 0x826395F0;
    'dispatch: loop {
        match pc {
            0x826395F0 => {
    //   block [0x826395F0..0x82639654)
	// 826395F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826395F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826395F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826395FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639604: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82639608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263960C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639610: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82639614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263961C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639624: 386AA6CC  addi r3, r10, -0x5934
	ctx.r[3].s64 = ctx.r[10].s64 + -22836;
	// 82639628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263962C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639630: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82639634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639638: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263963C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639640: 4BE2D7E1  bl 0x82466e20
	ctx.lr = 0x82639644;
	sub_82466E20(ctx, base);
	// 82639644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639658 size=24
    let mut pc: u32 = 0x82639658;
    'dispatch: loop {
        match pc {
            0x82639658 => {
    //   block [0x82639658..0x82639670)
	// 82639658: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263965C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639660: 394AB098  addi r10, r10, -0x4f68
	ctx.r[10].s64 = ctx.r[10].s64 + -20328;
	// 82639664: 816B4B44  lwz r11, 0x4b44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19268 as u32) ) } as u64;
	// 82639668: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263966C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639670 size=116
    let mut pc: u32 = 0x82639670;
    'dispatch: loop {
        match pc {
            0x82639670 => {
    //   block [0x82639670..0x826396E4)
	// 82639670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263967C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82639680: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639684: 390BB098  addi r8, r11, -0x4f68
	ctx.r[8].s64 = ctx.r[11].s64 + -20328;
	// 82639688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263968C: 392A6250  addi r9, r10, 0x6250
	ctx.r[9].s64 = ctx.r[10].s64 + 25168;
	// 82639690: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639694: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82639698: 38AAA6CC  addi r5, r10, -0x5934
	ctx.r[5].s64 = ctx.r[10].s64 + -22836;
	// 8263969C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826396A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826396A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826396A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826396AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826396B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826396B4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826396B8: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826396BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826396C0: 386BA6FC  addi r3, r11, -0x5904
	ctx.r[3].s64 = ctx.r[11].s64 + -22788;
	// 826396C4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826396C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826396CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826396D0: 4BE2D751  bl 0x82466e20
	ctx.lr = 0x826396D4;
	sub_82466E20(ctx, base);
	// 826396D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826396D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826396DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826396E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826396E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826396E8 size=112
    let mut pc: u32 = 0x826396E8;
    'dispatch: loop {
        match pc {
            0x826396E8 => {
    //   block [0x826396E8..0x82639758)
	// 826396E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826396EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826396F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826396F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826396F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826396FC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639704: 390B4B48  addi r8, r11, 0x4b48
	ctx.r[8].s64 = ctx.r[11].s64 + 19272;
	// 82639708: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8263970C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82639710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639714: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263971C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639720: 386AA72C  addi r3, r10, -0x58d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22740;
	// 82639724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263972C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263973C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639744: 4BE2D6DD  bl 0x82466e20
	ctx.lr = 0x82639748;
	sub_82466E20(ctx, base);
	// 82639748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263974C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639758 size=108
    let mut pc: u32 = 0x82639758;
    'dispatch: loop {
        match pc {
            0x82639758 => {
    //   block [0x82639758..0x826397C4)
	// 82639758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639764: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263976C: 38EB4C20  addi r7, r11, 0x4c20
	ctx.r[7].s64 = ctx.r[11].s64 + 19488;
	// 82639770: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82639774: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82639778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263977C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639788: 386AA75C  addi r3, r10, -0x58a4
	ctx.r[3].s64 = ctx.r[10].s64 + -22692;
	// 8263978C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263979C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826397A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826397A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826397A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826397AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826397B0: 4BE2D671  bl 0x82466e20
	ctx.lr = 0x826397B4;
	sub_82466E20(ctx, base);
	// 826397B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826397B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826397BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826397C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826397C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826397C8 size=108
    let mut pc: u32 = 0x826397C8;
    'dispatch: loop {
        match pc {
            0x826397C8 => {
    //   block [0x826397C8..0x82639834)
	// 826397C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826397CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826397D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826397D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826397D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826397DC: 38EB4C98  addi r7, r11, 0x4c98
	ctx.r[7].s64 = ctx.r[11].s64 + 19608;
	// 826397E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826397E4: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826397E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826397EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826397F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826397F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826397F8: 386AA78C  addi r3, r10, -0x5874
	ctx.r[3].s64 = ctx.r[10].s64 + -22644;
	// 826397FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263980C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263981C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639820: 4BE2D601  bl 0x82466e20
	ctx.lr = 0x82639824;
	sub_82466E20(ctx, base);
	// 82639824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263982C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639838 size=112
    let mut pc: u32 = 0x82639838;
    'dispatch: loop {
        match pc {
            0x82639838 => {
    //   block [0x82639838..0x826398A8)
	// 82639838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639848: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263984C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639854: 390B4CE0  addi r8, r11, 0x4ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 19680;
	// 82639858: 39200013  li r9, 0x13
	ctx.r[9].s64 = 19;
	// 8263985C: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82639860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263986C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639870: 386AA7BC  addi r3, r10, -0x5844
	ctx.r[3].s64 = ctx.r[10].s64 + -22596;
	// 82639874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263987C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263988C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639894: 4BE2D58D  bl 0x82466e20
	ctx.lr = 0x82639898;
	sub_82466E20(ctx, base);
	// 82639898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263989C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826398A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826398A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826398A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826398A8 size=108
    let mut pc: u32 = 0x826398A8;
    'dispatch: loop {
        match pc {
            0x826398A8 => {
    //   block [0x826398A8..0x82639914)
	// 826398A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826398AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826398B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826398B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826398B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826398BC: 38EB4EA8  addi r7, r11, 0x4ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 20136;
	// 826398C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826398C4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826398C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826398CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826398D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826398D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826398D8: 386AA7EC  addi r3, r10, -0x5814
	ctx.r[3].s64 = ctx.r[10].s64 + -22548;
	// 826398DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826398E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826398E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826398E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826398EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826398F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826398F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826398F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826398FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639900: 4BE2D521  bl 0x82466e20
	ctx.lr = 0x82639904;
	sub_82466E20(ctx, base);
	// 82639904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263990C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639918 size=24
    let mut pc: u32 = 0x82639918;
    'dispatch: loop {
        match pc {
            0x82639918 => {
    //   block [0x82639918..0x82639930)
	// 82639918: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263991C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639920: 394AB1B8  addi r10, r10, -0x4e48
	ctx.r[10].s64 = ctx.r[10].s64 + -20040;
	// 82639924: 816B4EC0  lwz r11, 0x4ec0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20160 as u32) ) } as u64;
	// 82639928: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263992C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639930 size=112
    let mut pc: u32 = 0x82639930;
    'dispatch: loop {
        match pc {
            0x82639930 => {
    //   block [0x82639930..0x826399A0)
	// 82639930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263993C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639940: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82639944: 392A62A8  addi r9, r10, 0x62a8
	ctx.r[9].s64 = ctx.r[10].s64 + 25256;
	// 82639948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263994C: 390BB1B8  addi r8, r11, -0x4e48
	ctx.r[8].s64 = ctx.r[11].s64 + -20040;
	// 82639950: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82639954: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82639958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263995C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639968: 386AA81C  addi r3, r10, -0x57e4
	ctx.r[3].s64 = ctx.r[10].s64 + -22500;
	// 8263996C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639970: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263997C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263998C: 4BE2D495  bl 0x82466e20
	ctx.lr = 0x82639990;
	sub_82466E20(ctx, base);
	// 82639990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263999C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826399A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826399A0 size=108
    let mut pc: u32 = 0x826399A0;
    'dispatch: loop {
        match pc {
            0x826399A0 => {
    //   block [0x826399A0..0x82639A0C)
	// 826399A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826399A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826399A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826399AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826399B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826399B4: 38EB4EC8  addi r7, r11, 0x4ec8
	ctx.r[7].s64 = ctx.r[11].s64 + 20168;
	// 826399B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826399BC: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826399C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826399C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826399C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826399CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826399D0: 386AA84C  addi r3, r10, -0x57b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22452;
	// 826399D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826399D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826399DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826399E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826399E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826399E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826399EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826399F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826399F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826399F8: 4BE2D429  bl 0x82466e20
	ctx.lr = 0x826399FC;
	sub_82466E20(ctx, base);
	// 826399FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639A10 size=112
    let mut pc: u32 = 0x82639A10;
    'dispatch: loop {
        match pc {
            0x82639A10 => {
    //   block [0x82639A10..0x82639A80)
	// 82639A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639A20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639A24: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639A2C: 390B4F28  addi r8, r11, 0x4f28
	ctx.r[8].s64 = ctx.r[11].s64 + 20264;
	// 82639A30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82639A34: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 82639A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639A48: 386AA87C  addi r3, r10, -0x5784
	ctx.r[3].s64 = ctx.r[10].s64 + -22404;
	// 82639A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639A6C: 4BE2D3B5  bl 0x82466e20
	ctx.lr = 0x82639A70;
	sub_82466E20(ctx, base);
	// 82639A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639A80 size=108
    let mut pc: u32 = 0x82639A80;
    'dispatch: loop {
        match pc {
            0x82639A80 => {
    //   block [0x82639A80..0x82639AEC)
	// 82639A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639A8C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639A94: 38EB4F40  addi r7, r11, 0x4f40
	ctx.r[7].s64 = ctx.r[11].s64 + 20288;
	// 82639A98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82639A9C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82639AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639AB0: 386AA8AC  addi r3, r10, -0x5754
	ctx.r[3].s64 = ctx.r[10].s64 + -22356;
	// 82639AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639AD8: 4BE2D349  bl 0x82466e20
	ctx.lr = 0x82639ADC;
	sub_82466E20(ctx, base);
	// 82639ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639AF0 size=108
    let mut pc: u32 = 0x82639AF0;
    'dispatch: loop {
        match pc {
            0x82639AF0 => {
    //   block [0x82639AF0..0x82639B5C)
	// 82639AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639AFC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639B04: 38EB4FA0  addi r7, r11, 0x4fa0
	ctx.r[7].s64 = ctx.r[11].s64 + 20384;
	// 82639B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639B0C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82639B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639B20: 386AA8DC  addi r3, r10, -0x5724
	ctx.r[3].s64 = ctx.r[10].s64 + -22308;
	// 82639B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639B48: 4BE2D2D9  bl 0x82466e20
	ctx.lr = 0x82639B4C;
	sub_82466E20(ctx, base);
	// 82639B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639B60 size=116
    let mut pc: u32 = 0x82639B60;
    'dispatch: loop {
        match pc {
            0x82639B60 => {
    //   block [0x82639B60..0x82639BD4)
	// 82639B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639B6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639B70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639B74: 390B4FD0  addi r8, r11, 0x4fd0
	ctx.r[8].s64 = ctx.r[11].s64 + 20432;
	// 82639B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639B7C: 392A62D4  addi r9, r10, 0x62d4
	ctx.r[9].s64 = ctx.r[10].s64 + 25300;
	// 82639B80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639B84: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82639B88: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639B8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639B94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639BA4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82639BA8: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82639BAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639BB0: 386BA90C  addi r3, r11, -0x56f4
	ctx.r[3].s64 = ctx.r[11].s64 + -22260;
	// 82639BB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639BB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639BC0: 4BE2D261  bl 0x82466e20
	ctx.lr = 0x82639BC4;
	sub_82466E20(ctx, base);
	// 82639BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639BD8 size=96
    let mut pc: u32 = 0x82639BD8;
    'dispatch: loop {
        match pc {
            0x82639BD8 => {
    //   block [0x82639BD8..0x82639C38)
	// 82639BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639BE4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639BEC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82639BF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639BF8: 386AA93C  addi r3, r10, -0x56c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22212;
	// 82639BFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639C04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82639C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82639C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82639C24: 4BE2D1FD  bl 0x82466e20
	ctx.lr = 0x82639C28;
	sub_82466E20(ctx, base);
	// 82639C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639C38 size=112
    let mut pc: u32 = 0x82639C38;
    'dispatch: loop {
        match pc {
            0x82639C38 => {
    //   block [0x82639C38..0x82639CA8)
	// 82639C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639C44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639C48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639C4C: 38AAA93C  addi r5, r10, -0x56c4
	ctx.r[5].s64 = ctx.r[10].s64 + -22212;
	// 82639C50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639C54: 390B4FE8  addi r8, r11, 0x4fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 20456;
	// 82639C58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82639C5C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82639C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639C70: 386AA96C  addi r3, r10, -0x5694
	ctx.r[3].s64 = ctx.r[10].s64 + -22164;
	// 82639C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639C94: 4BE2D18D  bl 0x82466e20
	ctx.lr = 0x82639C98;
	sub_82466E20(ctx, base);
	// 82639C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639CA8 size=112
    let mut pc: u32 = 0x82639CA8;
    'dispatch: loop {
        match pc {
            0x82639CA8 => {
    //   block [0x82639CA8..0x82639D18)
	// 82639CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639CB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639CB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639CBC: 392A62F0  addi r9, r10, 0x62f0
	ctx.r[9].s64 = ctx.r[10].s64 + 25328;
	// 82639CC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639CC4: 390B5020  addi r8, r11, 0x5020
	ctx.r[8].s64 = ctx.r[11].s64 + 20512;
	// 82639CC8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82639CCC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82639CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639CE0: 386AA99C  addi r3, r10, -0x5664
	ctx.r[3].s64 = ctx.r[10].s64 + -22116;
	// 82639CE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639CE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639D04: 4BE2D11D  bl 0x82466e20
	ctx.lr = 0x82639D08;
	sub_82466E20(ctx, base);
	// 82639D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639D18 size=108
    let mut pc: u32 = 0x82639D18;
    'dispatch: loop {
        match pc {
            0x82639D18 => {
    //   block [0x82639D18..0x82639D84)
	// 82639D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639D24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639D28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639D2C: 38EB50C8  addi r7, r11, 0x50c8
	ctx.r[7].s64 = ctx.r[11].s64 + 20680;
	// 82639D30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639D34: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82639D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639D48: 386AA9CC  addi r3, r10, -0x5634
	ctx.r[3].s64 = ctx.r[10].s64 + -22068;
	// 82639D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639D70: 4BE2D0B1  bl 0x82466e20
	ctx.lr = 0x82639D74;
	sub_82466E20(ctx, base);
	// 82639D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639D88 size=108
    let mut pc: u32 = 0x82639D88;
    'dispatch: loop {
        match pc {
            0x82639D88 => {
    //   block [0x82639D88..0x82639DF4)
	// 82639D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639D94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639D98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639D9C: 38EB50F8  addi r7, r11, 0x50f8
	ctx.r[7].s64 = ctx.r[11].s64 + 20728;
	// 82639DA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639DA4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82639DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639DAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639DB8: 386AA9FC  addi r3, r10, -0x5604
	ctx.r[3].s64 = ctx.r[10].s64 + -22020;
	// 82639DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639DE0: 4BE2D041  bl 0x82466e20
	ctx.lr = 0x82639DE4;
	sub_82466E20(ctx, base);
	// 82639DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639DF8 size=28
    let mut pc: u32 = 0x82639DF8;
    'dispatch: loop {
        match pc {
            0x82639DF8 => {
    //   block [0x82639DF8..0x82639E14)
	// 82639DF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639DFC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639E00: 394AB1E8  addi r10, r10, -0x4e18
	ctx.r[10].s64 = ctx.r[10].s64 + -19992;
	// 82639E04: 816B501C  lwz r11, 0x501c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20508 as u32) ) } as u64;
	// 82639E08: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82639E0C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82639E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639E18 size=112
    let mut pc: u32 = 0x82639E18;
    'dispatch: loop {
        match pc {
            0x82639E18 => {
    //   block [0x82639E18..0x82639E88)
	// 82639E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639E24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639E28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82639E2C: 392A6480  addi r9, r10, 0x6480
	ctx.r[9].s64 = ctx.r[10].s64 + 25728;
	// 82639E30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639E34: 390BB1E8  addi r8, r11, -0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + -19992;
	// 82639E38: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82639E3C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82639E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639E44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639E50: 386AAA2C  addi r3, r10, -0x55d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21972;
	// 82639E54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639E58: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82639E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639E74: 4BE2CFAD  bl 0x82466e20
	ctx.lr = 0x82639E78;
	sub_82466E20(ctx, base);
	// 82639E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639E88 size=108
    let mut pc: u32 = 0x82639E88;
    'dispatch: loop {
        match pc {
            0x82639E88 => {
    //   block [0x82639E88..0x82639EF4)
	// 82639E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639E94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639E98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639E9C: 38EB5130  addi r7, r11, 0x5130
	ctx.r[7].s64 = ctx.r[11].s64 + 20784;
	// 82639EA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639EA4: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82639EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639EAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639EB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639EB8: 386AAA5C  addi r3, r10, -0x55a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21924;
	// 82639EBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639EE0: 4BE2CF41  bl 0x82466e20
	ctx.lr = 0x82639EE4;
	sub_82466E20(ctx, base);
	// 82639EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639EF8 size=108
    let mut pc: u32 = 0x82639EF8;
    'dispatch: loop {
        match pc {
            0x82639EF8 => {
    //   block [0x82639EF8..0x82639F64)
	// 82639EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639F04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639F08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639F0C: 38EB5160  addi r7, r11, 0x5160
	ctx.r[7].s64 = ctx.r[11].s64 + 20832;
	// 82639F10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639F14: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82639F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639F1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639F28: 386AAA8C  addi r3, r10, -0x5574
	ctx.r[3].s64 = ctx.r[10].s64 + -21876;
	// 82639F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639F50: 4BE2CED1  bl 0x82466e20
	ctx.lr = 0x82639F54;
	sub_82466E20(ctx, base);
	// 82639F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639F68 size=108
    let mut pc: u32 = 0x82639F68;
    'dispatch: loop {
        match pc {
            0x82639F68 => {
    //   block [0x82639F68..0x82639FD4)
	// 82639F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639F74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639F78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639F7C: 38EB5190  addi r7, r11, 0x5190
	ctx.r[7].s64 = ctx.r[11].s64 + 20880;
	// 82639F80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82639F84: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82639F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639F98: 386AAABC  addi r3, r10, -0x5544
	ctx.r[3].s64 = ctx.r[10].s64 + -21828;
	// 82639F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639FC0: 4BE2CE61  bl 0x82466e20
	ctx.lr = 0x82639FC4;
	sub_82466E20(ctx, base);
	// 82639FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639FD8 size=24
    let mut pc: u32 = 0x82639FD8;
    'dispatch: loop {
        match pc {
            0x82639FD8 => {
    //   block [0x82639FD8..0x82639FF0)
	// 82639FD8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639FDC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639FE0: 394AB2A8  addi r10, r10, -0x4d58
	ctx.r[10].s64 = ctx.r[10].s64 + -19800;
	// 82639FE4: 816B51A8  lwz r11, 0x51a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20904 as u32) ) } as u64;
	// 82639FE8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82639FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639FF0 size=112
    let mut pc: u32 = 0x82639FF0;
    'dispatch: loop {
        match pc {
            0x82639FF0 => {
    //   block [0x82639FF0..0x8263A060)
	// 82639FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639FFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A000: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A004: 392A64D4  addi r9, r10, 0x64d4
	ctx.r[9].s64 = ctx.r[10].s64 + 25812;
	// 8263A008: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A00C: 390BB2A8  addi r8, r11, -0x4d58
	ctx.r[8].s64 = ctx.r[11].s64 + -19800;
	// 8263A010: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263A014: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8263A018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A01C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A028: 386AAAEC  addi r3, r10, -0x5514
	ctx.r[3].s64 = ctx.r[10].s64 + -21780;
	// 8263A02C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A030: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263A034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A04C: 4BE2CDD5  bl 0x82466e20
	ctx.lr = 0x8263A050;
	sub_82466E20(ctx, base);
	// 8263A050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A060 size=112
    let mut pc: u32 = 0x8263A060;
    'dispatch: loop {
        match pc {
            0x8263A060 => {
    //   block [0x8263A060..0x8263A0D0)
	// 8263A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A06C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A070: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A074: 392A6510  addi r9, r10, 0x6510
	ctx.r[9].s64 = ctx.r[10].s64 + 25872;
	// 8263A078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A07C: 390B51B8  addi r8, r11, 0x51b8
	ctx.r[8].s64 = ctx.r[11].s64 + 20920;
	// 8263A080: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263A084: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8263A088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A08C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A098: 386AAB1C  addi r3, r10, -0x54e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21732;
	// 8263A09C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A0A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8263A0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A0BC: 4BE2CD65  bl 0x82466e20
	ctx.lr = 0x8263A0C0;
	sub_82466E20(ctx, base);
	// 8263A0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A0D0 size=108
    let mut pc: u32 = 0x8263A0D0;
    'dispatch: loop {
        match pc {
            0x8263A0D0 => {
    //   block [0x8263A0D0..0x8263A13C)
	// 8263A0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A0DC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A0E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A0E4: 38EB5200  addi r7, r11, 0x5200
	ctx.r[7].s64 = ctx.r[11].s64 + 20992;
	// 8263A0E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A0EC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8263A0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A0F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A0F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A100: 386AAB4C  addi r3, r10, -0x54b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21684;
	// 8263A104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A128: 4BE2CCF9  bl 0x82466e20
	ctx.lr = 0x8263A12C;
	sub_82466E20(ctx, base);
	// 8263A12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A140 size=108
    let mut pc: u32 = 0x8263A140;
    'dispatch: loop {
        match pc {
            0x8263A140 => {
    //   block [0x8263A140..0x8263A1AC)
	// 8263A140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A14C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A150: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A154: 38EB5230  addi r7, r11, 0x5230
	ctx.r[7].s64 = ctx.r[11].s64 + 21040;
	// 8263A158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A15C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8263A160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A170: 386AAB7C  addi r3, r10, -0x5484
	ctx.r[3].s64 = ctx.r[10].s64 + -21636;
	// 8263A174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A198: 4BE2CC89  bl 0x82466e20
	ctx.lr = 0x8263A19C;
	sub_82466E20(ctx, base);
	// 8263A19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A1B0 size=112
    let mut pc: u32 = 0x8263A1B0;
    'dispatch: loop {
        match pc {
            0x8263A1B0 => {
    //   block [0x8263A1B0..0x8263A220)
	// 8263A1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A1BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A1C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A1C4: 392A6548  addi r9, r10, 0x6548
	ctx.r[9].s64 = ctx.r[10].s64 + 25928;
	// 8263A1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A1CC: 390B5260  addi r8, r11, 0x5260
	ctx.r[8].s64 = ctx.r[11].s64 + 21088;
	// 8263A1D0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8263A1D4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8263A1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A1DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A1E8: 386AABAC  addi r3, r10, -0x5454
	ctx.r[3].s64 = ctx.r[10].s64 + -21588;
	// 8263A1EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A1F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263A1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A20C: 4BE2CC15  bl 0x82466e20
	ctx.lr = 0x8263A210;
	sub_82466E20(ctx, base);
	// 8263A210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A220 size=108
    let mut pc: u32 = 0x8263A220;
    'dispatch: loop {
        match pc {
            0x8263A220 => {
    //   block [0x8263A220..0x8263A28C)
	// 8263A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A22C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A234: 38EB52C0  addi r7, r11, 0x52c0
	ctx.r[7].s64 = ctx.r[11].s64 + 21184;
	// 8263A238: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8263A23C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8263A240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A250: 386AABDC  addi r3, r10, -0x5424
	ctx.r[3].s64 = ctx.r[10].s64 + -21540;
	// 8263A254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A278: 4BE2CBA9  bl 0x82466e20
	ctx.lr = 0x8263A27C;
	sub_82466E20(ctx, base);
	// 8263A27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A290 size=108
    let mut pc: u32 = 0x8263A290;
    'dispatch: loop {
        match pc {
            0x8263A290 => {
    //   block [0x8263A290..0x8263A2FC)
	// 8263A290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A29C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A2A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A2A4: 38EB53C8  addi r7, r11, 0x53c8
	ctx.r[7].s64 = ctx.r[11].s64 + 21448;
	// 8263A2A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A2AC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8263A2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A2B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A2B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A2C0: 386AAC0C  addi r3, r10, -0x53f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21492;
	// 8263A2C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A2CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A2E8: 4BE2CB39  bl 0x82466e20
	ctx.lr = 0x8263A2EC;
	sub_82466E20(ctx, base);
	// 8263A2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A300 size=108
    let mut pc: u32 = 0x8263A300;
    'dispatch: loop {
        match pc {
            0x8263A300 => {
    //   block [0x8263A300..0x8263A36C)
	// 8263A300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A30C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A314: 38EB53E0  addi r7, r11, 0x53e0
	ctx.r[7].s64 = ctx.r[11].s64 + 21472;
	// 8263A318: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263A31C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8263A320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A330: 386AAC3C  addi r3, r10, -0x53c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21444;
	// 8263A334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A358: 4BE2CAC9  bl 0x82466e20
	ctx.lr = 0x8263A35C;
	sub_82466E20(ctx, base);
	// 8263A35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263A370 size=24
    let mut pc: u32 = 0x8263A370;
    'dispatch: loop {
        match pc {
            0x8263A370 => {
    //   block [0x8263A370..0x8263A388)
	// 8263A370: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A374: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263A378: 394AB380  addi r10, r10, -0x4c80
	ctx.r[10].s64 = ctx.r[10].s64 + -19584;
	// 8263A37C: 816B5470  lwz r11, 0x5470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21616 as u32) ) } as u64;
	// 8263A380: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263A384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A388 size=108
    let mut pc: u32 = 0x8263A388;
    'dispatch: loop {
        match pc {
            0x8263A388 => {
    //   block [0x8263A388..0x8263A3F4)
	// 8263A388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A394: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A39C: 38EBB380  addi r7, r11, -0x4c80
	ctx.r[7].s64 = ctx.r[11].s64 + -19584;
	// 8263A3A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A3A4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8263A3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A3AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A3B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A3B8: 386AAC6C  addi r3, r10, -0x5394
	ctx.r[3].s64 = ctx.r[10].s64 + -21396;
	// 8263A3BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A3E0: 4BE2CA41  bl 0x82466e20
	ctx.lr = 0x8263A3E4;
	sub_82466E20(ctx, base);
	// 8263A3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263A3F8 size=24
    let mut pc: u32 = 0x8263A3F8;
    'dispatch: loop {
        match pc {
            0x8263A3F8 => {
    //   block [0x8263A3F8..0x8263A410)
	// 8263A3F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A3FC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263A400: 394AB3B0  addi r10, r10, -0x4c50
	ctx.r[10].s64 = ctx.r[10].s64 + -19536;
	// 8263A404: 816B5470  lwz r11, 0x5470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21616 as u32) ) } as u64;
	// 8263A408: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263A40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A410 size=108
    let mut pc: u32 = 0x8263A410;
    'dispatch: loop {
        match pc {
            0x8263A410 => {
    //   block [0x8263A410..0x8263A47C)
	// 8263A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A41C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A424: 38EBB3B0  addi r7, r11, -0x4c50
	ctx.r[7].s64 = ctx.r[11].s64 + -19536;
	// 8263A428: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A42C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8263A430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A440: 386AAC9C  addi r3, r10, -0x5364
	ctx.r[3].s64 = ctx.r[10].s64 + -21348;
	// 8263A444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A468: 4BE2C9B9  bl 0x82466e20
	ctx.lr = 0x8263A46C;
	sub_82466E20(ctx, base);
	// 8263A46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A480 size=108
    let mut pc: u32 = 0x8263A480;
    'dispatch: loop {
        match pc {
            0x8263A480 => {
    //   block [0x8263A480..0x8263A4EC)
	// 8263A480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A48C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A494: 38EB5458  addi r7, r11, 0x5458
	ctx.r[7].s64 = ctx.r[11].s64 + 21592;
	// 8263A498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A49C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8263A4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A4A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A4A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A4B0: 386AACCC  addi r3, r10, -0x5334
	ctx.r[3].s64 = ctx.r[10].s64 + -21300;
	// 8263A4B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A4D8: 4BE2C949  bl 0x82466e20
	ctx.lr = 0x8263A4DC;
	sub_82466E20(ctx, base);
	// 8263A4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263A4F0 size=24
    let mut pc: u32 = 0x8263A4F0;
    'dispatch: loop {
        match pc {
            0x8263A4F0 => {
    //   block [0x8263A4F0..0x8263A508)
	// 8263A4F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A4F4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263A4F8: 394AB3E0  addi r10, r10, -0x4c20
	ctx.r[10].s64 = ctx.r[10].s64 + -19488;
	// 8263A4FC: 816B5470  lwz r11, 0x5470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21616 as u32) ) } as u64;
	// 8263A500: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263A504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A508 size=108
    let mut pc: u32 = 0x8263A508;
    'dispatch: loop {
        match pc {
            0x8263A508 => {
    //   block [0x8263A508..0x8263A574)
	// 8263A508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A514: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A51C: 38EBB3E0  addi r7, r11, -0x4c20
	ctx.r[7].s64 = ctx.r[11].s64 + -19488;
	// 8263A520: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A524: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8263A528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A52C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A538: 386AACFC  addi r3, r10, -0x5304
	ctx.r[3].s64 = ctx.r[10].s64 + -21252;
	// 8263A53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A560: 4BE2C8C1  bl 0x82466e20
	ctx.lr = 0x8263A564;
	sub_82466E20(ctx, base);
	// 8263A564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A578 size=112
    let mut pc: u32 = 0x8263A578;
    'dispatch: loop {
        match pc {
            0x8263A578 => {
    //   block [0x8263A578..0x8263A5E8)
	// 8263A578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A584: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A588: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A58C: 392A658C  addi r9, r10, 0x658c
	ctx.r[9].s64 = ctx.r[10].s64 + 25996;
	// 8263A590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A594: 390B5474  addi r8, r11, 0x5474
	ctx.r[8].s64 = ctx.r[11].s64 + 21620;
	// 8263A598: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263A59C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8263A5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A5A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A5B0: 386AAD2C  addi r3, r10, -0x52d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21204;
	// 8263A5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A5D4: 4BE2C84D  bl 0x82466e20
	ctx.lr = 0x8263A5D8;
	sub_82466E20(ctx, base);
	// 8263A5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A5E8 size=108
    let mut pc: u32 = 0x8263A5E8;
    'dispatch: loop {
        match pc {
            0x8263A5E8 => {
    //   block [0x8263A5E8..0x8263A654)
	// 8263A5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A5F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A5FC: 38EB54A4  addi r7, r11, 0x54a4
	ctx.r[7].s64 = ctx.r[11].s64 + 21668;
	// 8263A600: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A604: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8263A608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A60C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A618: 386AAD5C  addi r3, r10, -0x52a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21156;
	// 8263A61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A640: 4BE2C7E1  bl 0x82466e20
	ctx.lr = 0x8263A644;
	sub_82466E20(ctx, base);
	// 8263A644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A658 size=108
    let mut pc: u32 = 0x8263A658;
    'dispatch: loop {
        match pc {
            0x8263A658 => {
    //   block [0x8263A658..0x8263A6C4)
	// 8263A658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A664: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A66C: 38EB54D4  addi r7, r11, 0x54d4
	ctx.r[7].s64 = ctx.r[11].s64 + 21716;
	// 8263A670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A674: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 8263A678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A67C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A688: 386AAD8C  addi r3, r10, -0x5274
	ctx.r[3].s64 = ctx.r[10].s64 + -21108;
	// 8263A68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A6B0: 4BE2C771  bl 0x82466e20
	ctx.lr = 0x8263A6B4;
	sub_82466E20(ctx, base);
	// 8263A6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A6C8 size=108
    let mut pc: u32 = 0x8263A6C8;
    'dispatch: loop {
        match pc {
            0x8263A6C8 => {
    //   block [0x8263A6C8..0x8263A734)
	// 8263A6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A6D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A6DC: 38EB54EC  addi r7, r11, 0x54ec
	ctx.r[7].s64 = ctx.r[11].s64 + 21740;
	// 8263A6E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A6E4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8263A6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A6EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A6F8: 386AADBC  addi r3, r10, -0x5244
	ctx.r[3].s64 = ctx.r[10].s64 + -21060;
	// 8263A6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A720: 4BE2C701  bl 0x82466e20
	ctx.lr = 0x8263A724;
	sub_82466E20(ctx, base);
	// 8263A724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A738 size=112
    let mut pc: u32 = 0x8263A738;
    'dispatch: loop {
        match pc {
            0x8263A738 => {
    //   block [0x8263A738..0x8263A7A8)
	// 8263A738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A748: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A74C: 38AAAE1C  addi r5, r10, -0x51e4
	ctx.r[5].s64 = ctx.r[10].s64 + -20964;
	// 8263A750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A754: 390B551C  addi r8, r11, 0x551c
	ctx.r[8].s64 = ctx.r[11].s64 + 21788;
	// 8263A758: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263A75C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8263A760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A764: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A770: 386AADEC  addi r3, r10, -0x5214
	ctx.r[3].s64 = ctx.r[10].s64 + -21012;
	// 8263A774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263A778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A77C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A794: 4BE2C68D  bl 0x82466e20
	ctx.lr = 0x8263A798;
	sub_82466E20(ctx, base);
	// 8263A798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A7A8 size=108
    let mut pc: u32 = 0x8263A7A8;
    'dispatch: loop {
        match pc {
            0x8263A7A8 => {
    //   block [0x8263A7A8..0x8263A814)
	// 8263A7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A7B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A7BC: 38EB5534  addi r7, r11, 0x5534
	ctx.r[7].s64 = ctx.r[11].s64 + 21812;
	// 8263A7C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A7C4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8263A7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A7CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A7D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A7D8: 386AAE1C  addi r3, r10, -0x51e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20964;
	// 8263A7DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A7FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A800: 4BE2C621  bl 0x82466e20
	ctx.lr = 0x8263A804;
	sub_82466E20(ctx, base);
	// 8263A804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A818 size=108
    let mut pc: u32 = 0x8263A818;
    'dispatch: loop {
        match pc {
            0x8263A818 => {
    //   block [0x8263A818..0x8263A884)
	// 8263A818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A824: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A82C: 38EB5564  addi r7, r11, 0x5564
	ctx.r[7].s64 = ctx.r[11].s64 + 21860;
	// 8263A830: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A834: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8263A838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A83C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A848: 386AAE4C  addi r3, r10, -0x51b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20916;
	// 8263A84C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A870: 4BE2C5B1  bl 0x82466e20
	ctx.lr = 0x8263A874;
	sub_82466E20(ctx, base);
	// 8263A874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A888 size=108
    let mut pc: u32 = 0x8263A888;
    'dispatch: loop {
        match pc {
            0x8263A888 => {
    //   block [0x8263A888..0x8263A8F4)
	// 8263A888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A894: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A89C: 38EB557C  addi r7, r11, 0x557c
	ctx.r[7].s64 = ctx.r[11].s64 + 21884;
	// 8263A8A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A8A4: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8263A8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A8B8: 386AAE7C  addi r3, r10, -0x5184
	ctx.r[3].s64 = ctx.r[10].s64 + -20868;
	// 8263A8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A8E0: 4BE2C541  bl 0x82466e20
	ctx.lr = 0x8263A8E4;
	sub_82466E20(ctx, base);
	// 8263A8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A8F8 size=108
    let mut pc: u32 = 0x8263A8F8;
    'dispatch: loop {
        match pc {
            0x8263A8F8 => {
    //   block [0x8263A8F8..0x8263A964)
	// 8263A8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A904: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A90C: 38EB55B0  addi r7, r11, 0x55b0
	ctx.r[7].s64 = ctx.r[11].s64 + 21936;
	// 8263A910: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263A914: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8263A918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A91C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A928: 386AAEAC  addi r3, r10, -0x5154
	ctx.r[3].s64 = ctx.r[10].s64 + -20820;
	// 8263A92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A950: 4BE2C4D1  bl 0x82466e20
	ctx.lr = 0x8263A954;
	sub_82466E20(ctx, base);
	// 8263A954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A968 size=108
    let mut pc: u32 = 0x8263A968;
    'dispatch: loop {
        match pc {
            0x8263A968 => {
    //   block [0x8263A968..0x8263A9D4)
	// 8263A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A974: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A97C: 38EB5658  addi r7, r11, 0x5658
	ctx.r[7].s64 = ctx.r[11].s64 + 22104;
	// 8263A980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A984: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 8263A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A98C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A998: 386AAEDC  addi r3, r10, -0x5124
	ctx.r[3].s64 = ctx.r[10].s64 + -20772;
	// 8263A99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A9C0: 4BE2C461  bl 0x82466e20
	ctx.lr = 0x8263A9C4;
	sub_82466E20(ctx, base);
	// 8263A9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A9D8 size=108
    let mut pc: u32 = 0x8263A9D8;
    'dispatch: loop {
        match pc {
            0x8263A9D8 => {
    //   block [0x8263A9D8..0x8263AA44)
	// 8263A9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A9E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A9E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A9EC: 38EB5688  addi r7, r11, 0x5688
	ctx.r[7].s64 = ctx.r[11].s64 + 22152;
	// 8263A9F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A9F4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 8263A9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A9FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AA00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AA08: 386AAF0C  addi r3, r10, -0x50f4
	ctx.r[3].s64 = ctx.r[10].s64 + -20724;
	// 8263AA0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AA2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AA30: 4BE2C3F1  bl 0x82466e20
	ctx.lr = 0x8263AA34;
	sub_82466E20(ctx, base);
	// 8263AA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AA48 size=108
    let mut pc: u32 = 0x8263AA48;
    'dispatch: loop {
        match pc {
            0x8263AA48 => {
    //   block [0x8263AA48..0x8263AAB4)
	// 8263AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AA54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AA5C: 38EB56A0  addi r7, r11, 0x56a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22176;
	// 8263AA60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263AA64: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8263AA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AA6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AA70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AA78: 386AAF3C  addi r3, r10, -0x50c4
	ctx.r[3].s64 = ctx.r[10].s64 + -20676;
	// 8263AA7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AA9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AAA0: 4BE2C381  bl 0x82466e20
	ctx.lr = 0x8263AAA4;
	sub_82466E20(ctx, base);
	// 8263AAA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AAA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AAAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AAB8 size=112
    let mut pc: u32 = 0x8263AAB8;
    'dispatch: loop {
        match pc {
            0x8263AAB8 => {
    //   block [0x8263AAB8..0x8263AB28)
	// 8263AAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AAC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AAC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AACC: 38AAAD8C  addi r5, r10, -0x5274
	ctx.r[5].s64 = ctx.r[10].s64 + -21108;
	// 8263AAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AAD4: 390B56D0  addi r8, r11, 0x56d0
	ctx.r[8].s64 = ctx.r[11].s64 + 22224;
	// 8263AAD8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263AADC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8263AAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AAE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AAE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AAF0: 386AAF6C  addi r3, r10, -0x5094
	ctx.r[3].s64 = ctx.r[10].s64 + -20628;
	// 8263AAF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263AAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AB14: 4BE2C30D  bl 0x82466e20
	ctx.lr = 0x8263AB18;
	sub_82466E20(ctx, base);
	// 8263AB18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263AB28 size=24
    let mut pc: u32 = 0x8263AB28;
    'dispatch: loop {
        match pc {
            0x8263AB28 => {
    //   block [0x8263AB28..0x8263AB40)
	// 8263AB28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AB2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263AB30: 394AB410  addi r10, r10, -0x4bf0
	ctx.r[10].s64 = ctx.r[10].s64 + -19440;
	// 8263AB34: 816B55AC  lwz r11, 0x55ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21932 as u32) ) } as u64;
	// 8263AB38: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AB40 size=112
    let mut pc: u32 = 0x8263AB40;
    'dispatch: loop {
        match pc {
            0x8263AB40 => {
    //   block [0x8263AB40..0x8263ABB0)
	// 8263AB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AB4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263AB50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263AB54: 392A65B8  addi r9, r10, 0x65b8
	ctx.r[9].s64 = ctx.r[10].s64 + 26040;
	// 8263AB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AB5C: 390BB410  addi r8, r11, -0x4bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -19440;
	// 8263AB60: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263AB64: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8263AB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AB6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AB70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AB78: 386AAF9C  addi r3, r10, -0x5064
	ctx.r[3].s64 = ctx.r[10].s64 + -20580;
	// 8263AB7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263AB80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263AB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AB94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AB9C: 4BE2C285  bl 0x82466e20
	ctx.lr = 0x8263ABA0;
	sub_82466E20(ctx, base);
	// 8263ABA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ABA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ABA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ABAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ABB0 size=108
    let mut pc: u32 = 0x8263ABB0;
    'dispatch: loop {
        match pc {
            0x8263ABB0 => {
    //   block [0x8263ABB0..0x8263AC1C)
	// 8263ABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ABB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ABB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ABBC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ABC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ABC4: 38EB577C  addi r7, r11, 0x577c
	ctx.r[7].s64 = ctx.r[11].s64 + 22396;
	// 8263ABC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263ABCC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8263ABD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ABD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ABD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263ABDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263ABE0: 386AAFCC  addi r3, r10, -0x5034
	ctx.r[3].s64 = ctx.r[10].s64 + -20532;
	// 8263ABE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263ABE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263ABEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ABF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ABF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ABF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ABFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AC04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AC08: 4BE2C219  bl 0x82466e20
	ctx.lr = 0x8263AC0C;
	sub_82466E20(ctx, base);
	// 8263AC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AC20 size=116
    let mut pc: u32 = 0x8263AC20;
    'dispatch: loop {
        match pc {
            0x8263AC20 => {
    //   block [0x8263AC20..0x8263AC94)
	// 8263AC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AC2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AC30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263AC34: 390B57B0  addi r8, r11, 0x57b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22448;
	// 8263AC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AC3C: 392A65FC  addi r9, r10, 0x65fc
	ctx.r[9].s64 = ctx.r[10].s64 + 26108;
	// 8263AC40: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AC44: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8263AC48: 38AAAD8C  addi r5, r10, -0x5274
	ctx.r[5].s64 = ctx.r[10].s64 + -21108;
	// 8263AC4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AC54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AC64: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263AC68: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8263AC6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263AC70: 386BAFFC  addi r3, r11, -0x5004
	ctx.r[3].s64 = ctx.r[11].s64 + -20484;
	// 8263AC74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263AC78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AC80: 4BE2C1A1  bl 0x82466e20
	ctx.lr = 0x8263AC84;
	sub_82466E20(ctx, base);
	// 8263AC84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263AC98 size=24
    let mut pc: u32 = 0x8263AC98;
    'dispatch: loop {
        match pc {
            0x8263AC98 => {
    //   block [0x8263AC98..0x8263ACB0)
	// 8263AC98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AC9C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263ACA0: 394AB488  addi r10, r10, -0x4b78
	ctx.r[10].s64 = ctx.r[10].s64 + -19320;
	// 8263ACA4: 816B57AC  lwz r11, 0x57ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22444 as u32) ) } as u64;
	// 8263ACA8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ACB0 size=112
    let mut pc: u32 = 0x8263ACB0;
    'dispatch: loop {
        match pc {
            0x8263ACB0 => {
    //   block [0x8263ACB0..0x8263AD20)
	// 8263ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ACB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ACB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ACBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263ACC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263ACC4: 392A6638  addi r9, r10, 0x6638
	ctx.r[9].s64 = ctx.r[10].s64 + 26168;
	// 8263ACC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ACCC: 390BB488  addi r8, r11, -0x4b78
	ctx.r[8].s64 = ctx.r[11].s64 + -19320;
	// 8263ACD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263ACD4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8263ACD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ACDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ACE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263ACE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ACE8: 386AB02C  addi r3, r10, -0x4fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -20436;
	// 8263ACEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263ACF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263ACF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ACF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ACFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AD0C: 4BE2C115  bl 0x82466e20
	ctx.lr = 0x8263AD10;
	sub_82466E20(ctx, base);
	// 8263AD10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AD20 size=108
    let mut pc: u32 = 0x8263AD20;
    'dispatch: loop {
        match pc {
            0x8263AD20 => {
    //   block [0x8263AD20..0x8263AD8C)
	// 8263AD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AD2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AD30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AD34: 38EB5870  addi r7, r11, 0x5870
	ctx.r[7].s64 = ctx.r[11].s64 + 22640;
	// 8263AD38: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263AD3C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8263AD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AD44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AD48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AD50: 386AB05C  addi r3, r10, -0x4fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -20388;
	// 8263AD54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AD60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AD68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AD70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AD74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AD78: 4BE2C0A9  bl 0x82466e20
	ctx.lr = 0x8263AD7C;
	sub_82466E20(ctx, base);
	// 8263AD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AD90 size=108
    let mut pc: u32 = 0x8263AD90;
    'dispatch: loop {
        match pc {
            0x8263AD90 => {
    //   block [0x8263AD90..0x8263ADFC)
	// 8263AD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AD9C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ADA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ADA4: 38EB5888  addi r7, r11, 0x5888
	ctx.r[7].s64 = ctx.r[11].s64 + 22664;
	// 8263ADA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263ADAC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8263ADB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ADB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ADB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263ADBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263ADC0: 386AB08C  addi r3, r10, -0x4f74
	ctx.r[3].s64 = ctx.r[10].s64 + -20340;
	// 8263ADC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263ADC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263ADCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ADD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ADD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ADD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ADDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ADE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263ADE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263ADE8: 4BE2C039  bl 0x82466e20
	ctx.lr = 0x8263ADEC;
	sub_82466E20(ctx, base);
	// 8263ADEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ADF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ADF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ADF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263AE00 size=24
    let mut pc: u32 = 0x8263AE00;
    'dispatch: loop {
        match pc {
            0x8263AE00 => {
    //   block [0x8263AE00..0x8263AE18)
	// 8263AE00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AE04: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263AE08: 394AB4D0  addi r10, r10, -0x4b30
	ctx.r[10].s64 = ctx.r[10].s64 + -19248;
	// 8263AE0C: 816B58B8  lwz r11, 0x58b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22712 as u32) ) } as u64;
	// 8263AE10: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AE18 size=112
    let mut pc: u32 = 0x8263AE18;
    'dispatch: loop {
        match pc {
            0x8263AE18 => {
    //   block [0x8263AE18..0x8263AE88)
	// 8263AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AE24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263AE28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263AE2C: 392A6674  addi r9, r10, 0x6674
	ctx.r[9].s64 = ctx.r[10].s64 + 26228;
	// 8263AE30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AE34: 390BB4D0  addi r8, r11, -0x4b30
	ctx.r[8].s64 = ctx.r[11].s64 + -19248;
	// 8263AE38: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263AE3C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8263AE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AE44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AE48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AE50: 386AB0BC  addi r3, r10, -0x4f44
	ctx.r[3].s64 = ctx.r[10].s64 + -20292;
	// 8263AE54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263AE58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263AE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AE74: 4BE2BFAD  bl 0x82466e20
	ctx.lr = 0x8263AE78;
	sub_82466E20(ctx, base);
	// 8263AE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AE88 size=112
    let mut pc: u32 = 0x8263AE88;
    'dispatch: loop {
        match pc {
            0x8263AE88 => {
    //   block [0x8263AE88..0x8263AEF8)
	// 8263AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AE94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AE98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AE9C: 38AAAD8C  addi r5, r10, -0x5274
	ctx.r[5].s64 = ctx.r[10].s64 + -21108;
	// 8263AEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AEA4: 390B58BC  addi r8, r11, 0x58bc
	ctx.r[8].s64 = ctx.r[11].s64 + 22716;
	// 8263AEA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263AEAC: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 8263AEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AEB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AEC0: 386AB0EC  addi r3, r10, -0x4f14
	ctx.r[3].s64 = ctx.r[10].s64 + -20244;
	// 8263AEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263AEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AEE4: 4BE2BF3D  bl 0x82466e20
	ctx.lr = 0x8263AEE8;
	sub_82466E20(ctx, base);
	// 8263AEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AEF8 size=108
    let mut pc: u32 = 0x8263AEF8;
    'dispatch: loop {
        match pc {
            0x8263AEF8 => {
    //   block [0x8263AEF8..0x8263AF64)
	// 8263AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AF04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AF0C: 38EB58EC  addi r7, r11, 0x58ec
	ctx.r[7].s64 = ctx.r[11].s64 + 22764;
	// 8263AF10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263AF14: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8263AF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AF1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AF28: 386AB11C  addi r3, r10, -0x4ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -20196;
	// 8263AF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AF50: 4BE2BED1  bl 0x82466e20
	ctx.lr = 0x8263AF54;
	sub_82466E20(ctx, base);
	// 8263AF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AF68 size=108
    let mut pc: u32 = 0x8263AF68;
    'dispatch: loop {
        match pc {
            0x8263AF68 => {
    //   block [0x8263AF68..0x8263AFD4)
	// 8263AF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AF74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AF7C: 38EB5920  addi r7, r11, 0x5920
	ctx.r[7].s64 = ctx.r[11].s64 + 22816;
	// 8263AF80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263AF84: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8263AF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AF8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AF90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AF98: 386AB14C  addi r3, r10, -0x4eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -20148;
	// 8263AF9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AFA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AFC0: 4BE2BE61  bl 0x82466e20
	ctx.lr = 0x8263AFC4;
	sub_82466E20(ctx, base);
	// 8263AFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AFD8 size=108
    let mut pc: u32 = 0x8263AFD8;
    'dispatch: loop {
        match pc {
            0x8263AFD8 => {
    //   block [0x8263AFD8..0x8263B044)
	// 8263AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AFE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AFE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AFEC: 38EB5980  addi r7, r11, 0x5980
	ctx.r[7].s64 = ctx.r[11].s64 + 22912;
	// 8263AFF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263AFF4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8263AFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AFFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B008: 386AB17C  addi r3, r10, -0x4e84
	ctx.r[3].s64 = ctx.r[10].s64 + -20100;
	// 8263B00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B030: 4BE2BDF1  bl 0x82466e20
	ctx.lr = 0x8263B034;
	sub_82466E20(ctx, base);
	// 8263B034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B048 size=108
    let mut pc: u32 = 0x8263B048;
    'dispatch: loop {
        match pc {
            0x8263B048 => {
    //   block [0x8263B048..0x8263B0B4)
	// 8263B048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B054: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B05C: 38EB59B0  addi r7, r11, 0x59b0
	ctx.r[7].s64 = ctx.r[11].s64 + 22960;
	// 8263B060: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8263B064: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8263B068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B06C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B078: 386AB1AC  addi r3, r10, -0x4e54
	ctx.r[3].s64 = ctx.r[10].s64 + -20052;
	// 8263B07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B0A0: 4BE2BD81  bl 0x82466e20
	ctx.lr = 0x8263B0A4;
	sub_82466E20(ctx, base);
	// 8263B0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B0B8 size=108
    let mut pc: u32 = 0x8263B0B8;
    'dispatch: loop {
        match pc {
            0x8263B0B8 => {
    //   block [0x8263B0B8..0x8263B124)
	// 8263B0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B0C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B0C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B0CC: 38EB5AD0  addi r7, r11, 0x5ad0
	ctx.r[7].s64 = ctx.r[11].s64 + 23248;
	// 8263B0D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B0D4: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 8263B0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B0DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B0E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B0E8: 386AB1DC  addi r3, r10, -0x4e24
	ctx.r[3].s64 = ctx.r[10].s64 + -20004;
	// 8263B0EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B110: 4BE2BD11  bl 0x82466e20
	ctx.lr = 0x8263B114;
	sub_82466E20(ctx, base);
	// 8263B114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B128 size=108
    let mut pc: u32 = 0x8263B128;
    'dispatch: loop {
        match pc {
            0x8263B128 => {
    //   block [0x8263B128..0x8263B194)
	// 8263B128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B134: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B13C: 38EB5AE8  addi r7, r11, 0x5ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 23272;
	// 8263B140: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B144: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 8263B148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B14C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B158: 386AB20C  addi r3, r10, -0x4df4
	ctx.r[3].s64 = ctx.r[10].s64 + -19956;
	// 8263B15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B180: 4BE2BCA1  bl 0x82466e20
	ctx.lr = 0x8263B184;
	sub_82466E20(ctx, base);
	// 8263B184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B198 size=108
    let mut pc: u32 = 0x8263B198;
    'dispatch: loop {
        match pc {
            0x8263B198 => {
    //   block [0x8263B198..0x8263B204)
	// 8263B198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B1A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B1A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B1AC: 38EB5B00  addi r7, r11, 0x5b00
	ctx.r[7].s64 = ctx.r[11].s64 + 23296;
	// 8263B1B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B1B4: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 8263B1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B1BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B1C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B1C8: 386AB23C  addi r3, r10, -0x4dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -19908;
	// 8263B1CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B1EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B1F0: 4BE2BC31  bl 0x82466e20
	ctx.lr = 0x8263B1F4;
	sub_82466E20(ctx, base);
	// 8263B1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B208 size=108
    let mut pc: u32 = 0x8263B208;
    'dispatch: loop {
        match pc {
            0x8263B208 => {
    //   block [0x8263B208..0x8263B274)
	// 8263B208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B214: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B21C: 38EB5B18  addi r7, r11, 0x5b18
	ctx.r[7].s64 = ctx.r[11].s64 + 23320;
	// 8263B220: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B224: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 8263B228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B22C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B230: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B238: 386AB26C  addi r3, r10, -0x4d94
	ctx.r[3].s64 = ctx.r[10].s64 + -19860;
	// 8263B23C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B25C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B260: 4BE2BBC1  bl 0x82466e20
	ctx.lr = 0x8263B264;
	sub_82466E20(ctx, base);
	// 8263B264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B278 size=108
    let mut pc: u32 = 0x8263B278;
    'dispatch: loop {
        match pc {
            0x8263B278 => {
    //   block [0x8263B278..0x8263B2E4)
	// 8263B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B284: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B28C: 38EB5B30  addi r7, r11, 0x5b30
	ctx.r[7].s64 = ctx.r[11].s64 + 23344;
	// 8263B290: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B294: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 8263B298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B29C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B2A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B2A8: 386AB29C  addi r3, r10, -0x4d64
	ctx.r[3].s64 = ctx.r[10].s64 + -19812;
	// 8263B2AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B2B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B2CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B2D0: 4BE2BB51  bl 0x82466e20
	ctx.lr = 0x8263B2D4;
	sub_82466E20(ctx, base);
	// 8263B2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B2E8 size=108
    let mut pc: u32 = 0x8263B2E8;
    'dispatch: loop {
        match pc {
            0x8263B2E8 => {
    //   block [0x8263B2E8..0x8263B354)
	// 8263B2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B2F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B2F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B2FC: 38EB5B48  addi r7, r11, 0x5b48
	ctx.r[7].s64 = ctx.r[11].s64 + 23368;
	// 8263B300: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B304: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 8263B308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B30C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B318: 386AB2CC  addi r3, r10, -0x4d34
	ctx.r[3].s64 = ctx.r[10].s64 + -19764;
	// 8263B31C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B340: 4BE2BAE1  bl 0x82466e20
	ctx.lr = 0x8263B344;
	sub_82466E20(ctx, base);
	// 8263B344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B358 size=108
    let mut pc: u32 = 0x8263B358;
    'dispatch: loop {
        match pc {
            0x8263B358 => {
    //   block [0x8263B358..0x8263B3C4)
	// 8263B358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B364: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B36C: 38EB5B60  addi r7, r11, 0x5b60
	ctx.r[7].s64 = ctx.r[11].s64 + 23392;
	// 8263B370: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263B374: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8263B378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B37C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B388: 386AB2FC  addi r3, r10, -0x4d04
	ctx.r[3].s64 = ctx.r[10].s64 + -19716;
	// 8263B38C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B3AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B3B0: 4BE2BA71  bl 0x82466e20
	ctx.lr = 0x8263B3B4;
	sub_82466E20(ctx, base);
	// 8263B3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B3C8 size=108
    let mut pc: u32 = 0x8263B3C8;
    'dispatch: loop {
        match pc {
            0x8263B3C8 => {
    //   block [0x8263B3C8..0x8263B434)
	// 8263B3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B3D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B3DC: 38EB5BF0  addi r7, r11, 0x5bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 23536;
	// 8263B3E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263B3E4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8263B3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B3EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B3F8: 386AB32C  addi r3, r10, -0x4cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -19668;
	// 8263B3FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B420: 4BE2BA01  bl 0x82466e20
	ctx.lr = 0x8263B424;
	sub_82466E20(ctx, base);
	// 8263B424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B438 size=108
    let mut pc: u32 = 0x8263B438;
    'dispatch: loop {
        match pc {
            0x8263B438 => {
    //   block [0x8263B438..0x8263B4A4)
	// 8263B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B444: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B44C: 38EB5CB0  addi r7, r11, 0x5cb0
	ctx.r[7].s64 = ctx.r[11].s64 + 23728;
	// 8263B450: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8263B454: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8263B458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B45C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B468: 386AB35C  addi r3, r10, -0x4ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -19620;
	// 8263B46C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B48C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B490: 4BE2B991  bl 0x82466e20
	ctx.lr = 0x8263B494;
	sub_82466E20(ctx, base);
	// 8263B494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B49C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B4A8 size=108
    let mut pc: u32 = 0x8263B4A8;
    'dispatch: loop {
        match pc {
            0x8263B4A8 => {
    //   block [0x8263B4A8..0x8263B514)
	// 8263B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B4B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B4B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B4BC: 38EB5D88  addi r7, r11, 0x5d88
	ctx.r[7].s64 = ctx.r[11].s64 + 23944;
	// 8263B4C0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263B4C4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8263B4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B4CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B4D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B4D8: 386AB38C  addi r3, r10, -0x4c74
	ctx.r[3].s64 = ctx.r[10].s64 + -19572;
	// 8263B4DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B4FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B500: 4BE2B921  bl 0x82466e20
	ctx.lr = 0x8263B504;
	sub_82466E20(ctx, base);
	// 8263B504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B518 size=108
    let mut pc: u32 = 0x8263B518;
    'dispatch: loop {
        match pc {
            0x8263B518 => {
    //   block [0x8263B518..0x8263B584)
	// 8263B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B524: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B52C: 38EB5E48  addi r7, r11, 0x5e48
	ctx.r[7].s64 = ctx.r[11].s64 + 24136;
	// 8263B530: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263B534: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8263B538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B53C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B548: 386AB3BC  addi r3, r10, -0x4c44
	ctx.r[3].s64 = ctx.r[10].s64 + -19524;
	// 8263B54C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B570: 4BE2B8B1  bl 0x82466e20
	ctx.lr = 0x8263B574;
	sub_82466E20(ctx, base);
	// 8263B574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B588 size=112
    let mut pc: u32 = 0x8263B588;
    'dispatch: loop {
        match pc {
            0x8263B588 => {
    //   block [0x8263B588..0x8263B5F8)
	// 8263B588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B594: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263B598: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8263B59C: 38EA5EF0  addi r7, r10, 0x5ef0
	ctx.r[7].s64 = ctx.r[10].s64 + 24304;
	// 8263B5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B5A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263B5A8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8263B5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B5B0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B5B4: 396B6688  addi r11, r11, 0x6688
	ctx.r[11].s64 = ctx.r[11].s64 + 26248;
	// 8263B5B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B5BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B5C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B5C4: 386AB3EC  addi r3, r10, -0x4c14
	ctx.r[3].s64 = ctx.r[10].s64 + -19476;
	// 8263B5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B5CC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263B5D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B5D4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263B5D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B5DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B5E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B5E4: 4BE2B83D  bl 0x82466e20
	ctx.lr = 0x8263B5E8;
	sub_82466E20(ctx, base);
	// 8263B5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B5F8 size=108
    let mut pc: u32 = 0x8263B5F8;
    'dispatch: loop {
        match pc {
            0x8263B5F8 => {
    //   block [0x8263B5F8..0x8263B664)
	// 8263B5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B604: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B60C: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 8263B610: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263B614: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8263B618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B61C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B628: 386AB41C  addi r3, r10, -0x4be4
	ctx.r[3].s64 = ctx.r[10].s64 + -19428;
	// 8263B62C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B650: 4BE2B7D1  bl 0x82466e20
	ctx.lr = 0x8263B654;
	sub_82466E20(ctx, base);
	// 8263B654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B668 size=108
    let mut pc: u32 = 0x8263B668;
    'dispatch: loop {
        match pc {
            0x8263B668 => {
    //   block [0x8263B668..0x8263B6D4)
	// 8263B668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B674: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B67C: 38EB6070  addi r7, r11, 0x6070
	ctx.r[7].s64 = ctx.r[11].s64 + 24688;
	// 8263B680: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8263B684: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8263B688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B68C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B698: 386AB44C  addi r3, r10, -0x4bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -19380;
	// 8263B69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B6C0: 4BE2B761  bl 0x82466e20
	ctx.lr = 0x8263B6C4;
	sub_82466E20(ctx, base);
	// 8263B6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B6D8 size=108
    let mut pc: u32 = 0x8263B6D8;
    'dispatch: loop {
        match pc {
            0x8263B6D8 => {
    //   block [0x8263B6D8..0x8263B744)
	// 8263B6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B6E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B6E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B6EC: 38EB6178  addi r7, r11, 0x6178
	ctx.r[7].s64 = ctx.r[11].s64 + 24952;
	// 8263B6F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8263B6F4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8263B6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B6FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B708: 386AB47C  addi r3, r10, -0x4b84
	ctx.r[3].s64 = ctx.r[10].s64 + -19332;
	// 8263B70C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B730: 4BE2B6F1  bl 0x82466e20
	ctx.lr = 0x8263B734;
	sub_82466E20(ctx, base);
	// 8263B734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B73C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B748 size=108
    let mut pc: u32 = 0x8263B748;
    'dispatch: loop {
        match pc {
            0x8263B748 => {
    //   block [0x8263B748..0x8263B7B4)
	// 8263B748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B754: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B75C: 38EB6250  addi r7, r11, 0x6250
	ctx.r[7].s64 = ctx.r[11].s64 + 25168;
	// 8263B760: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263B764: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8263B768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B76C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B778: 386AB4AC  addi r3, r10, -0x4b54
	ctx.r[3].s64 = ctx.r[10].s64 + -19284;
	// 8263B77C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B7A0: 4BE2B681  bl 0x82466e20
	ctx.lr = 0x8263B7A4;
	sub_82466E20(ctx, base);
	// 8263B7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B7B8 size=108
    let mut pc: u32 = 0x8263B7B8;
    'dispatch: loop {
        match pc {
            0x8263B7B8 => {
    //   block [0x8263B7B8..0x8263B824)
	// 8263B7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B7C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B7C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B7CC: 38EB6298  addi r7, r11, 0x6298
	ctx.r[7].s64 = ctx.r[11].s64 + 25240;
	// 8263B7D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B7D4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8263B7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B7DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B7E8: 386AB4DC  addi r3, r10, -0x4b24
	ctx.r[3].s64 = ctx.r[10].s64 + -19236;
	// 8263B7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B810: 4BE2B611  bl 0x82466e20
	ctx.lr = 0x8263B814;
	sub_82466E20(ctx, base);
	// 8263B814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B828 size=112
    let mut pc: u32 = 0x8263B828;
    'dispatch: loop {
        match pc {
            0x8263B828 => {
    //   block [0x8263B828..0x8263B898)
	// 8263B828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B834: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B838: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B83C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263B840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B844: 390B62B0  addi r8, r11, 0x62b0
	ctx.r[8].s64 = ctx.r[11].s64 + 25264;
	// 8263B848: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263B84C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8263B850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B854: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263B85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B860: 386AB50C  addi r3, r10, -0x4af4
	ctx.r[3].s64 = ctx.r[10].s64 + -19188;
	// 8263B864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263B868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B884: 4BE2B59D  bl 0x82466e20
	ctx.lr = 0x8263B888;
	sub_82466E20(ctx, base);
	// 8263B888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B898 size=112
    let mut pc: u32 = 0x8263B898;
    'dispatch: loop {
        match pc {
            0x8263B898 => {
    //   block [0x8263B898..0x8263B908)
	// 8263B898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B8A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B8A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B8AC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263B8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B8B4: 390B62F8  addi r8, r11, 0x62f8
	ctx.r[8].s64 = ctx.r[11].s64 + 25336;
	// 8263B8B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263B8BC: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 8263B8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B8C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263B8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B8D0: 386AB53C  addi r3, r10, -0x4ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -19140;
	// 8263B8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263B8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B8F4: 4BE2B52D  bl 0x82466e20
	ctx.lr = 0x8263B8F8;
	sub_82466E20(ctx, base);
	// 8263B8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B908 size=108
    let mut pc: u32 = 0x8263B908;
    'dispatch: loop {
        match pc {
            0x8263B908 => {
    //   block [0x8263B908..0x8263B974)
	// 8263B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B914: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B91C: 38EB6340  addi r7, r11, 0x6340
	ctx.r[7].s64 = ctx.r[11].s64 + 25408;
	// 8263B920: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B924: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 8263B928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B92C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B938: 386AB56C  addi r3, r10, -0x4a94
	ctx.r[3].s64 = ctx.r[10].s64 + -19092;
	// 8263B93C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B95C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B960: 4BE2B4C1  bl 0x82466e20
	ctx.lr = 0x8263B964;
	sub_82466E20(ctx, base);
	// 8263B964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263B978 size=24
    let mut pc: u32 = 0x8263B978;
    'dispatch: loop {
        match pc {
            0x8263B978 => {
    //   block [0x8263B978..0x8263B990)
	// 8263B978: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B97C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263B980: 394AB548  addi r10, r10, -0x4ab8
	ctx.r[10].s64 = ctx.r[10].s64 + -19128;
	// 8263B984: 816B591C  lwz r11, 0x591c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22812 as u32) ) } as u64;
	// 8263B988: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263B98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B990 size=112
    let mut pc: u32 = 0x8263B990;
    'dispatch: loop {
        match pc {
            0x8263B990 => {
    //   block [0x8263B990..0x8263BA00)
	// 8263B990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B99C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B9A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263B9A4: 38AAB77C  addi r5, r10, -0x4884
	ctx.r[5].s64 = ctx.r[10].s64 + -18564;
	// 8263B9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B9AC: 390BB548  addi r8, r11, -0x4ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -19128;
	// 8263B9B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8263B9B4: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 8263B9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B9BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263B9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B9C8: 386AB59C  addi r3, r10, -0x4a64
	ctx.r[3].s64 = ctx.r[10].s64 + -19044;
	// 8263B9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263B9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B9EC: 4BE2B435  bl 0x82466e20
	ctx.lr = 0x8263B9F0;
	sub_82466E20(ctx, base);
	// 8263B9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BA00 size=108
    let mut pc: u32 = 0x8263BA00;
    'dispatch: loop {
        match pc {
            0x8263BA00 => {
    //   block [0x8263BA00..0x8263BA6C)
	// 8263BA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BA0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BA14: 38EB6358  addi r7, r11, 0x6358
	ctx.r[7].s64 = ctx.r[11].s64 + 25432;
	// 8263BA18: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263BA1C: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 8263BA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BA24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BA28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BA30: 386AB5CC  addi r3, r10, -0x4a34
	ctx.r[3].s64 = ctx.r[10].s64 + -18996;
	// 8263BA34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BA58: 4BE2B3C9  bl 0x82466e20
	ctx.lr = 0x8263BA5C;
	sub_82466E20(ctx, base);
	// 8263BA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BA70 size=112
    let mut pc: u32 = 0x8263BA70;
    'dispatch: loop {
        match pc {
            0x8263BA70 => {
    //   block [0x8263BA70..0x8263BAE0)
	// 8263BA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BA7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BA80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BA84: 38AAB77C  addi r5, r10, -0x4884
	ctx.r[5].s64 = ctx.r[10].s64 + -18564;
	// 8263BA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BA8C: 390B63B8  addi r8, r11, 0x63b8
	ctx.r[8].s64 = ctx.r[11].s64 + 25528;
	// 8263BA90: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8263BA94: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 8263BA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BA9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BAA8: 386AB5FC  addi r3, r10, -0x4a04
	ctx.r[3].s64 = ctx.r[10].s64 + -18948;
	// 8263BAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BACC: 4BE2B355  bl 0x82466e20
	ctx.lr = 0x8263BAD0;
	sub_82466E20(ctx, base);
	// 8263BAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BAE0 size=108
    let mut pc: u32 = 0x8263BAE0;
    'dispatch: loop {
        match pc {
            0x8263BAE0 => {
    //   block [0x8263BAE0..0x8263BB4C)
	// 8263BAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BAEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BAF4: 38EB6478  addi r7, r11, 0x6478
	ctx.r[7].s64 = ctx.r[11].s64 + 25720;
	// 8263BAF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263BAFC: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 8263BB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BB04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BB08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BB10: 386AB62C  addi r3, r10, -0x49d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18900;
	// 8263BB14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BB38: 4BE2B2E9  bl 0x82466e20
	ctx.lr = 0x8263BB3C;
	sub_82466E20(ctx, base);
	// 8263BB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BB50 size=108
    let mut pc: u32 = 0x8263BB50;
    'dispatch: loop {
        match pc {
            0x8263BB50 => {
    //   block [0x8263BB50..0x8263BBBC)
	// 8263BB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BB5C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BB64: 38EB64D8  addi r7, r11, 0x64d8
	ctx.r[7].s64 = ctx.r[11].s64 + 25816;
	// 8263BB68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263BB6C: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 8263BB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BB74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BB78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BB80: 386AB65C  addi r3, r10, -0x49a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18852;
	// 8263BB84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BBA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BBA8: 4BE2B279  bl 0x82466e20
	ctx.lr = 0x8263BBAC;
	sub_82466E20(ctx, base);
	// 8263BBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BBC0 size=108
    let mut pc: u32 = 0x8263BBC0;
    'dispatch: loop {
        match pc {
            0x8263BBC0 => {
    //   block [0x8263BBC0..0x8263BC2C)
	// 8263BBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BBCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BBD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BBD4: 38EB6520  addi r7, r11, 0x6520
	ctx.r[7].s64 = ctx.r[11].s64 + 25888;
	// 8263BBD8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263BBDC: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 8263BBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BBE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BBE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BBF0: 386AB68C  addi r3, r10, -0x4974
	ctx.r[3].s64 = ctx.r[10].s64 + -18804;
	// 8263BBF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BC14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BC18: 4BE2B209  bl 0x82466e20
	ctx.lr = 0x8263BC1C;
	sub_82466E20(ctx, base);
	// 8263BC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BC30 size=112
    let mut pc: u32 = 0x8263BC30;
    'dispatch: loop {
        match pc {
            0x8263BC30 => {
    //   block [0x8263BC30..0x8263BCA0)
	// 8263BC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BC3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BC40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BC44: 38AAB68C  addi r5, r10, -0x4974
	ctx.r[5].s64 = ctx.r[10].s64 + -18804;
	// 8263BC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BC4C: 390B6568  addi r8, r11, 0x6568
	ctx.r[8].s64 = ctx.r[11].s64 + 25960;
	// 8263BC50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263BC54: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 8263BC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BC5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BC68: 386AB6BC  addi r3, r10, -0x4944
	ctx.r[3].s64 = ctx.r[10].s64 + -18756;
	// 8263BC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BC8C: 4BE2B195  bl 0x82466e20
	ctx.lr = 0x8263BC90;
	sub_82466E20(ctx, base);
	// 8263BC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BCA0 size=108
    let mut pc: u32 = 0x8263BCA0;
    'dispatch: loop {
        match pc {
            0x8263BCA0 => {
    //   block [0x8263BCA0..0x8263BD0C)
	// 8263BCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BCAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BCB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BCB4: 38EB6598  addi r7, r11, 0x6598
	ctx.r[7].s64 = ctx.r[11].s64 + 26008;
	// 8263BCB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263BCBC: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 8263BCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BCC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BCC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BCCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BCD0: 386AB6EC  addi r3, r10, -0x4914
	ctx.r[3].s64 = ctx.r[10].s64 + -18708;
	// 8263BCD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BCD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BCF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BCF8: 4BE2B129  bl 0x82466e20
	ctx.lr = 0x8263BCFC;
	sub_82466E20(ctx, base);
	// 8263BCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BD10 size=108
    let mut pc: u32 = 0x8263BD10;
    'dispatch: loop {
        match pc {
            0x8263BD10 => {
    //   block [0x8263BD10..0x8263BD7C)
	// 8263BD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BD1C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BD24: 38EB65E0  addi r7, r11, 0x65e0
	ctx.r[7].s64 = ctx.r[11].s64 + 26080;
	// 8263BD28: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263BD2C: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 8263BD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BD34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BD38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BD3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BD40: 386AB71C  addi r3, r10, -0x48e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18660;
	// 8263BD44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BD64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BD68: 4BE2B0B9  bl 0x82466e20
	ctx.lr = 0x8263BD6C;
	sub_82466E20(ctx, base);
	// 8263BD6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BD80 size=108
    let mut pc: u32 = 0x8263BD80;
    'dispatch: loop {
        match pc {
            0x8263BD80 => {
    //   block [0x8263BD80..0x8263BDEC)
	// 8263BD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BD8C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BD94: 38EB66A0  addi r7, r11, 0x66a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26272;
	// 8263BD98: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8263BD9C: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 8263BDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BDA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BDA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BDB0: 386AB74C  addi r3, r10, -0x48b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18612;
	// 8263BDB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BDD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BDD8: 4BE2B049  bl 0x82466e20
	ctx.lr = 0x8263BDDC;
	sub_82466E20(ctx, base);
	// 8263BDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BDF0 size=112
    let mut pc: u32 = 0x8263BDF0;
    'dispatch: loop {
        match pc {
            0x8263BDF0 => {
    //   block [0x8263BDF0..0x8263BE60)
	// 8263BDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BDFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BE00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BE04: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263BE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BE0C: 390B6820  addi r8, r11, 0x6820
	ctx.r[8].s64 = ctx.r[11].s64 + 26656;
	// 8263BE10: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263BE14: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 8263BE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BE1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BE20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BE28: 386AB77C  addi r3, r10, -0x4884
	ctx.r[3].s64 = ctx.r[10].s64 + -18564;
	// 8263BE2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BE4C: 4BE2AFD5  bl 0x82466e20
	ctx.lr = 0x8263BE50;
	sub_82466E20(ctx, base);
	// 8263BE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BE60 size=108
    let mut pc: u32 = 0x8263BE60;
    'dispatch: loop {
        match pc {
            0x8263BE60 => {
    //   block [0x8263BE60..0x8263BECC)
	// 8263BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BE6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BE74: 38EB6880  addi r7, r11, 0x6880
	ctx.r[7].s64 = ctx.r[11].s64 + 26752;
	// 8263BE78: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263BE7C: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 8263BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BE84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BE88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BE90: 386AB7AC  addi r3, r10, -0x4854
	ctx.r[3].s64 = ctx.r[10].s64 + -18516;
	// 8263BE94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BEB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BEB8: 4BE2AF69  bl 0x82466e20
	ctx.lr = 0x8263BEBC;
	sub_82466E20(ctx, base);
	// 8263BEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BED0 size=112
    let mut pc: u32 = 0x8263BED0;
    'dispatch: loop {
        match pc {
            0x8263BED0 => {
    //   block [0x8263BED0..0x8263BF40)
	// 8263BED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BEE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BEE4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263BEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BEEC: 390B68F8  addi r8, r11, 0x68f8
	ctx.r[8].s64 = ctx.r[11].s64 + 26872;
	// 8263BEF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263BEF4: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 8263BEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BEFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BF08: 386AB7DC  addi r3, r10, -0x4824
	ctx.r[3].s64 = ctx.r[10].s64 + -18468;
	// 8263BF0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BF2C: 4BE2AEF5  bl 0x82466e20
	ctx.lr = 0x8263BF30;
	sub_82466E20(ctx, base);
	// 8263BF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BF40 size=108
    let mut pc: u32 = 0x8263BF40;
    'dispatch: loop {
        match pc {
            0x8263BF40 => {
    //   block [0x8263BF40..0x8263BFAC)
	// 8263BF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BF4C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BF54: 38EB6940  addi r7, r11, 0x6940
	ctx.r[7].s64 = ctx.r[11].s64 + 26944;
	// 8263BF58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263BF5C: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 8263BF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BF64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BF70: 386AB80C  addi r3, r10, -0x47f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18420;
	// 8263BF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BF98: 4BE2AE89  bl 0x82466e20
	ctx.lr = 0x8263BF9C;
	sub_82466E20(ctx, base);
	// 8263BF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BFB0 size=108
    let mut pc: u32 = 0x8263BFB0;
    'dispatch: loop {
        match pc {
            0x8263BFB0 => {
    //   block [0x8263BFB0..0x8263C01C)
	// 8263BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BFBC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BFC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BFC4: 38EB69A0  addi r7, r11, 0x69a0
	ctx.r[7].s64 = ctx.r[11].s64 + 27040;
	// 8263BFC8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263BFCC: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 8263BFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BFD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BFE0: 386AB83C  addi r3, r10, -0x47c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18372;
	// 8263BFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C008: 4BE2AE19  bl 0x82466e20
	ctx.lr = 0x8263C00C;
	sub_82466E20(ctx, base);
	// 8263C00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C020 size=108
    let mut pc: u32 = 0x8263C020;
    'dispatch: loop {
        match pc {
            0x8263C020 => {
    //   block [0x8263C020..0x8263C08C)
	// 8263C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C02C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C034: 38EB6A60  addi r7, r11, 0x6a60
	ctx.r[7].s64 = ctx.r[11].s64 + 27232;
	// 8263C038: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263C03C: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 8263C040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C050: 386AB86C  addi r3, r10, -0x4794
	ctx.r[3].s64 = ctx.r[10].s64 + -18324;
	// 8263C054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C078: 4BE2ADA9  bl 0x82466e20
	ctx.lr = 0x8263C07C;
	sub_82466E20(ctx, base);
	// 8263C07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C090 size=108
    let mut pc: u32 = 0x8263C090;
    'dispatch: loop {
        match pc {
            0x8263C090 => {
    //   block [0x8263C090..0x8263C0FC)
	// 8263C090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C09C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C0A4: 38EB6AF0  addi r7, r11, 0x6af0
	ctx.r[7].s64 = ctx.r[11].s64 + 27376;
	// 8263C0A8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8263C0AC: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 8263C0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C0B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C0B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C0BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C0C0: 386AB89C  addi r3, r10, -0x4764
	ctx.r[3].s64 = ctx.r[10].s64 + -18276;
	// 8263C0C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C0E8: 4BE2AD39  bl 0x82466e20
	ctx.lr = 0x8263C0EC;
	sub_82466E20(ctx, base);
	// 8263C0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C100 size=108
    let mut pc: u32 = 0x8263C100;
    'dispatch: loop {
        match pc {
            0x8263C100 => {
    //   block [0x8263C100..0x8263C16C)
	// 8263C100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C10C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C114: 38EB6C28  addi r7, r11, 0x6c28
	ctx.r[7].s64 = ctx.r[11].s64 + 27688;
	// 8263C118: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263C11C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8263C120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C130: 386AB8CC  addi r3, r10, -0x4734
	ctx.r[3].s64 = ctx.r[10].s64 + -18228;
	// 8263C134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C158: 4BE2ACC9  bl 0x82466e20
	ctx.lr = 0x8263C15C;
	sub_82466E20(ctx, base);
	// 8263C15C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C170 size=116
    let mut pc: u32 = 0x8263C170;
    'dispatch: loop {
        match pc {
            0x8263C170 => {
    //   block [0x8263C170..0x8263C1E4)
	// 8263C170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C17C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C180: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C184: 390B6C90  addi r8, r11, 0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + 27792;
	// 8263C188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C18C: 392A673C  addi r9, r10, 0x673c
	ctx.r[9].s64 = ctx.r[10].s64 + 26428;
	// 8263C190: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C194: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263C198: 38AAB8CC  addi r5, r10, -0x4734
	ctx.r[5].s64 = ctx.r[10].s64 + -18228;
	// 8263C19C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C1B4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263C1B8: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8263C1BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263C1C0: 386BB8FC  addi r3, r11, -0x4704
	ctx.r[3].s64 = ctx.r[11].s64 + -18180;
	// 8263C1C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C1C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C1D0: 4BE2AC51  bl 0x82466e20
	ctx.lr = 0x8263C1D4;
	sub_82466E20(ctx, base);
	// 8263C1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C1E8 size=96
    let mut pc: u32 = 0x8263C1E8;
    'dispatch: loop {
        match pc {
            0x8263C1E8 => {
    //   block [0x8263C1E8..0x8263C248)
	// 8263C1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C1F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C1FC: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8263C200: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C208: 386AB92C  addi r3, r10, -0x46d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18132;
	// 8263C20C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C214: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C234: 4BE2ABED  bl 0x82466e20
	ctx.lr = 0x8263C238;
	sub_82466E20(ctx, base);
	// 8263C238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C248 size=112
    let mut pc: u32 = 0x8263C248;
    'dispatch: loop {
        match pc {
            0x8263C248 => {
    //   block [0x8263C248..0x8263C2B8)
	// 8263C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C258: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C25C: 38AAD93C  addi r5, r10, -0x26c4
	ctx.r[5].s64 = ctx.r[10].s64 + -9924;
	// 8263C260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C264: 390B6D08  addi r8, r11, 0x6d08
	ctx.r[8].s64 = ctx.r[11].s64 + 27912;
	// 8263C268: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263C26C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8263C270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C280: 386AB95C  addi r3, r10, -0x46a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18084;
	// 8263C284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C2A4: 4BE2AB7D  bl 0x82466e20
	ctx.lr = 0x8263C2A8;
	sub_82466E20(ctx, base);
	// 8263C2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C2B8 size=96
    let mut pc: u32 = 0x8263C2B8;
    'dispatch: loop {
        match pc {
            0x8263C2B8 => {
    //   block [0x8263C2B8..0x8263C318)
	// 8263C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C2C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C2CC: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8263C2D0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C2D8: 386AB98C  addi r3, r10, -0x4674
	ctx.r[3].s64 = ctx.r[10].s64 + -18036;
	// 8263C2DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C2E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C2F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C304: 4BE2AB1D  bl 0x82466e20
	ctx.lr = 0x8263C308;
	sub_82466E20(ctx, base);
	// 8263C308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263C318 size=24
    let mut pc: u32 = 0x8263C318;
    'dispatch: loop {
        match pc {
            0x8263C318 => {
    //   block [0x8263C318..0x8263C330)
	// 8263C318: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C31C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263C320: 394AB608  addi r10, r10, -0x49f8
	ctx.r[10].s64 = ctx.r[10].s64 + -18936;
	// 8263C324: 816B6C8C  lwz r11, 0x6c8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27788 as u32) ) } as u64;
	// 8263C328: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C330 size=116
    let mut pc: u32 = 0x8263C330;
    'dispatch: loop {
        match pc {
            0x8263C330 => {
    //   block [0x8263C330..0x8263C3A4)
	// 8263C330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C33C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263C340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C344: 390BB608  addi r8, r11, -0x49f8
	ctx.r[8].s64 = ctx.r[11].s64 + -18936;
	// 8263C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C34C: 392A6778  addi r9, r10, 0x6778
	ctx.r[9].s64 = ctx.r[10].s64 + 26488;
	// 8263C350: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C354: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263C358: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263C35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C364: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C36C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263C370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C374: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263C378: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8263C37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263C380: 386BB9BC  addi r3, r11, -0x4644
	ctx.r[3].s64 = ctx.r[11].s64 + -17988;
	// 8263C384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C390: 4BE2AA91  bl 0x82466e20
	ctx.lr = 0x8263C394;
	sub_82466E20(ctx, base);
	// 8263C394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C3A8 size=104
    let mut pc: u32 = 0x8263C3A8;
    'dispatch: loop {
        match pc {
            0x8263C3A8 => {
    //   block [0x8263C3A8..0x8263C410)
	// 8263C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C3B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C3BC: 392A67A4  addi r9, r10, 0x67a4
	ctx.r[9].s64 = ctx.r[10].s64 + 26532;
	// 8263C3C0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C3C8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263C3CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C3DC: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 8263C3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C3E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C3F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C3F4: 386AB9EC  addi r3, r10, -0x4614
	ctx.r[3].s64 = ctx.r[10].s64 + -17940;
	// 8263C3F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C3FC: 4BE2AA25  bl 0x82466e20
	ctx.lr = 0x8263C400;
	sub_82466E20(ctx, base);
	// 8263C400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C410 size=96
    let mut pc: u32 = 0x8263C410;
    'dispatch: loop {
        match pc {
            0x8263C410 => {
    //   block [0x8263C410..0x8263C470)
	// 8263C410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C41C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C424: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8263C428: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C430: 386ABA1C  addi r3, r10, -0x45e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17892;
	// 8263C434: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C43C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C45C: 4BE2A9C5  bl 0x82466e20
	ctx.lr = 0x8263C460;
	sub_82466E20(ctx, base);
	// 8263C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C470 size=100
    let mut pc: u32 = 0x8263C470;
    'dispatch: loop {
        match pc {
            0x8263C470 => {
    //   block [0x8263C470..0x8263C4D4)
	// 8263C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C47C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C484: 38AAB9EC  addi r5, r10, -0x4614
	ctx.r[5].s64 = ctx.r[10].s64 + -17940;
	// 8263C488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C490: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8263C494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C4A4: 386ABA4C  addi r3, r10, -0x45b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17844;
	// 8263C4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C4AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C4B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C4B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C4C0: 4BE2A961  bl 0x82466e20
	ctx.lr = 0x8263C4C4;
	sub_82466E20(ctx, base);
	// 8263C4C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C4D8 size=112
    let mut pc: u32 = 0x8263C4D8;
    'dispatch: loop {
        match pc {
            0x8263C4D8 => {
    //   block [0x8263C4D8..0x8263C548)
	// 8263C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C4E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C4E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C4EC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 8263C4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C4F4: 390B6D70  addi r8, r11, 0x6d70
	ctx.r[8].s64 = ctx.r[11].s64 + 28016;
	// 8263C4F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263C4FC: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8263C500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C510: 386ABA7C  addi r3, r10, -0x4584
	ctx.r[3].s64 = ctx.r[10].s64 + -17796;
	// 8263C514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C534: 4BE2A8ED  bl 0x82466e20
	ctx.lr = 0x8263C538;
	sub_82466E20(ctx, base);
	// 8263C538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C548 size=112
    let mut pc: u32 = 0x8263C548;
    'dispatch: loop {
        match pc {
            0x8263C548 => {
    //   block [0x8263C548..0x8263C5B8)
	// 8263C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C558: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C55C: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 8263C560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C564: 390B6DB8  addi r8, r11, 0x6db8
	ctx.r[8].s64 = ctx.r[11].s64 + 28088;
	// 8263C568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263C56C: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8263C570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C580: 386ABAAC  addi r3, r10, -0x4554
	ctx.r[3].s64 = ctx.r[10].s64 + -17748;
	// 8263C584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C5A4: 4BE2A87D  bl 0x82466e20
	ctx.lr = 0x8263C5A8;
	sub_82466E20(ctx, base);
	// 8263C5A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C5B8 size=100
    let mut pc: u32 = 0x8263C5B8;
    'dispatch: loop {
        match pc {
            0x8263C5B8 => {
    //   block [0x8263C5B8..0x8263C61C)
	// 8263C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C5C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C5CC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 8263C5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C5D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C5D8: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8263C5DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C5EC: 386ABADC  addi r3, r10, -0x4524
	ctx.r[3].s64 = ctx.r[10].s64 + -17700;
	// 8263C5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C5F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C5F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C600: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C608: 4BE2A819  bl 0x82466e20
	ctx.lr = 0x8263C60C;
	sub_82466E20(ctx, base);
	// 8263C60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C620 size=96
    let mut pc: u32 = 0x8263C620;
    'dispatch: loop {
        match pc {
            0x8263C620 => {
    //   block [0x8263C620..0x8263C680)
	// 8263C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C62C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C634: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8263C638: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C640: 386ABB0C  addi r3, r10, -0x44f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17652;
	// 8263C644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C64C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C660: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C66C: 4BE2A7B5  bl 0x82466e20
	ctx.lr = 0x8263C670;
	sub_82466E20(ctx, base);
	// 8263C670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C680 size=112
    let mut pc: u32 = 0x8263C680;
    'dispatch: loop {
        match pc {
            0x8263C680 => {
    //   block [0x8263C680..0x8263C6F0)
	// 8263C680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C68C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C690: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C694: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263C698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C69C: 390B6DD0  addi r8, r11, 0x6dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 28112;
	// 8263C6A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263C6A4: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8263C6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C6AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C6B8: 386ABB3C  addi r3, r10, -0x44c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17604;
	// 8263C6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C6DC: 4BE2A745  bl 0x82466e20
	ctx.lr = 0x8263C6E0;
	sub_82466E20(ctx, base);
	// 8263C6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C6F0 size=96
    let mut pc: u32 = 0x8263C6F0;
    'dispatch: loop {
        match pc {
            0x8263C6F0 => {
    //   block [0x8263C6F0..0x8263C750)
	// 8263C6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C6FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C704: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 8263C708: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C710: 386ABB6C  addi r3, r10, -0x4494
	ctx.r[3].s64 = ctx.r[10].s64 + -17556;
	// 8263C714: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C71C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C72C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C730: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C73C: 4BE2A6E5  bl 0x82466e20
	ctx.lr = 0x8263C740;
	sub_82466E20(ctx, base);
	// 8263C740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C750 size=112
    let mut pc: u32 = 0x8263C750;
    'dispatch: loop {
        match pc {
            0x8263C750 => {
    //   block [0x8263C750..0x8263C7C0)
	// 8263C750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C75C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C760: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C764: 38AABB6C  addi r5, r10, -0x4494
	ctx.r[5].s64 = ctx.r[10].s64 + -17556;
	// 8263C768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C76C: 390B6E00  addi r8, r11, 0x6e00
	ctx.r[8].s64 = ctx.r[11].s64 + 28160;
	// 8263C770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263C774: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 8263C778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C77C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C788: 386ABB9C  addi r3, r10, -0x4464
	ctx.r[3].s64 = ctx.r[10].s64 + -17508;
	// 8263C78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C7AC: 4BE2A675  bl 0x82466e20
	ctx.lr = 0x8263C7B0;
	sub_82466E20(ctx, base);
	// 8263C7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C7C0 size=108
    let mut pc: u32 = 0x8263C7C0;
    'dispatch: loop {
        match pc {
            0x8263C7C0 => {
    //   block [0x8263C7C0..0x8263C82C)
	// 8263C7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C7CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C7D4: 38EB6E18  addi r7, r11, 0x6e18
	ctx.r[7].s64 = ctx.r[11].s64 + 28184;
	// 8263C7D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263C7DC: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8263C7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C7E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C7E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C7EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C7F0: 386ABBCC  addi r3, r10, -0x4434
	ctx.r[3].s64 = ctx.r[10].s64 + -17460;
	// 8263C7F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C7FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C818: 4BE2A609  bl 0x82466e20
	ctx.lr = 0x8263C81C;
	sub_82466E20(ctx, base);
	// 8263C81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C830 size=112
    let mut pc: u32 = 0x8263C830;
    'dispatch: loop {
        match pc {
            0x8263C830 => {
    //   block [0x8263C830..0x8263C8A0)
	// 8263C830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C83C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C840: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C844: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263C848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C84C: 390B6E78  addi r8, r11, 0x6e78
	ctx.r[8].s64 = ctx.r[11].s64 + 28280;
	// 8263C850: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263C854: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8263C858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C85C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C868: 386ABBFC  addi r3, r10, -0x4404
	ctx.r[3].s64 = ctx.r[10].s64 + -17412;
	// 8263C86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C88C: 4BE2A595  bl 0x82466e20
	ctx.lr = 0x8263C890;
	sub_82466E20(ctx, base);
	// 8263C890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C8A0 size=112
    let mut pc: u32 = 0x8263C8A0;
    'dispatch: loop {
        match pc {
            0x8263C8A0 => {
    //   block [0x8263C8A0..0x8263C910)
	// 8263C8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C8B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C8B4: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263C8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C8BC: 390B6E90  addi r8, r11, 0x6e90
	ctx.r[8].s64 = ctx.r[11].s64 + 28304;
	// 8263C8C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263C8C4: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8263C8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C8CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C8D8: 386ABC2C  addi r3, r10, -0x43d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17364;
	// 8263C8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C8FC: 4BE2A525  bl 0x82466e20
	ctx.lr = 0x8263C900;
	sub_82466E20(ctx, base);
	// 8263C900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C910 size=100
    let mut pc: u32 = 0x8263C910;
    'dispatch: loop {
        match pc {
            0x8263C910 => {
    //   block [0x8263C910..0x8263C974)
	// 8263C910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C91C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C924: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263C928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C930: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8263C934: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C944: 386ABC5C  addi r3, r10, -0x43a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17316;
	// 8263C948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C94C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C950: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C960: 4BE2A4C1  bl 0x82466e20
	ctx.lr = 0x8263C964;
	sub_82466E20(ctx, base);
	// 8263C964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C978 size=116
    let mut pc: u32 = 0x8263C978;
    'dispatch: loop {
        match pc {
            0x8263C978 => {
    //   block [0x8263C978..0x8263C9EC)
	// 8263C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C984: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C98C: 390B6EC0  addi r8, r11, 0x6ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 28352;
	// 8263C990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C994: 392A67D0  addi r9, r10, 0x67d0
	ctx.r[9].s64 = ctx.r[10].s64 + 26576;
	// 8263C998: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C99C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263C9A0: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263C9A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C9AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C9BC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263C9C0: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8263C9C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263C9C8: 386BBC8C  addi r3, r11, -0x4374
	ctx.r[3].s64 = ctx.r[11].s64 + -17268;
	// 8263C9CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C9D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C9D8: 4BE2A449  bl 0x82466e20
	ctx.lr = 0x8263C9DC;
	sub_82466E20(ctx, base);
	// 8263C9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C9F0 size=112
    let mut pc: u32 = 0x8263C9F0;
    'dispatch: loop {
        match pc {
            0x8263C9F0 => {
    //   block [0x8263C9F0..0x8263CA60)
	// 8263C9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C9FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CA00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CA04: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263CA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CA0C: 390B6EF0  addi r8, r11, 0x6ef0
	ctx.r[8].s64 = ctx.r[11].s64 + 28400;
	// 8263CA10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263CA14: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8263CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CA1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CA20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CA28: 386ABCBC  addi r3, r10, -0x4344
	ctx.r[3].s64 = ctx.r[10].s64 + -17220;
	// 8263CA2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CA3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CA4C: 4BE2A3D5  bl 0x82466e20
	ctx.lr = 0x8263CA50;
	sub_82466E20(ctx, base);
	// 8263CA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CA60 size=116
    let mut pc: u32 = 0x8263CA60;
    'dispatch: loop {
        match pc {
            0x8263CA60 => {
    //   block [0x8263CA60..0x8263CAD4)
	// 8263CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CA6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CA70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263CA74: 390B6F0C  addi r8, r11, 0x6f0c
	ctx.r[8].s64 = ctx.r[11].s64 + 28428;
	// 8263CA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CA7C: 392A67FC  addi r9, r10, 0x67fc
	ctx.r[9].s64 = ctx.r[10].s64 + 26620;
	// 8263CA80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CA84: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263CA88: 38AAC31C  addi r5, r10, -0x3ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -15588;
	// 8263CA8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CA94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CAA4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263CAA8: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8263CAAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263CAB0: 386BBCEC  addi r3, r11, -0x4314
	ctx.r[3].s64 = ctx.r[11].s64 + -17172;
	// 8263CAB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263CAB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CAC0: 4BE2A361  bl 0x82466e20
	ctx.lr = 0x8263CAC4;
	sub_82466E20(ctx, base);
	// 8263CAC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CAD8 size=112
    let mut pc: u32 = 0x8263CAD8;
    'dispatch: loop {
        match pc {
            0x8263CAD8 => {
    //   block [0x8263CAD8..0x8263CB48)
	// 8263CAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CAE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CAE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CAEC: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CAF4: 390B6F28  addi r8, r11, 0x6f28
	ctx.r[8].s64 = ctx.r[11].s64 + 28456;
	// 8263CAF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263CAFC: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8263CB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CB04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CB10: 386ABD1C  addi r3, r10, -0x42e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17124;
	// 8263CB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CB24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263CB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CB34: 4BE2A2ED  bl 0x82466e20
	ctx.lr = 0x8263CB38;
	sub_82466E20(ctx, base);
	// 8263CB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CB48 size=112
    let mut pc: u32 = 0x8263CB48;
    'dispatch: loop {
        match pc {
            0x8263CB48 => {
    //   block [0x8263CB48..0x8263CBB8)
	// 8263CB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CB54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CB58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CB5C: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263CB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CB64: 390B6FA0  addi r8, r11, 0x6fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 28576;
	// 8263CB68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263CB6C: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8263CB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CB74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CB78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CB7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CB80: 386ABD4C  addi r3, r10, -0x42b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17076;
	// 8263CB84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CB8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CB94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CBA4: 4BE2A27D  bl 0x82466e20
	ctx.lr = 0x8263CBA8;
	sub_82466E20(ctx, base);
	// 8263CBA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CBB8 size=112
    let mut pc: u32 = 0x8263CBB8;
    'dispatch: loop {
        match pc {
            0x8263CBB8 => {
    //   block [0x8263CBB8..0x8263CC28)
	// 8263CBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CBC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CBC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CBCC: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CBD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CBD4: 390B6FE8  addi r8, r11, 0x6fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 28648;
	// 8263CBD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263CBDC: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8263CBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CBE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CBE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CBF0: 386ABD7C  addi r3, r10, -0x4284
	ctx.r[3].s64 = ctx.r[10].s64 + -17028;
	// 8263CBF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CC14: 4BE2A20D  bl 0x82466e20
	ctx.lr = 0x8263CC18;
	sub_82466E20(ctx, base);
	// 8263CC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CC28 size=112
    let mut pc: u32 = 0x8263CC28;
    'dispatch: loop {
        match pc {
            0x8263CC28 => {
    //   block [0x8263CC28..0x8263CC98)
	// 8263CC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CC34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CC38: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CC3C: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CC44: 390B7030  addi r8, r11, 0x7030
	ctx.r[8].s64 = ctx.r[11].s64 + 28720;
	// 8263CC48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263CC4C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8263CC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CC54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CC58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CC60: 386ABDAC  addi r3, r10, -0x4254
	ctx.r[3].s64 = ctx.r[10].s64 + -16980;
	// 8263CC64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CC84: 4BE2A19D  bl 0x82466e20
	ctx.lr = 0x8263CC88;
	sub_82466E20(ctx, base);
	// 8263CC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CC98 size=108
    let mut pc: u32 = 0x8263CC98;
    'dispatch: loop {
        match pc {
            0x8263CC98 => {
    //   block [0x8263CC98..0x8263CD04)
	// 8263CC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CCA4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CCAC: 38EB7078  addi r7, r11, 0x7078
	ctx.r[7].s64 = ctx.r[11].s64 + 28792;
	// 8263CCB0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263CCB4: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8263CCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CCBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CCC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CCC8: 386ABDDC  addi r3, r10, -0x4224
	ctx.r[3].s64 = ctx.r[10].s64 + -16932;
	// 8263CCCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263CCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CCD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CCDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CCEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263CCF0: 4BE2A131  bl 0x82466e20
	ctx.lr = 0x8263CCF4;
	sub_82466E20(ctx, base);
	// 8263CCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


