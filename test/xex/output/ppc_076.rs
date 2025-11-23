pub fn sub_825F2348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2348 size=116
    let mut pc: u32 = 0x825F2348;
    'dispatch: loop {
        match pc {
            0x825F2348 => {
    //   block [0x825F2348..0x825F23BC)
	// 825F2348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2354: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2358: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 825F235C: 390AEB58  addi r8, r10, -0x14a8
	ctx.r[8].s64 = ctx.r[10].s64 + -5288;
	// 825F2360: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2364: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2368: 38AAC2C4  addi r5, r10, -0x3d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -15676;
	// 825F236C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2370: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F2374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F237C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 825F2380: 396B1A2C  addi r11, r11, 0x1a2c
	ctx.r[11].s64 = ctx.r[11].s64 + 6700;
	// 825F2384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2388: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F238C: 386AC714  addi r3, r10, -0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14572;
	// 825F2390: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F2394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2398: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F239C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F23A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F23A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F23A8: 4BE74A79  bl 0x82466e20
	ctx.lr = 0x825F23AC;
	sub_82466E20(ctx, base);
	// 825F23AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F23B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F23B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F23B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F23C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F23C0 size=112
    let mut pc: u32 = 0x825F23C0;
    'dispatch: loop {
        match pc {
            0x825F23C0 => {
    //   block [0x825F23C0..0x825F2430)
	// 825F23C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F23C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F23C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F23CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F23D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F23D4: 38AAE094  addi r5, r10, -0x1f6c
	ctx.r[5].s64 = ctx.r[10].s64 + -8044;
	// 825F23D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F23DC: 390BA610  addi r8, r11, -0x59f0
	ctx.r[8].s64 = ctx.r[11].s64 + -23024;
	// 825F23E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F23E4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 825F23E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F23EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F23F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F23F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F23F8: 386AC744  addi r3, r10, -0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14524;
	// 825F23FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F240C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F241C: 4BE74A05  bl 0x82466e20
	ctx.lr = 0x825F2420;
	sub_82466E20(ctx, base);
	// 825F2420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F242C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2430 size=108
    let mut pc: u32 = 0x825F2430;
    'dispatch: loop {
        match pc {
            0x825F2430 => {
    //   block [0x825F2430..0x825F249C)
	// 825F2430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F243C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2444: 38EBA640  addi r7, r11, -0x59c0
	ctx.r[7].s64 = ctx.r[11].s64 + -22976;
	// 825F2448: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F244C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 825F2450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2454: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F245C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2460: 386AC774  addi r3, r10, -0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + -14476;
	// 825F2464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F2468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F246C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F247C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2488: 4BE74999  bl 0x82466e20
	ctx.lr = 0x825F248C;
	sub_82466E20(ctx, base);
	// 825F248C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F24A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F24A0 size=112
    let mut pc: u32 = 0x825F24A0;
    'dispatch: loop {
        match pc {
            0x825F24A0 => {
    //   block [0x825F24A0..0x825F2510)
	// 825F24A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F24A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F24A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F24AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F24B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F24B4: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F24B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F24BC: 390BA670  addi r8, r11, -0x5990
	ctx.r[8].s64 = ctx.r[11].s64 + -22928;
	// 825F24C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F24C4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 825F24C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F24CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F24D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F24D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F24D8: 386AC7A4  addi r3, r10, -0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + -14428;
	// 825F24DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F24E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F24E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F24E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F24EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F24F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F24F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F24F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F24FC: 4BE74925  bl 0x82466e20
	ctx.lr = 0x825F2500;
	sub_82466E20(ctx, base);
	// 825F2500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F250C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2510 size=112
    let mut pc: u32 = 0x825F2510;
    'dispatch: loop {
        match pc {
            0x825F2510 => {
    //   block [0x825F2510..0x825F2580)
	// 825F2510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F251C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2524: 38AAC984  addi r5, r10, -0x367c
	ctx.r[5].s64 = ctx.r[10].s64 + -13948;
	// 825F2528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F252C: 390BA6A0  addi r8, r11, -0x5960
	ctx.r[8].s64 = ctx.r[11].s64 + -22880;
	// 825F2530: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2534: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 825F2538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F253C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2548: 386AC7D4  addi r3, r10, -0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + -14380;
	// 825F254C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F255C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F256C: 4BE748B5  bl 0x82466e20
	ctx.lr = 0x825F2570;
	sub_82466E20(ctx, base);
	// 825F2570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F257C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2580 size=108
    let mut pc: u32 = 0x825F2580;
    'dispatch: loop {
        match pc {
            0x825F2580 => {
    //   block [0x825F2580..0x825F25EC)
	// 825F2580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F258C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2594: 38EBA6D0  addi r7, r11, -0x5930
	ctx.r[7].s64 = ctx.r[11].s64 + -22832;
	// 825F2598: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F259C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 825F25A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F25A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F25A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F25AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F25B0: 386AC804  addi r3, r10, -0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14332;
	// 825F25B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F25B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F25BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F25C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F25C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F25C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F25CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F25D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F25D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F25D8: 4BE74849  bl 0x82466e20
	ctx.lr = 0x825F25DC;
	sub_82466E20(ctx, base);
	// 825F25DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F25E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F25E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F25E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F25F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F25F0 size=112
    let mut pc: u32 = 0x825F25F0;
    'dispatch: loop {
        match pc {
            0x825F25F0 => {
    //   block [0x825F25F0..0x825F2660)
	// 825F25F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F25F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F25F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F25FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2600: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2604: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F2608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F260C: 390BA718  addi r8, r11, -0x58e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22760;
	// 825F2610: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F2614: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 825F2618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F261C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2628: 386AC834  addi r3, r10, -0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + -14284;
	// 825F262C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F263C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F264C: 4BE747D5  bl 0x82466e20
	ctx.lr = 0x825F2650;
	sub_82466E20(ctx, base);
	// 825F2650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F265C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2660 size=100
    let mut pc: u32 = 0x825F2660;
    'dispatch: loop {
        match pc {
            0x825F2660 => {
    //   block [0x825F2660..0x825F26C4)
	// 825F2660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F266C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2674: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2680: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 825F2684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F268C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2694: 386AC864  addi r3, r10, -0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + -14236;
	// 825F2698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F269C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F26A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F26A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F26A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F26AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F26B0: 4BE74771  bl 0x82466e20
	ctx.lr = 0x825F26B4;
	sub_82466E20(ctx, base);
	// 825F26B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F26B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F26BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F26C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F26C8 size=112
    let mut pc: u32 = 0x825F26C8;
    'dispatch: loop {
        match pc {
            0x825F26C8 => {
    //   block [0x825F26C8..0x825F2738)
	// 825F26C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F26CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F26D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F26D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F26D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F26DC: 38AAC5C4  addi r5, r10, -0x3a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -14908;
	// 825F26E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F26E4: 390BA778  addi r8, r11, -0x5888
	ctx.r[8].s64 = ctx.r[11].s64 + -22664;
	// 825F26E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F26EC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 825F26F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F26F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F26F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F26FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2700: 386AC894  addi r3, r10, -0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + -14188;
	// 825F2704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F270C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F271C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2724: 4BE746FD  bl 0x82466e20
	ctx.lr = 0x825F2728;
	sub_82466E20(ctx, base);
	// 825F2728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F272C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2738 size=112
    let mut pc: u32 = 0x825F2738;
    'dispatch: loop {
        match pc {
            0x825F2738 => {
    //   block [0x825F2738..0x825F27A8)
	// 825F2738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F273C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2748: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F274C: 38AAC5C4  addi r5, r10, -0x3a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -14908;
	// 825F2750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2754: 390BA7C0  addi r8, r11, -0x5840
	ctx.r[8].s64 = ctx.r[11].s64 + -22592;
	// 825F2758: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F275C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 825F2760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F276C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2770: 386AC8C4  addi r3, r10, -0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + -14140;
	// 825F2774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F277C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2794: 4BE7468D  bl 0x82466e20
	ctx.lr = 0x825F2798;
	sub_82466E20(ctx, base);
	// 825F2798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F27A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F27A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F27A8 size=108
    let mut pc: u32 = 0x825F27A8;
    'dispatch: loop {
        match pc {
            0x825F27A8 => {
    //   block [0x825F27A8..0x825F2814)
	// 825F27A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F27AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F27B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F27B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F27B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F27BC: 38EBA868  addi r7, r11, -0x5798
	ctx.r[7].s64 = ctx.r[11].s64 + -22424;
	// 825F27C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F27C4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 825F27C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F27CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F27D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F27D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F27D8: 386AC8F4  addi r3, r10, -0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + -14092;
	// 825F27DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F27E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F27E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F27E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F27EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F27F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F27F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F27F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F27FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2800: 4BE74621  bl 0x82466e20
	ctx.lr = 0x825F2804;
	sub_82466E20(ctx, base);
	// 825F2804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F280C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2818 size=24
    let mut pc: u32 = 0x825F2818;
    'dispatch: loop {
        match pc {
            0x825F2818 => {
    //   block [0x825F2818..0x825F2830)
	// 825F2818: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F281C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2820: 394AEC90  addi r10, r10, -0x1370
	ctx.r[10].s64 = ctx.r[10].s64 + -4976;
	// 825F2824: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F2828: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F282C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2830 size=116
    let mut pc: u32 = 0x825F2830;
    'dispatch: loop {
        match pc {
            0x825F2830 => {
    //   block [0x825F2830..0x825F28A4)
	// 825F2830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F283C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2840: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F2844: 390AEC90  addi r8, r10, -0x1370
	ctx.r[8].s64 = ctx.r[10].s64 + -4976;
	// 825F2848: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F284C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2850: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F2854: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2858: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F285C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2864: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 825F2868: 396B1A64  addi r11, r11, 0x1a64
	ctx.r[11].s64 = ctx.r[11].s64 + 6756;
	// 825F286C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2870: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2874: 386AC924  addi r3, r10, -0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14044;
	// 825F2878: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F287C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2880: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F2884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F288C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2890: 4BE74591  bl 0x82466e20
	ctx.lr = 0x825F2894;
	sub_82466E20(ctx, base);
	// 825F2894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F289C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F28A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F28A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F28A8 size=100
    let mut pc: u32 = 0x825F28A8;
    'dispatch: loop {
        match pc {
            0x825F28A8 => {
    //   block [0x825F28A8..0x825F290C)
	// 825F28A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F28AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F28B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F28B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F28B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F28BC: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F28C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F28C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F28C8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 825F28CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F28D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F28D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F28D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F28DC: 386AC954  addi r3, r10, -0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13996;
	// 825F28E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F28E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F28E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F28EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F28F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F28F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F28F8: 4BE74529  bl 0x82466e20
	ctx.lr = 0x825F28FC;
	sub_82466E20(ctx, base);
	// 825F28FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2910 size=100
    let mut pc: u32 = 0x825F2910;
    'dispatch: loop {
        match pc {
            0x825F2910 => {
    //   block [0x825F2910..0x825F2974)
	// 825F2910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F291C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2924: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F292C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2930: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 825F2934: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F293C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2944: 386AC984  addi r3, r10, -0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + -13948;
	// 825F2948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F294C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2950: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F2954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F295C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2960: 4BE744C1  bl 0x82466e20
	ctx.lr = 0x825F2964;
	sub_82466E20(ctx, base);
	// 825F2964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F296C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2978 size=112
    let mut pc: u32 = 0x825F2978;
    'dispatch: loop {
        match pc {
            0x825F2978 => {
    //   block [0x825F2978..0x825F29E8)
	// 825F2978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2988: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F298C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F2990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2994: 390BA8C8  addi r8, r11, -0x5738
	ctx.r[8].s64 = ctx.r[11].s64 + -22328;
	// 825F2998: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F299C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 825F29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F29A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F29A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F29AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F29B0: 386AC9B4  addi r3, r10, -0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + -13900;
	// 825F29B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F29B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F29BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F29C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F29C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F29C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F29CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F29D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F29D4: 4BE7444D  bl 0x82466e20
	ctx.lr = 0x825F29D8;
	sub_82466E20(ctx, base);
	// 825F29D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F29DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F29E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F29E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F29E8 size=112
    let mut pc: u32 = 0x825F29E8;
    'dispatch: loop {
        match pc {
            0x825F29E8 => {
    //   block [0x825F29E8..0x825F2A58)
	// 825F29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F29EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F29F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F29F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F29F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F29FC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F2A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2A04: 390BA958  addi r8, r11, -0x56a8
	ctx.r[8].s64 = ctx.r[11].s64 + -22184;
	// 825F2A08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F2A0C: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 825F2A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2A14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2A20: 386AC9E4  addi r3, r10, -0x361c
	ctx.r[3].s64 = ctx.r[10].s64 + -13852;
	// 825F2A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2A44: 4BE743DD  bl 0x82466e20
	ctx.lr = 0x825F2A48;
	sub_82466E20(ctx, base);
	// 825F2A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2A58 size=112
    let mut pc: u32 = 0x825F2A58;
    'dispatch: loop {
        match pc {
            0x825F2A58 => {
    //   block [0x825F2A58..0x825F2AC8)
	// 825F2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2A64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2A68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2A6C: 38AAC564  addi r5, r10, -0x3a9c
	ctx.r[5].s64 = ctx.r[10].s64 + -15004;
	// 825F2A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2A74: 390BA9B8  addi r8, r11, -0x5648
	ctx.r[8].s64 = ctx.r[11].s64 + -22088;
	// 825F2A78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2A7C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 825F2A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2A84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2A90: 386ACA14  addi r3, r10, -0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13804;
	// 825F2A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2AB4: 4BE7436D  bl 0x82466e20
	ctx.lr = 0x825F2AB8;
	sub_82466E20(ctx, base);
	// 825F2AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2AC8 size=112
    let mut pc: u32 = 0x825F2AC8;
    'dispatch: loop {
        match pc {
            0x825F2AC8 => {
    //   block [0x825F2AC8..0x825F2B38)
	// 825F2AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2AD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2AD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2ADC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F2AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2AE4: 390BA9E8  addi r8, r11, -0x5618
	ctx.r[8].s64 = ctx.r[11].s64 + -22040;
	// 825F2AE8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F2AEC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 825F2AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2AF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2B00: 386ACA44  addi r3, r10, -0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13756;
	// 825F2B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2B24: 4BE742FD  bl 0x82466e20
	ctx.lr = 0x825F2B28;
	sub_82466E20(ctx, base);
	// 825F2B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2B38 size=112
    let mut pc: u32 = 0x825F2B38;
    'dispatch: loop {
        match pc {
            0x825F2B38 => {
    //   block [0x825F2B38..0x825F2BA8)
	// 825F2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2B44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2B48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2B4C: 38AAC6B4  addi r5, r10, -0x394c
	ctx.r[5].s64 = ctx.r[10].s64 + -14668;
	// 825F2B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2B54: 390BAA78  addi r8, r11, -0x5588
	ctx.r[8].s64 = ctx.r[11].s64 + -21896;
	// 825F2B58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F2B5C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 825F2B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2B64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2B70: 386ACA74  addi r3, r10, -0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + -13708;
	// 825F2B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2B94: 4BE7428D  bl 0x82466e20
	ctx.lr = 0x825F2B98;
	sub_82466E20(ctx, base);
	// 825F2B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2BA8 size=112
    let mut pc: u32 = 0x825F2BA8;
    'dispatch: loop {
        match pc {
            0x825F2BA8 => {
    //   block [0x825F2BA8..0x825F2C18)
	// 825F2BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2BB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2BB8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2BBC: 38AAC8C4  addi r5, r10, -0x373c
	ctx.r[5].s64 = ctx.r[10].s64 + -14140;
	// 825F2BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2BC4: 390BAA90  addi r8, r11, -0x5570
	ctx.r[8].s64 = ctx.r[11].s64 + -21872;
	// 825F2BC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2BCC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 825F2BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2BD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2BE0: 386ACAA4  addi r3, r10, -0x355c
	ctx.r[3].s64 = ctx.r[10].s64 + -13660;
	// 825F2BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2C04: 4BE7421D  bl 0x82466e20
	ctx.lr = 0x825F2C08;
	sub_82466E20(ctx, base);
	// 825F2C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2C18 size=112
    let mut pc: u32 = 0x825F2C18;
    'dispatch: loop {
        match pc {
            0x825F2C18 => {
    //   block [0x825F2C18..0x825F2C88)
	// 825F2C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2C24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2C28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2C2C: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2C34: 390BAAC0  addi r8, r11, -0x5540
	ctx.r[8].s64 = ctx.r[11].s64 + -21824;
	// 825F2C38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F2C3C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 825F2C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2C50: 386ACAD4  addi r3, r10, -0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + -13612;
	// 825F2C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2C74: 4BE741AD  bl 0x82466e20
	ctx.lr = 0x825F2C78;
	sub_82466E20(ctx, base);
	// 825F2C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2C88 size=24
    let mut pc: u32 = 0x825F2C88;
    'dispatch: loop {
        match pc {
            0x825F2C88 => {
    //   block [0x825F2C88..0x825F2CA0)
	// 825F2C88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2C8C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2C90: 394AED08  addi r10, r10, -0x12f8
	ctx.r[10].s64 = ctx.r[10].s64 + -4856;
	// 825F2C94: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F2C98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825F2C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2CA0 size=116
    let mut pc: u32 = 0x825F2CA0;
    'dispatch: loop {
        match pc {
            0x825F2CA0 => {
    //   block [0x825F2CA0..0x825F2D14)
	// 825F2CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2CAC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2CB0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F2CB4: 390AED08  addi r8, r10, -0x12f8
	ctx.r[8].s64 = ctx.r[10].s64 + -4856;
	// 825F2CB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2CBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2CC0: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F2CC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2CC8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F2CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2CD4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 825F2CD8: 396B1A7C  addi r11, r11, 0x1a7c
	ctx.r[11].s64 = ctx.r[11].s64 + 6780;
	// 825F2CDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2CE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2CE4: 386ACB04  addi r3, r10, -0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + -13564;
	// 825F2CE8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F2CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2CF0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F2CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2D00: 4BE74121  bl 0x82466e20
	ctx.lr = 0x825F2D04;
	sub_82466E20(ctx, base);
	// 825F2D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2D18 size=112
    let mut pc: u32 = 0x825F2D18;
    'dispatch: loop {
        match pc {
            0x825F2D18 => {
    //   block [0x825F2D18..0x825F2D88)
	// 825F2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2D24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2D28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2D2C: 38AAC2C4  addi r5, r10, -0x3d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -15676;
	// 825F2D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2D34: 390BAB08  addi r8, r11, -0x54f8
	ctx.r[8].s64 = ctx.r[11].s64 + -21752;
	// 825F2D38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2D3C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 825F2D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2D50: 386ACB34  addi r3, r10, -0x34cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13516;
	// 825F2D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2D74: 4BE740AD  bl 0x82466e20
	ctx.lr = 0x825F2D78;
	sub_82466E20(ctx, base);
	// 825F2D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2D88 size=112
    let mut pc: u32 = 0x825F2D88;
    'dispatch: loop {
        match pc {
            0x825F2D88 => {
    //   block [0x825F2D88..0x825F2DF8)
	// 825F2D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2D94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2D98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2D9C: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F2DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2DA4: 390BAB38  addi r8, r11, -0x54c8
	ctx.r[8].s64 = ctx.r[11].s64 + -21704;
	// 825F2DA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2DAC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 825F2DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2DB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2DC0: 386ACB64  addi r3, r10, -0x349c
	ctx.r[3].s64 = ctx.r[10].s64 + -13468;
	// 825F2DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2DE4: 4BE7403D  bl 0x82466e20
	ctx.lr = 0x825F2DE8;
	sub_82466E20(ctx, base);
	// 825F2DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2DF8 size=100
    let mut pc: u32 = 0x825F2DF8;
    'dispatch: loop {
        match pc {
            0x825F2DF8 => {
    //   block [0x825F2DF8..0x825F2E5C)
	// 825F2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2E04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2E0C: 392A1AEC  addi r9, r10, 0x1aec
	ctx.r[9].s64 = ctx.r[10].s64 + 6892;
	// 825F2E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2E18: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 825F2E1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2E2C: 386ACB94  addi r3, r10, -0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + -13420;
	// 825F2E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2E34: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825F2E38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F2E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2E40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F2E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2E48: 4BE73FD9  bl 0x82466e20
	ctx.lr = 0x825F2E4C;
	sub_82466E20(ctx, base);
	// 825F2E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2E60 size=24
    let mut pc: u32 = 0x825F2E60;
    'dispatch: loop {
        match pc {
            0x825F2E60 => {
    //   block [0x825F2E60..0x825F2E78)
	// 825F2E60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2E64: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2E68: 394AEDB0  addi r10, r10, -0x1250
	ctx.r[10].s64 = ctx.r[10].s64 + -4688;
	// 825F2E6C: 816BAB74  lwz r11, -0x548c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21644 as u32) ) } as u64;
	// 825F2E70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F2E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2E78 size=112
    let mut pc: u32 = 0x825F2E78;
    'dispatch: loop {
        match pc {
            0x825F2E78 => {
    //   block [0x825F2E78..0x825F2EE8)
	// 825F2E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2E84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2E8C: 392A1C28  addi r9, r10, 0x1c28
	ctx.r[9].s64 = ctx.r[10].s64 + 7208;
	// 825F2E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E94: 390BEDB0  addi r8, r11, -0x1250
	ctx.r[8].s64 = ctx.r[11].s64 + -4688;
	// 825F2E98: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F2E9C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 825F2EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2EA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2EB0: 386ACBC4  addi r3, r10, -0x343c
	ctx.r[3].s64 = ctx.r[10].s64 + -13372;
	// 825F2EB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F2EB8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F2EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2ED4: 4BE73F4D  bl 0x82466e20
	ctx.lr = 0x825F2ED8;
	sub_82466E20(ctx, base);
	// 825F2ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2EE8 size=112
    let mut pc: u32 = 0x825F2EE8;
    'dispatch: loop {
        match pc {
            0x825F2EE8 => {
    //   block [0x825F2EE8..0x825F2F58)
	// 825F2EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2EF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2EF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2EFC: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F2F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2F04: 390BAB7C  addi r8, r11, -0x5484
	ctx.r[8].s64 = ctx.r[11].s64 + -21636;
	// 825F2F08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2F0C: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 825F2F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2F14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2F20: 386ACBF4  addi r3, r10, -0x340c
	ctx.r[3].s64 = ctx.r[10].s64 + -13324;
	// 825F2F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2F44: 4BE73EDD  bl 0x82466e20
	ctx.lr = 0x825F2F48;
	sub_82466E20(ctx, base);
	// 825F2F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2F58 size=108
    let mut pc: u32 = 0x825F2F58;
    'dispatch: loop {
        match pc {
            0x825F2F58 => {
    //   block [0x825F2F58..0x825F2FC4)
	// 825F2F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2F64: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2F6C: 38EBABAC  addi r7, r11, -0x5454
	ctx.r[7].s64 = ctx.r[11].s64 + -21588;
	// 825F2F70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F2F74: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 825F2F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F2F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2F88: 386ACC24  addi r3, r10, -0x33dc
	ctx.r[3].s64 = ctx.r[10].s64 + -13276;
	// 825F2F8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F2F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2FB0: 4BE73E71  bl 0x82466e20
	ctx.lr = 0x825F2FB4;
	sub_82466E20(ctx, base);
	// 825F2FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2FC8 size=100
    let mut pc: u32 = 0x825F2FC8;
    'dispatch: loop {
        match pc {
            0x825F2FC8 => {
    //   block [0x825F2FC8..0x825F302C)
	// 825F2FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2FD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2FDC: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F2FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2FE8: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 825F2FEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2FFC: 386ACC54  addi r3, r10, -0x33ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13228;
	// 825F3000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3008: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F300C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3010: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F3014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3018: 4BE73E09  bl 0x82466e20
	ctx.lr = 0x825F301C;
	sub_82466E20(ctx, base);
	// 825F301C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3030 size=112
    let mut pc: u32 = 0x825F3030;
    'dispatch: loop {
        match pc {
            0x825F3030 => {
    //   block [0x825F3030..0x825F30A0)
	// 825F3030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F303C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3040: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3044: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F304C: 390BABC4  addi r8, r11, -0x543c
	ctx.r[8].s64 = ctx.r[11].s64 + -21564;
	// 825F3050: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3054: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 825F3058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F305C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3068: 386ACC84  addi r3, r10, -0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + -13180;
	// 825F306C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F307C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F308C: 4BE73D95  bl 0x82466e20
	ctx.lr = 0x825F3090;
	sub_82466E20(ctx, base);
	// 825F3090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F30A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F30A0 size=112
    let mut pc: u32 = 0x825F30A0;
    'dispatch: loop {
        match pc {
            0x825F30A0 => {
    //   block [0x825F30A0..0x825F3110)
	// 825F30A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F30A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F30A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F30AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F30B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F30B4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F30B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F30BC: 390BABDC  addi r8, r11, -0x5424
	ctx.r[8].s64 = ctx.r[11].s64 + -21540;
	// 825F30C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F30C4: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 825F30C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F30CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F30D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F30D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F30D8: 386ACCB4  addi r3, r10, -0x334c
	ctx.r[3].s64 = ctx.r[10].s64 + -13132;
	// 825F30DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F30E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F30E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F30E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F30EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F30F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F30F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F30F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F30FC: 4BE73D25  bl 0x82466e20
	ctx.lr = 0x825F3100;
	sub_82466E20(ctx, base);
	// 825F3100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3110 size=112
    let mut pc: u32 = 0x825F3110;
    'dispatch: loop {
        match pc {
            0x825F3110 => {
    //   block [0x825F3110..0x825F3180)
	// 825F3110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F311C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3120: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3124: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F312C: 390BAC0C  addi r8, r11, -0x53f4
	ctx.r[8].s64 = ctx.r[11].s64 + -21492;
	// 825F3130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3134: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 825F3138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F313C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3148: 386ACCE4  addi r3, r10, -0x331c
	ctx.r[3].s64 = ctx.r[10].s64 + -13084;
	// 825F314C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F315C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F316C: 4BE73CB5  bl 0x82466e20
	ctx.lr = 0x825F3170;
	sub_82466E20(ctx, base);
	// 825F3170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F317C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3180 size=112
    let mut pc: u32 = 0x825F3180;
    'dispatch: loop {
        match pc {
            0x825F3180 => {
    //   block [0x825F3180..0x825F31F0)
	// 825F3180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F318C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3190: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3194: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F319C: 390BAC3C  addi r8, r11, -0x53c4
	ctx.r[8].s64 = ctx.r[11].s64 + -21444;
	// 825F31A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F31A4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 825F31A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F31AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F31B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F31B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F31B8: 386ACD14  addi r3, r10, -0x32ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13036;
	// 825F31BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F31C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F31C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F31C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F31CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F31D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F31D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F31D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F31DC: 4BE73C45  bl 0x82466e20
	ctx.lr = 0x825F31E0;
	sub_82466E20(ctx, base);
	// 825F31E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F31E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F31E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F31EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F31F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F31F0 size=112
    let mut pc: u32 = 0x825F31F0;
    'dispatch: loop {
        match pc {
            0x825F31F0 => {
    //   block [0x825F31F0..0x825F3260)
	// 825F31F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F31F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F31F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F31FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3200: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3204: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F320C: 390BAC6C  addi r8, r11, -0x5394
	ctx.r[8].s64 = ctx.r[11].s64 + -21396;
	// 825F3210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3214: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 825F3218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F321C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3228: 386ACD44  addi r3, r10, -0x32bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12988;
	// 825F322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F324C: 4BE73BD5  bl 0x82466e20
	ctx.lr = 0x825F3250;
	sub_82466E20(ctx, base);
	// 825F3250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3260 size=112
    let mut pc: u32 = 0x825F3260;
    'dispatch: loop {
        match pc {
            0x825F3260 => {
    //   block [0x825F3260..0x825F32D0)
	// 825F3260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F326C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3270: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3274: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F327C: 390BAC84  addi r8, r11, -0x537c
	ctx.r[8].s64 = ctx.r[11].s64 + -21372;
	// 825F3280: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3284: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 825F3288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F328C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3298: 386ACD74  addi r3, r10, -0x328c
	ctx.r[3].s64 = ctx.r[10].s64 + -12940;
	// 825F329C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F32A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F32A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F32A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F32AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F32B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F32B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F32B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F32BC: 4BE73B65  bl 0x82466e20
	ctx.lr = 0x825F32C0;
	sub_82466E20(ctx, base);
	// 825F32C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F32C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F32C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F32CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F32D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F32D0 size=112
    let mut pc: u32 = 0x825F32D0;
    'dispatch: loop {
        match pc {
            0x825F32D0 => {
    //   block [0x825F32D0..0x825F3340)
	// 825F32D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F32D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F32D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F32DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F32E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F32E4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F32E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F32EC: 390BACA0  addi r8, r11, -0x5360
	ctx.r[8].s64 = ctx.r[11].s64 + -21344;
	// 825F32F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F32F4: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 825F32F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F32FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3308: 386ACDA4  addi r3, r10, -0x325c
	ctx.r[3].s64 = ctx.r[10].s64 + -12892;
	// 825F330C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F331C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F332C: 4BE73AF5  bl 0x82466e20
	ctx.lr = 0x825F3330;
	sub_82466E20(ctx, base);
	// 825F3330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3340 size=112
    let mut pc: u32 = 0x825F3340;
    'dispatch: loop {
        match pc {
            0x825F3340 => {
    //   block [0x825F3340..0x825F33B0)
	// 825F3340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F334C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3350: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3354: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F335C: 390BACE8  addi r8, r11, -0x5318
	ctx.r[8].s64 = ctx.r[11].s64 + -21272;
	// 825F3360: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F3364: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 825F3368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F336C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3378: 386ACDD4  addi r3, r10, -0x322c
	ctx.r[3].s64 = ctx.r[10].s64 + -12844;
	// 825F337C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F338C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F339C: 4BE73A85  bl 0x82466e20
	ctx.lr = 0x825F33A0;
	sub_82466E20(ctx, base);
	// 825F33A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F33A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F33A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F33AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F33B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F33B0 size=112
    let mut pc: u32 = 0x825F33B0;
    'dispatch: loop {
        match pc {
            0x825F33B0 => {
    //   block [0x825F33B0..0x825F3420)
	// 825F33B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F33B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F33B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F33BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F33C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F33C4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F33C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F33CC: 390BAD30  addi r8, r11, -0x52d0
	ctx.r[8].s64 = ctx.r[11].s64 + -21200;
	// 825F33D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F33D4: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 825F33D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F33DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F33E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F33E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F33E8: 386ACE04  addi r3, r10, -0x31fc
	ctx.r[3].s64 = ctx.r[10].s64 + -12796;
	// 825F33EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F33F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F33F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F33F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F33FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F340C: 4BE73A15  bl 0x82466e20
	ctx.lr = 0x825F3410;
	sub_82466E20(ctx, base);
	// 825F3410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3420 size=112
    let mut pc: u32 = 0x825F3420;
    'dispatch: loop {
        match pc {
            0x825F3420 => {
    //   block [0x825F3420..0x825F3490)
	// 825F3420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F342C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3430: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3434: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F343C: 390BAD48  addi r8, r11, -0x52b8
	ctx.r[8].s64 = ctx.r[11].s64 + -21176;
	// 825F3440: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3444: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 825F3448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F344C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3458: 386ACE34  addi r3, r10, -0x31cc
	ctx.r[3].s64 = ctx.r[10].s64 + -12748;
	// 825F345C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F346C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F347C: 4BE739A5  bl 0x82466e20
	ctx.lr = 0x825F3480;
	sub_82466E20(ctx, base);
	// 825F3480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3490 size=116
    let mut pc: u32 = 0x825F3490;
    'dispatch: loop {
        match pc {
            0x825F3490 => {
    //   block [0x825F3490..0x825F3504)
	// 825F3490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F349C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F34A0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F34A4: 390AAD78  addi r8, r10, -0x5288
	ctx.r[8].s64 = ctx.r[10].s64 + -21128;
	// 825F34A8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F34AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F34B0: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F34B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F34B8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F34BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F34C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F34C4: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 825F34C8: 396B1C50  addi r11, r11, 0x1c50
	ctx.r[11].s64 = ctx.r[11].s64 + 7248;
	// 825F34CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F34D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F34D4: 386ACE64  addi r3, r10, -0x319c
	ctx.r[3].s64 = ctx.r[10].s64 + -12700;
	// 825F34D8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F34DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F34E0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F34E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F34E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F34EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F34F0: 4BE73931  bl 0x82466e20
	ctx.lr = 0x825F34F4;
	sub_82466E20(ctx, base);
	// 825F34F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F34F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F34FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3508 size=116
    let mut pc: u32 = 0x825F3508;
    'dispatch: loop {
        match pc {
            0x825F3508 => {
    //   block [0x825F3508..0x825F357C)
	// 825F3508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F350C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3514: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3518: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F351C: 390AADF0  addi r8, r10, -0x5210
	ctx.r[8].s64 = ctx.r[10].s64 + -21008;
	// 825F3520: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3524: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F3528: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F352C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3530: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F353C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 825F3540: 396B1C68  addi r11, r11, 0x1c68
	ctx.r[11].s64 = ctx.r[11].s64 + 7272;
	// 825F3544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3548: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F354C: 386ACE94  addi r3, r10, -0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + -12652;
	// 825F3550: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F3554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3558: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F355C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3568: 4BE738B9  bl 0x82466e20
	ctx.lr = 0x825F356C;
	sub_82466E20(ctx, base);
	// 825F356C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F3580 size=24
    let mut pc: u32 = 0x825F3580;
    'dispatch: loop {
        match pc {
            0x825F3580 => {
    //   block [0x825F3580..0x825F3598)
	// 825F3580: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3584: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3588: 394AEDC8  addi r10, r10, -0x1238
	ctx.r[10].s64 = ctx.r[10].s64 + -4664;
	// 825F358C: 816BAC9C  lwz r11, -0x5364(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21348 as u32) ) } as u64;
	// 825F3590: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825F3594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3598 size=116
    let mut pc: u32 = 0x825F3598;
    'dispatch: loop {
        match pc {
            0x825F3598 => {
    //   block [0x825F3598..0x825F360C)
	// 825F3598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F35A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F35A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F35A8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F35AC: 392B1C94  addi r9, r11, 0x1c94
	ctx.r[9].s64 = ctx.r[11].s64 + 7316;
	// 825F35B0: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F35B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F35B8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F35BC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825F35C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F35C4: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 825F35C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F35CC: 396BEDC8  addi r11, r11, -0x1238
	ctx.r[11].s64 = ctx.r[11].s64 + -4664;
	// 825F35D0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F35D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F35D8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F35DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F35E0: 386ACEC4  addi r3, r10, -0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + -12604;
	// 825F35E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F35E8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F35EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F35F0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F35F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F35F8: 4BE73829  bl 0x82466e20
	ctx.lr = 0x825F35FC;
	sub_82466E20(ctx, base);
	// 825F35FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3610 size=112
    let mut pc: u32 = 0x825F3610;
    'dispatch: loop {
        match pc {
            0x825F3610 => {
    //   block [0x825F3610..0x825F3680)
	// 825F3610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F361C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3620: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3624: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F362C: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 825F3630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F3634: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 825F3638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F363C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3648: 386ACEF4  addi r3, r10, -0x310c
	ctx.r[3].s64 = ctx.r[10].s64 + -12556;
	// 825F364C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F365C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F366C: 4BE737B5  bl 0x82466e20
	ctx.lr = 0x825F3670;
	sub_82466E20(ctx, base);
	// 825F3670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F367C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3680 size=112
    let mut pc: u32 = 0x825F3680;
    'dispatch: loop {
        match pc {
            0x825F3680 => {
    //   block [0x825F3680..0x825F36F0)
	// 825F3680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F368C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3690: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3694: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F369C: 390BAEE0  addi r8, r11, -0x5120
	ctx.r[8].s64 = ctx.r[11].s64 + -20768;
	// 825F36A0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F36A4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 825F36A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F36AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F36B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F36B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F36B8: 386ACF24  addi r3, r10, -0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12508;
	// 825F36BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F36C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F36C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F36C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F36CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F36D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F36D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F36D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F36DC: 4BE73745  bl 0x82466e20
	ctx.lr = 0x825F36E0;
	sub_82466E20(ctx, base);
	// 825F36E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F36E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F36E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F36EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F36F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F36F0 size=112
    let mut pc: u32 = 0x825F36F0;
    'dispatch: loop {
        match pc {
            0x825F36F0 => {
    //   block [0x825F36F0..0x825F3760)
	// 825F36F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F36F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F36F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F36FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3700: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3704: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F370C: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 825F3710: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F3714: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 825F3718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F371C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3728: 386ACF54  addi r3, r10, -0x30ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12460;
	// 825F372C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F373C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F374C: 4BE736D5  bl 0x82466e20
	ctx.lr = 0x825F3750;
	sub_82466E20(ctx, base);
	// 825F3750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F375C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3760 size=112
    let mut pc: u32 = 0x825F3760;
    'dispatch: loop {
        match pc {
            0x825F3760 => {
    //   block [0x825F3760..0x825F37D0)
	// 825F3760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F376C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3770: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3774: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F377C: 390BB000  addi r8, r11, -0x5000
	ctx.r[8].s64 = ctx.r[11].s64 + -20480;
	// 825F3780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F3784: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 825F3788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F378C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3798: 386ACF84  addi r3, r10, -0x307c
	ctx.r[3].s64 = ctx.r[10].s64 + -12412;
	// 825F379C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F37A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F37A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F37A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F37AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F37B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F37B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F37B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F37BC: 4BE73665  bl 0x82466e20
	ctx.lr = 0x825F37C0;
	sub_82466E20(ctx, base);
	// 825F37C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F37C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F37C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F37CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F37D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F37D0 size=112
    let mut pc: u32 = 0x825F37D0;
    'dispatch: loop {
        match pc {
            0x825F37D0 => {
    //   block [0x825F37D0..0x825F3840)
	// 825F37D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F37D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F37D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F37DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F37E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F37E4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F37E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F37EC: 390BB048  addi r8, r11, -0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -20408;
	// 825F37F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F37F4: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 825F37F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F37FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3808: 386ACFB4  addi r3, r10, -0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + -12364;
	// 825F380C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F381C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F382C: 4BE735F5  bl 0x82466e20
	ctx.lr = 0x825F3830;
	sub_82466E20(ctx, base);
	// 825F3830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F383C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3840 size=112
    let mut pc: u32 = 0x825F3840;
    'dispatch: loop {
        match pc {
            0x825F3840 => {
    //   block [0x825F3840..0x825F38B0)
	// 825F3840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F384C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3850: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3854: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F385C: 390BB0D8  addi r8, r11, -0x4f28
	ctx.r[8].s64 = ctx.r[11].s64 + -20264;
	// 825F3860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F3864: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 825F3868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F386C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3878: 386ACFE4  addi r3, r10, -0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + -12316;
	// 825F387C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F388C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F389C: 4BE73585  bl 0x82466e20
	ctx.lr = 0x825F38A0;
	sub_82466E20(ctx, base);
	// 825F38A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F38A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F38A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F38AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F38B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F38B0 size=112
    let mut pc: u32 = 0x825F38B0;
    'dispatch: loop {
        match pc {
            0x825F38B0 => {
    //   block [0x825F38B0..0x825F3920)
	// 825F38B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F38B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F38B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F38BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F38C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F38C4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F38C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F38CC: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 825F38D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F38D4: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 825F38D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F38DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F38E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F38E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F38E8: 386AD014  addi r3, r10, -0x2fec
	ctx.r[3].s64 = ctx.r[10].s64 + -12268;
	// 825F38EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F38F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F38F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F38F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F38FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F390C: 4BE73515  bl 0x82466e20
	ctx.lr = 0x825F3910;
	sub_82466E20(ctx, base);
	// 825F3910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F391C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3920 size=112
    let mut pc: u32 = 0x825F3920;
    'dispatch: loop {
        match pc {
            0x825F3920 => {
    //   block [0x825F3920..0x825F3990)
	// 825F3920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F392C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3930: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3934: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F3938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F393C: 390BB198  addi r8, r11, -0x4e68
	ctx.r[8].s64 = ctx.r[11].s64 + -20072;
	// 825F3940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3944: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 825F3948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F394C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3958: 386AD044  addi r3, r10, -0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -12220;
	// 825F395C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F396C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F397C: 4BE734A5  bl 0x82466e20
	ctx.lr = 0x825F3980;
	sub_82466E20(ctx, base);
	// 825F3980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F398C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3990 size=112
    let mut pc: u32 = 0x825F3990;
    'dispatch: loop {
        match pc {
            0x825F3990 => {
    //   block [0x825F3990..0x825F3A00)
	// 825F3990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F399C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F39A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F39A4: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F39A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F39AC: 390BB1C8  addi r8, r11, -0x4e38
	ctx.r[8].s64 = ctx.r[11].s64 + -20024;
	// 825F39B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F39B4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 825F39B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F39BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F39C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F39C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F39C8: 386AD074  addi r3, r10, -0x2f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -12172;
	// 825F39CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F39D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F39D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F39D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F39DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F39E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F39E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F39E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F39EC: 4BE73435  bl 0x82466e20
	ctx.lr = 0x825F39F0;
	sub_82466E20(ctx, base);
	// 825F39F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F39F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F39F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F39FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3A00 size=100
    let mut pc: u32 = 0x825F3A00;
    'dispatch: loop {
        match pc {
            0x825F3A00 => {
    //   block [0x825F3A00..0x825F3A64)
	// 825F3A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3A14: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F3A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3A20: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 825F3A24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3A34: 386AD0A4  addi r3, r10, -0x2f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -12124;
	// 825F3A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3A3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3A40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F3A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3A48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F3A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3A50: 4BE733D1  bl 0x82466e20
	ctx.lr = 0x825F3A54;
	sub_82466E20(ctx, base);
	// 825F3A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3A68 size=112
    let mut pc: u32 = 0x825F3A68;
    'dispatch: loop {
        match pc {
            0x825F3A68 => {
    //   block [0x825F3A68..0x825F3AD8)
	// 825F3A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3A7C: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F3A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3A84: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 825F3A88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3A8C: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 825F3A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3A94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3AA0: 386AD0D4  addi r3, r10, -0x2f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -12076;
	// 825F3AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3AC4: 4BE7335D  bl 0x82466e20
	ctx.lr = 0x825F3AC8;
	sub_82466E20(ctx, base);
	// 825F3AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3AD8 size=112
    let mut pc: u32 = 0x825F3AD8;
    'dispatch: loop {
        match pc {
            0x825F3AD8 => {
    //   block [0x825F3AD8..0x825F3B48)
	// 825F3AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3AE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3AE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3AEC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F3AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3AF4: 390BB210  addi r8, r11, -0x4df0
	ctx.r[8].s64 = ctx.r[11].s64 + -19952;
	// 825F3AF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F3AFC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 825F3B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3B04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3B10: 386AD104  addi r3, r10, -0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + -12028;
	// 825F3B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3B34: 4BE732ED  bl 0x82466e20
	ctx.lr = 0x825F3B38;
	sub_82466E20(ctx, base);
	// 825F3B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3B48 size=112
    let mut pc: u32 = 0x825F3B48;
    'dispatch: loop {
        match pc {
            0x825F3B48 => {
    //   block [0x825F3B48..0x825F3BB8)
	// 825F3B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3B58: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3B5C: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F3B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3B64: 390BB270  addi r8, r11, -0x4d90
	ctx.r[8].s64 = ctx.r[11].s64 + -19856;
	// 825F3B68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3B6C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 825F3B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3B74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3B80: 386AD134  addi r3, r10, -0x2ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -11980;
	// 825F3B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3BA4: 4BE7327D  bl 0x82466e20
	ctx.lr = 0x825F3BA8;
	sub_82466E20(ctx, base);
	// 825F3BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3BB8 size=112
    let mut pc: u32 = 0x825F3BB8;
    'dispatch: loop {
        match pc {
            0x825F3BB8 => {
    //   block [0x825F3BB8..0x825F3C28)
	// 825F3BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3BC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3BCC: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F3BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3BD4: 390BB288  addi r8, r11, -0x4d78
	ctx.r[8].s64 = ctx.r[11].s64 + -19832;
	// 825F3BD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3BDC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 825F3BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3BE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3BF0: 386AD164  addi r3, r10, -0x2e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11932;
	// 825F3BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3C14: 4BE7320D  bl 0x82466e20
	ctx.lr = 0x825F3C18;
	sub_82466E20(ctx, base);
	// 825F3C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3C28 size=112
    let mut pc: u32 = 0x825F3C28;
    'dispatch: loop {
        match pc {
            0x825F3C28 => {
    //   block [0x825F3C28..0x825F3C98)
	// 825F3C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3C38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3C3C: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F3C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3C44: 390BB2B8  addi r8, r11, -0x4d48
	ctx.r[8].s64 = ctx.r[11].s64 + -19784;
	// 825F3C48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3C4C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 825F3C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3C54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3C60: 386AD194  addi r3, r10, -0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11884;
	// 825F3C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3C84: 4BE7319D  bl 0x82466e20
	ctx.lr = 0x825F3C88;
	sub_82466E20(ctx, base);
	// 825F3C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F3C98 size=24
    let mut pc: u32 = 0x825F3C98;
    'dispatch: loop {
        match pc {
            0x825F3C98 => {
    //   block [0x825F3C98..0x825F3CB0)
	// 825F3C98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3C9C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3CA0: 394AEE70  addi r10, r10, -0x1190
	ctx.r[10].s64 = ctx.r[10].s64 + -4496;
	// 825F3CA4: 816BB2D0  lwz r11, -0x4d30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19760 as u32) ) } as u64;
	// 825F3CA8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F3CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3CB0 size=112
    let mut pc: u32 = 0x825F3CB0;
    'dispatch: loop {
        match pc {
            0x825F3CB0 => {
    //   block [0x825F3CB0..0x825F3D20)
	// 825F3CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3CBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3CC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3CC4: 392A1CF0  addi r9, r10, 0x1cf0
	ctx.r[9].s64 = ctx.r[10].s64 + 7408;
	// 825F3CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3CCC: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 825F3CD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F3CD4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 825F3CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3CDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3CE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3CE8: 386AD1C4  addi r3, r10, -0x2e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11836;
	// 825F3CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3CF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F3CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3D0C: 4BE73115  bl 0x82466e20
	ctx.lr = 0x825F3D10;
	sub_82466E20(ctx, base);
	// 825F3D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3D20 size=108
    let mut pc: u32 = 0x825F3D20;
    'dispatch: loop {
        match pc {
            0x825F3D20 => {
    //   block [0x825F3D20..0x825F3D8C)
	// 825F3D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3D2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3D34: 38EBB2D4  addi r7, r11, -0x4d2c
	ctx.r[7].s64 = ctx.r[11].s64 + -19756;
	// 825F3D38: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F3D3C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 825F3D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F3D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3D50: 386AD1F4  addi r3, r10, -0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11788;
	// 825F3D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F3D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3D78: 4BE730A9  bl 0x82466e20
	ctx.lr = 0x825F3D7C;
	sub_82466E20(ctx, base);
	// 825F3D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3D90 size=108
    let mut pc: u32 = 0x825F3D90;
    'dispatch: loop {
        match pc {
            0x825F3D90 => {
    //   block [0x825F3D90..0x825F3DFC)
	// 825F3D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3D9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3DA4: 38EBB2F0  addi r7, r11, -0x4d10
	ctx.r[7].s64 = ctx.r[11].s64 + -19728;
	// 825F3DA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F3DAC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 825F3DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3DB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F3DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3DC0: 386AD224  addi r3, r10, -0x2ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -11740;
	// 825F3DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F3DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3DE8: 4BE73039  bl 0x82466e20
	ctx.lr = 0x825F3DEC;
	sub_82466E20(ctx, base);
	// 825F3DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3E00 size=116
    let mut pc: u32 = 0x825F3E00;
    'dispatch: loop {
        match pc {
            0x825F3E00 => {
    //   block [0x825F3E00..0x825F3E74)
	// 825F3E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3E0C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3E14: 390BB338  addi r8, r11, -0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -19656;
	// 825F3E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3E1C: 392A1DA8  addi r9, r10, 0x1da8
	ctx.r[9].s64 = ctx.r[10].s64 + 7592;
	// 825F3E20: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3E24: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F3E28: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F3E2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3E34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3E44: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F3E48: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 825F3E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3E50: 386BD254  addi r3, r11, -0x2dac
	ctx.r[3].s64 = ctx.r[11].s64 + -11692;
	// 825F3E54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F3E58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3E60: 4BE72FC1  bl 0x82466e20
	ctx.lr = 0x825F3E64;
	sub_82466E20(ctx, base);
	// 825F3E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F3E78 size=24
    let mut pc: u32 = 0x825F3E78;
    'dispatch: loop {
        match pc {
            0x825F3E78 => {
    //   block [0x825F3E78..0x825F3E90)
	// 825F3E78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3E7C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3E80: 394AEEB8  addi r10, r10, -0x1148
	ctx.r[10].s64 = ctx.r[10].s64 + -4424;
	// 825F3E84: 816BB350  lwz r11, -0x4cb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19632 as u32) ) } as u64;
	// 825F3E88: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F3E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3E90 size=116
    let mut pc: u32 = 0x825F3E90;
    'dispatch: loop {
        match pc {
            0x825F3E90 => {
    //   block [0x825F3E90..0x825F3F04)
	// 825F3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3E9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3EA4: 390BEEB8  addi r8, r11, -0x1148
	ctx.r[8].s64 = ctx.r[11].s64 + -4424;
	// 825F3EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3EAC: 392A1E04  addi r9, r10, 0x1e04
	ctx.r[9].s64 = ctx.r[10].s64 + 7684;
	// 825F3EB0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3EB4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825F3EB8: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F3EBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3EC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3ED4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F3ED8: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 825F3EDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3EE0: 386BD284  addi r3, r11, -0x2d7c
	ctx.r[3].s64 = ctx.r[11].s64 + -11644;
	// 825F3EE4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825F3EE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3EF0: 4BE72F31  bl 0x82466e20
	ctx.lr = 0x825F3EF4;
	sub_82466E20(ctx, base);
	// 825F3EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3F08 size=108
    let mut pc: u32 = 0x825F3F08;
    'dispatch: loop {
        match pc {
            0x825F3F08 => {
    //   block [0x825F3F08..0x825F3F74)
	// 825F3F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3F14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3F1C: 38EBB35C  addi r7, r11, -0x4ca4
	ctx.r[7].s64 = ctx.r[11].s64 + -19620;
	// 825F3F20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F3F24: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 825F3F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3F2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F3F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3F38: 386AD2B4  addi r3, r10, -0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11596;
	// 825F3F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F3F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3F60: 4BE72EC1  bl 0x82466e20
	ctx.lr = 0x825F3F64;
	sub_82466E20(ctx, base);
	// 825F3F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3F78 size=112
    let mut pc: u32 = 0x825F3F78;
    'dispatch: loop {
        match pc {
            0x825F3F78 => {
    //   block [0x825F3F78..0x825F3FE8)
	// 825F3F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3F84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3F88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3F8C: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F3F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3F94: 390BB38C  addi r8, r11, -0x4c74
	ctx.r[8].s64 = ctx.r[11].s64 + -19572;
	// 825F3F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3F9C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 825F3FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3FA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3FB0: 386AD2E4  addi r3, r10, -0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11548;
	// 825F3FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3FD4: 4BE72E4D  bl 0x82466e20
	ctx.lr = 0x825F3FD8;
	sub_82466E20(ctx, base);
	// 825F3FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3FE8 size=112
    let mut pc: u32 = 0x825F3FE8;
    'dispatch: loop {
        match pc {
            0x825F3FE8 => {
    //   block [0x825F3FE8..0x825F4058)
	// 825F3FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3FF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3FF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3FFC: 392A1E48  addi r9, r10, 0x1e48
	ctx.r[9].s64 = ctx.r[10].s64 + 7752;
	// 825F4000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4004: 390BB3A8  addi r8, r11, -0x4c58
	ctx.r[8].s64 = ctx.r[11].s64 + -19544;
	// 825F4008: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F400C: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 825F4010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F401C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4020: 386AD314  addi r3, r10, -0x2cec
	ctx.r[3].s64 = ctx.r[10].s64 + -11500;
	// 825F4024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4028: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F402C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F403C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4044: 4BE72DDD  bl 0x82466e20
	ctx.lr = 0x825F4048;
	sub_82466E20(ctx, base);
	// 825F4048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F404C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4058 size=112
    let mut pc: u32 = 0x825F4058;
    'dispatch: loop {
        match pc {
            0x825F4058 => {
    //   block [0x825F4058..0x825F40C8)
	// 825F4058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4068: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F406C: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4074: 390BB3F0  addi r8, r11, -0x4c10
	ctx.r[8].s64 = ctx.r[11].s64 + -19472;
	// 825F4078: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F407C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 825F4080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4084: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F408C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4090: 386AD344  addi r3, r10, -0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11452;
	// 825F4094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F409C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F40A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F40A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F40A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F40AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F40B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F40B4: 4BE72D6D  bl 0x82466e20
	ctx.lr = 0x825F40B8;
	sub_82466E20(ctx, base);
	// 825F40B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F40BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F40C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F40C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F40C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F40C8 size=112
    let mut pc: u32 = 0x825F40C8;
    'dispatch: loop {
        match pc {
            0x825F40C8 => {
    //   block [0x825F40C8..0x825F4138)
	// 825F40C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F40CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F40D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F40D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F40D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F40DC: 392A1E74  addi r9, r10, 0x1e74
	ctx.r[9].s64 = ctx.r[10].s64 + 7796;
	// 825F40E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F40E4: 390BB410  addi r8, r11, -0x4bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -19440;
	// 825F40E8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F40EC: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 825F40F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F40F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F40F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F40FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4100: 386AD374  addi r3, r10, -0x2c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11404;
	// 825F4104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F410C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F411C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4124: 4BE72CFD  bl 0x82466e20
	ctx.lr = 0x825F4128;
	sub_82466E20(ctx, base);
	// 825F4128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F412C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4138 size=112
    let mut pc: u32 = 0x825F4138;
    'dispatch: loop {
        match pc {
            0x825F4138 => {
    //   block [0x825F4138..0x825F41A8)
	// 825F4138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4144: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4148: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F414C: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4154: 390BB4A0  addi r8, r11, -0x4b60
	ctx.r[8].s64 = ctx.r[11].s64 + -19296;
	// 825F4158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F415C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 825F4160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F416C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4170: 386AD3A4  addi r3, r10, -0x2c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11356;
	// 825F4174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F417C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F418C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4194: 4BE72C8D  bl 0x82466e20
	ctx.lr = 0x825F4198;
	sub_82466E20(ctx, base);
	// 825F4198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F419C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F41A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F41A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F41A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F41A8 size=112
    let mut pc: u32 = 0x825F41A8;
    'dispatch: loop {
        match pc {
            0x825F41A8 => {
    //   block [0x825F41A8..0x825F4218)
	// 825F41A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F41AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F41B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F41B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F41B8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F41BC: 38AAD404  addi r5, r10, -0x2bfc
	ctx.r[5].s64 = ctx.r[10].s64 + -11260;
	// 825F41C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F41C4: 390BB4B8  addi r8, r11, -0x4b48
	ctx.r[8].s64 = ctx.r[11].s64 + -19272;
	// 825F41C8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F41CC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 825F41D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F41D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F41D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F41DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F41E0: 386AD3D4  addi r3, r10, -0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	// 825F41E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F41E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F41EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F41F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F41F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F41F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F41FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4204: 4BE72C1D  bl 0x82466e20
	ctx.lr = 0x825F4208;
	sub_82466E20(ctx, base);
	// 825F4208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F420C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4218 size=100
    let mut pc: u32 = 0x825F4218;
    'dispatch: loop {
        match pc {
            0x825F4218 => {
    //   block [0x825F4218..0x825F427C)
	// 825F4218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4224: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F422C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F4230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4238: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 825F423C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F424C: 386AD404  addi r3, r10, -0x2bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11260;
	// 825F4250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4258: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F425C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4260: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F4264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4268: 4BE72BB9  bl 0x82466e20
	ctx.lr = 0x825F426C;
	sub_82466E20(ctx, base);
	// 825F426C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F4280 size=24
    let mut pc: u32 = 0x825F4280;
    'dispatch: loop {
        match pc {
            0x825F4280 => {
    //   block [0x825F4280..0x825F4298)
	// 825F4280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4284: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4288: 394AEF90  addi r10, r10, -0x1070
	ctx.r[10].s64 = ctx.r[10].s64 + -4208;
	// 825F428C: 816BB40C  lwz r11, -0x4bf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19444 as u32) ) } as u64;
	// 825F4290: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825F4294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4298 size=116
    let mut pc: u32 = 0x825F4298;
    'dispatch: loop {
        match pc {
            0x825F4298 => {
    //   block [0x825F4298..0x825F430C)
	// 825F4298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F42A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F42A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F42A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F42AC: 390BEF90  addi r8, r11, -0x1070
	ctx.r[8].s64 = ctx.r[11].s64 + -4208;
	// 825F42B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F42B4: 392A1EB0  addi r9, r10, 0x1eb0
	ctx.r[9].s64 = ctx.r[10].s64 + 7856;
	// 825F42B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F42BC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F42C0: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F42C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F42C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F42CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F42D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F42D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F42D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F42DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F42E0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 825F42E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F42E8: 386BD434  addi r3, r11, -0x2bcc
	ctx.r[3].s64 = ctx.r[11].s64 + -11212;
	// 825F42EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F42F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F42F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F42F8: 4BE72B29  bl 0x82466e20
	ctx.lr = 0x825F42FC;
	sub_82466E20(ctx, base);
	// 825F42FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4310 size=108
    let mut pc: u32 = 0x825F4310;
    'dispatch: loop {
        match pc {
            0x825F4310 => {
    //   block [0x825F4310..0x825F437C)
	// 825F4310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F431C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4324: 38EBB530  addi r7, r11, -0x4ad0
	ctx.r[7].s64 = ctx.r[11].s64 + -19152;
	// 825F4328: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F432C: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 825F4330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F433C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4340: 386AD464  addi r3, r10, -0x2b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11164;
	// 825F4344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F434C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F435C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4368: 4BE72AB9  bl 0x82466e20
	ctx.lr = 0x825F436C;
	sub_82466E20(ctx, base);
	// 825F436C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4380 size=112
    let mut pc: u32 = 0x825F4380;
    'dispatch: loop {
        match pc {
            0x825F4380 => {
    //   block [0x825F4380..0x825F43F0)
	// 825F4380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F438C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4390: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4394: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F439C: 390BB560  addi r8, r11, -0x4aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -19104;
	// 825F43A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F43A4: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 825F43A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F43AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F43B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F43B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F43B8: 386AD494  addi r3, r10, -0x2b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11116;
	// 825F43BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F43C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F43C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F43C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F43CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F43D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F43D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F43D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F43DC: 4BE72A45  bl 0x82466e20
	ctx.lr = 0x825F43E0;
	sub_82466E20(ctx, base);
	// 825F43E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F43E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F43E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F43EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F43F0 size=112
    let mut pc: u32 = 0x825F43F0;
    'dispatch: loop {
        match pc {
            0x825F43F0 => {
    //   block [0x825F43F0..0x825F4460)
	// 825F43F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F43F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F43F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F43FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4400: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4404: 392A1ED4  addi r9, r10, 0x1ed4
	ctx.r[9].s64 = ctx.r[10].s64 + 7892;
	// 825F4408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F440C: 390BB580  addi r8, r11, -0x4a80
	ctx.r[8].s64 = ctx.r[11].s64 + -19072;
	// 825F4410: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F4414: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 825F4418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F441C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4428: 386AD4C4  addi r3, r10, -0x2b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11068;
	// 825F442C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4430: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F4434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F443C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F444C: 4BE729D5  bl 0x82466e20
	ctx.lr = 0x825F4450;
	sub_82466E20(ctx, base);
	// 825F4450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F445C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4460 size=112
    let mut pc: u32 = 0x825F4460;
    'dispatch: loop {
        match pc {
            0x825F4460 => {
    //   block [0x825F4460..0x825F44D0)
	// 825F4460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F446C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4470: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4474: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F447C: 390BB628  addi r8, r11, -0x49d8
	ctx.r[8].s64 = ctx.r[11].s64 + -18904;
	// 825F4480: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F4484: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 825F4488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F448C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4498: 386AD4F4  addi r3, r10, -0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11020;
	// 825F449C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F44A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F44A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F44A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F44AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F44B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F44B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F44B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F44BC: 4BE72965  bl 0x82466e20
	ctx.lr = 0x825F44C0;
	sub_82466E20(ctx, base);
	// 825F44C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F44C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F44C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F44CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F44D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F44D0 size=108
    let mut pc: u32 = 0x825F44D0;
    'dispatch: loop {
        match pc {
            0x825F44D0 => {
    //   block [0x825F44D0..0x825F453C)
	// 825F44D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F44D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F44D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F44DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F44E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F44E4: 38EBB640  addi r7, r11, -0x49c0
	ctx.r[7].s64 = ctx.r[11].s64 + -18880;
	// 825F44E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F44EC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 825F44F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F44F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F44F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F44FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4500: 386AD524  addi r3, r10, -0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + -10972;
	// 825F4504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F450C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F451C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4528: 4BE728F9  bl 0x82466e20
	ctx.lr = 0x825F452C;
	sub_82466E20(ctx, base);
	// 825F452C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4540 size=112
    let mut pc: u32 = 0x825F4540;
    'dispatch: loop {
        match pc {
            0x825F4540 => {
    //   block [0x825F4540..0x825F45B0)
	// 825F4540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F454C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4550: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4554: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F455C: 390BB670  addi r8, r11, -0x4990
	ctx.r[8].s64 = ctx.r[11].s64 + -18832;
	// 825F4560: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F4564: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 825F4568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F456C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4578: 386AD554  addi r3, r10, -0x2aac
	ctx.r[3].s64 = ctx.r[10].s64 + -10924;
	// 825F457C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F458C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F459C: 4BE72885  bl 0x82466e20
	ctx.lr = 0x825F45A0;
	sub_82466E20(ctx, base);
	// 825F45A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F45A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F45A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F45AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F45B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F45B0 size=112
    let mut pc: u32 = 0x825F45B0;
    'dispatch: loop {
        match pc {
            0x825F45B0 => {
    //   block [0x825F45B0..0x825F4620)
	// 825F45B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F45B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F45B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F45BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F45C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F45C4: 392A1F08  addi r9, r10, 0x1f08
	ctx.r[9].s64 = ctx.r[10].s64 + 7944;
	// 825F45C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F45CC: 390BB688  addi r8, r11, -0x4978
	ctx.r[8].s64 = ctx.r[11].s64 + -18808;
	// 825F45D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F45D4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 825F45D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F45DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F45E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F45E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F45E8: 386AD584  addi r3, r10, -0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -10876;
	// 825F45EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F45F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F45F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F45F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F45FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F460C: 4BE72815  bl 0x82466e20
	ctx.lr = 0x825F4610;
	sub_82466E20(ctx, base);
	// 825F4610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F461C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4620 size=112
    let mut pc: u32 = 0x825F4620;
    'dispatch: loop {
        match pc {
            0x825F4620 => {
    //   block [0x825F4620..0x825F4690)
	// 825F4620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F462C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4630: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4634: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F463C: 390BB730  addi r8, r11, -0x48d0
	ctx.r[8].s64 = ctx.r[11].s64 + -18640;
	// 825F4640: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F4644: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 825F4648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F464C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4658: 386AD5B4  addi r3, r10, -0x2a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -10828;
	// 825F465C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F466C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F467C: 4BE727A5  bl 0x82466e20
	ctx.lr = 0x825F4680;
	sub_82466E20(ctx, base);
	// 825F4680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F468C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4690 size=112
    let mut pc: u32 = 0x825F4690;
    'dispatch: loop {
        match pc {
            0x825F4690 => {
    //   block [0x825F4690..0x825F4700)
	// 825F4690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F469C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F46A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F46A4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F46A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F46AC: 390BB778  addi r8, r11, -0x4888
	ctx.r[8].s64 = ctx.r[11].s64 + -18568;
	// 825F46B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825F46B4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 825F46B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F46BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F46C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F46C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F46C8: 386AD5E4  addi r3, r10, -0x2a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -10780;
	// 825F46CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F46D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F46D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F46D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F46DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F46E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F46E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F46E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F46EC: 4BE72735  bl 0x82466e20
	ctx.lr = 0x825F46F0;
	sub_82466E20(ctx, base);
	// 825F46F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F46F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F46F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F46FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4700 size=100
    let mut pc: u32 = 0x825F4700;
    'dispatch: loop {
        match pc {
            0x825F4700 => {
    //   block [0x825F4700..0x825F4764)
	// 825F4700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F470C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4714: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F471C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4720: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 825F4724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F472C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4734: 386AD614  addi r3, r10, -0x29ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10732;
	// 825F4738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F473C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F4744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F474C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4750: 4BE726D1  bl 0x82466e20
	ctx.lr = 0x825F4754;
	sub_82466E20(ctx, base);
	// 825F4754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F475C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4768 size=112
    let mut pc: u32 = 0x825F4768;
    'dispatch: loop {
        match pc {
            0x825F4768 => {
    //   block [0x825F4768..0x825F47D8)
	// 825F4768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4774: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4778: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F477C: 38AAD284  addi r5, r10, -0x2d7c
	ctx.r[5].s64 = ctx.r[10].s64 + -11644;
	// 825F4780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4784: 390BB838  addi r8, r11, -0x47c8
	ctx.r[8].s64 = ctx.r[11].s64 + -18376;
	// 825F4788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F478C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 825F4790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4794: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F479C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F47A0: 386AD644  addi r3, r10, -0x29bc
	ctx.r[3].s64 = ctx.r[10].s64 + -10684;
	// 825F47A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F47A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F47AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F47B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F47B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F47B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F47BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F47C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F47C4: 4BE7265D  bl 0x82466e20
	ctx.lr = 0x825F47C8;
	sub_82466E20(ctx, base);
	// 825F47C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F47CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F47D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F47D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F47D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F47D8 size=112
    let mut pc: u32 = 0x825F47D8;
    'dispatch: loop {
        match pc {
            0x825F47D8 => {
    //   block [0x825F47D8..0x825F4848)
	// 825F47D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F47DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F47E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F47E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F47E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F47EC: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F47F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F47F4: 390BB868  addi r8, r11, -0x4798
	ctx.r[8].s64 = ctx.r[11].s64 + -18328;
	// 825F47F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F47FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 825F4800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4804: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F480C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4810: 386AD674  addi r3, r10, -0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + -10636;
	// 825F4814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F481C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F482C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4834: 4BE725ED  bl 0x82466e20
	ctx.lr = 0x825F4838;
	sub_82466E20(ctx, base);
	// 825F4838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F483C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4848 size=108
    let mut pc: u32 = 0x825F4848;
    'dispatch: loop {
        match pc {
            0x825F4848 => {
    //   block [0x825F4848..0x825F48B4)
	// 825F4848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F484C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4854: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F485C: 38EBB880  addi r7, r11, -0x4780
	ctx.r[7].s64 = ctx.r[11].s64 + -18304;
	// 825F4860: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F4864: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 825F4868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F486C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4878: 386AD6A4  addi r3, r10, -0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	// 825F487C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F488C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F489C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F48A0: 4BE72581  bl 0x82466e20
	ctx.lr = 0x825F48A4;
	sub_82466E20(ctx, base);
	// 825F48A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F48A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F48AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F48B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F48B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F48B8 size=112
    let mut pc: u32 = 0x825F48B8;
    'dispatch: loop {
        match pc {
            0x825F48B8 => {
    //   block [0x825F48B8..0x825F4928)
	// 825F48B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F48BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F48C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F48C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F48C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F48CC: 38AAD614  addi r5, r10, -0x29ec
	ctx.r[5].s64 = ctx.r[10].s64 + -10732;
	// 825F48D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F48D4: 390BB8B0  addi r8, r11, -0x4750
	ctx.r[8].s64 = ctx.r[11].s64 + -18256;
	// 825F48D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F48DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 825F48E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F48E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F48E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F48EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F48F0: 386AD6D4  addi r3, r10, -0x292c
	ctx.r[3].s64 = ctx.r[10].s64 + -10540;
	// 825F48F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F48F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F48FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F490C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4914: 4BE7250D  bl 0x82466e20
	ctx.lr = 0x825F4918;
	sub_82466E20(ctx, base);
	// 825F4918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F491C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4928 size=112
    let mut pc: u32 = 0x825F4928;
    'dispatch: loop {
        match pc {
            0x825F4928 => {
    //   block [0x825F4928..0x825F4998)
	// 825F4928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F492C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4934: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4938: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F493C: 392A1F34  addi r9, r10, 0x1f34
	ctx.r[9].s64 = ctx.r[10].s64 + 7988;
	// 825F4940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4944: 390BB948  addi r8, r11, -0x46b8
	ctx.r[8].s64 = ctx.r[11].s64 + -18104;
	// 825F4948: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F494C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 825F4950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F495C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4960: 386AD704  addi r3, r10, -0x28fc
	ctx.r[3].s64 = ctx.r[10].s64 + -10492;
	// 825F4964: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F496C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F497C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4984: 4BE7249D  bl 0x82466e20
	ctx.lr = 0x825F4988;
	sub_82466E20(ctx, base);
	// 825F4988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F498C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4998 size=112
    let mut pc: u32 = 0x825F4998;
    'dispatch: loop {
        match pc {
            0x825F4998 => {
    //   block [0x825F4998..0x825F4A08)
	// 825F4998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F499C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F49A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F49A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F49A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F49AC: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F49B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F49B4: 390BB990  addi r8, r11, -0x4670
	ctx.r[8].s64 = ctx.r[11].s64 + -18032;
	// 825F49B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F49BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 825F49C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F49C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F49C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F49CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F49D0: 386AD734  addi r3, r10, -0x28cc
	ctx.r[3].s64 = ctx.r[10].s64 + -10444;
	// 825F49D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F49D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F49DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F49E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F49E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F49E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F49EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F49F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F49F4: 4BE7242D  bl 0x82466e20
	ctx.lr = 0x825F49F8;
	sub_82466E20(ctx, base);
	// 825F49F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F49FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4A08 size=108
    let mut pc: u32 = 0x825F4A08;
    'dispatch: loop {
        match pc {
            0x825F4A08 => {
    //   block [0x825F4A08..0x825F4A74)
	// 825F4A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4A14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4A1C: 38EBB9A8  addi r7, r11, -0x4658
	ctx.r[7].s64 = ctx.r[11].s64 + -18008;
	// 825F4A20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F4A24: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 825F4A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4A2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4A30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4A38: 386AD764  addi r3, r10, -0x289c
	ctx.r[3].s64 = ctx.r[10].s64 + -10396;
	// 825F4A3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4A60: 4BE723C1  bl 0x82466e20
	ctx.lr = 0x825F4A64;
	sub_82466E20(ctx, base);
	// 825F4A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4A78 size=116
    let mut pc: u32 = 0x825F4A78;
    'dispatch: loop {
        match pc {
            0x825F4A78 => {
    //   block [0x825F4A78..0x825F4AEC)
	// 825F4A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4A84: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4A88: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825F4A8C: 390ABA38  addi r8, r10, -0x45c8
	ctx.r[8].s64 = ctx.r[10].s64 + -17864;
	// 825F4A90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4A94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F4A98: 38AAD614  addi r5, r10, -0x29ec
	ctx.r[5].s64 = ctx.r[10].s64 + -10732;
	// 825F4A9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4AA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4AAC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 825F4AB0: 396B1F48  addi r11, r11, 0x1f48
	ctx.r[11].s64 = ctx.r[11].s64 + 8008;
	// 825F4AB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4AB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4ABC: 386AD794  addi r3, r10, -0x286c
	ctx.r[3].s64 = ctx.r[10].s64 + -10348;
	// 825F4AC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F4AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4AC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F4ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4AD8: 4BE72349  bl 0x82466e20
	ctx.lr = 0x825F4ADC;
	sub_82466E20(ctx, base);
	// 825F4ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4AF0 size=108
    let mut pc: u32 = 0x825F4AF0;
    'dispatch: loop {
        match pc {
            0x825F4AF0 => {
    //   block [0x825F4AF0..0x825F4B5C)
	// 825F4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4AFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4B04: 38EBBB10  addi r7, r11, -0x44f0
	ctx.r[7].s64 = ctx.r[11].s64 + -17648;
	// 825F4B08: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F4B0C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 825F4B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4B14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4B20: 386AD7C4  addi r3, r10, -0x283c
	ctx.r[3].s64 = ctx.r[10].s64 + -10300;
	// 825F4B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4B48: 4BE722D9  bl 0x82466e20
	ctx.lr = 0x825F4B4C;
	sub_82466E20(ctx, base);
	// 825F4B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4B60 size=112
    let mut pc: u32 = 0x825F4B60;
    'dispatch: loop {
        match pc {
            0x825F4B60 => {
    //   block [0x825F4B60..0x825F4BD0)
	// 825F4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4B6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4B70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4B74: 38AAD614  addi r5, r10, -0x29ec
	ctx.r[5].s64 = ctx.r[10].s64 + -10732;
	// 825F4B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4B7C: 390BBB58  addi r8, r11, -0x44a8
	ctx.r[8].s64 = ctx.r[11].s64 + -17576;
	// 825F4B80: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F4B84: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 825F4B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4B98: 386AD7F4  addi r3, r10, -0x280c
	ctx.r[3].s64 = ctx.r[10].s64 + -10252;
	// 825F4B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4BBC: 4BE72265  bl 0x82466e20
	ctx.lr = 0x825F4BC0;
	sub_82466E20(ctx, base);
	// 825F4BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4BD0 size=112
    let mut pc: u32 = 0x825F4BD0;
    'dispatch: loop {
        match pc {
            0x825F4BD0 => {
    //   block [0x825F4BD0..0x825F4C40)
	// 825F4BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4BDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4BE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4BE4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4BEC: 390BBBD0  addi r8, r11, -0x4430
	ctx.r[8].s64 = ctx.r[11].s64 + -17456;
	// 825F4BF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F4BF4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 825F4BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4BFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4C00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4C08: 386AD824  addi r3, r10, -0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	// 825F4C0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4C2C: 4BE721F5  bl 0x82466e20
	ctx.lr = 0x825F4C30;
	sub_82466E20(ctx, base);
	// 825F4C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4C40 size=108
    let mut pc: u32 = 0x825F4C40;
    'dispatch: loop {
        match pc {
            0x825F4C40 => {
    //   block [0x825F4C40..0x825F4CAC)
	// 825F4C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4C4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4C54: 38EBBC00  addi r7, r11, -0x4400
	ctx.r[7].s64 = ctx.r[11].s64 + -17408;
	// 825F4C58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F4C5C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 825F4C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4C64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4C70: 386AD854  addi r3, r10, -0x27ac
	ctx.r[3].s64 = ctx.r[10].s64 + -10156;
	// 825F4C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4C98: 4BE72189  bl 0x82466e20
	ctx.lr = 0x825F4C9C;
	sub_82466E20(ctx, base);
	// 825F4C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4CB0 size=108
    let mut pc: u32 = 0x825F4CB0;
    'dispatch: loop {
        match pc {
            0x825F4CB0 => {
    //   block [0x825F4CB0..0x825F4D1C)
	// 825F4CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4CBC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4CC4: 38EBBC60  addi r7, r11, -0x43a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17312;
	// 825F4CC8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F4CCC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 825F4CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4CD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4CE0: 386AD884  addi r3, r10, -0x277c
	ctx.r[3].s64 = ctx.r[10].s64 + -10108;
	// 825F4CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4D08: 4BE72119  bl 0x82466e20
	ctx.lr = 0x825F4D0C;
	sub_82466E20(ctx, base);
	// 825F4D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4D20 size=112
    let mut pc: u32 = 0x825F4D20;
    'dispatch: loop {
        match pc {
            0x825F4D20 => {
    //   block [0x825F4D20..0x825F4D90)
	// 825F4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4D2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4D30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4D34: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4D38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4D3C: 390BBCD8  addi r8, r11, -0x4328
	ctx.r[8].s64 = ctx.r[11].s64 + -17192;
	// 825F4D40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F4D44: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 825F4D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4D4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4D50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4D58: 386AD8B4  addi r3, r10, -0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + -10060;
	// 825F4D5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4D7C: 4BE720A5  bl 0x82466e20
	ctx.lr = 0x825F4D80;
	sub_82466E20(ctx, base);
	// 825F4D80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F4D90 size=24
    let mut pc: u32 = 0x825F4D90;
    'dispatch: loop {
        match pc {
            0x825F4D90 => {
    //   block [0x825F4D90..0x825F4DA8)
	// 825F4D90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4D94: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4D98: 394AF008  addi r10, r10, -0xff8
	ctx.r[10].s64 = ctx.r[10].s64 + -4088;
	// 825F4D9C: 816BB944  lwz r11, -0x46bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 825F4DA0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F4DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4DA8 size=116
    let mut pc: u32 = 0x825F4DA8;
    'dispatch: loop {
        match pc {
            0x825F4DA8 => {
    //   block [0x825F4DA8..0x825F4E1C)
	// 825F4DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4DB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4DBC: 390BF008  addi r8, r11, -0xff8
	ctx.r[8].s64 = ctx.r[11].s64 + -4088;
	// 825F4DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4DC4: 392A1FAC  addi r9, r10, 0x1fac
	ctx.r[9].s64 = ctx.r[10].s64 + 8108;
	// 825F4DC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4DCC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F4DD0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F4DD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4DDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4DEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F4DF0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 825F4DF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4DF8: 386BD8E4  addi r3, r11, -0x271c
	ctx.r[3].s64 = ctx.r[11].s64 + -10012;
	// 825F4DFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F4E00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4E08: 4BE72019  bl 0x82466e20
	ctx.lr = 0x825F4E0C;
	sub_82466E20(ctx, base);
	// 825F4E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4E20 size=112
    let mut pc: u32 = 0x825F4E20;
    'dispatch: loop {
        match pc {
            0x825F4E20 => {
    //   block [0x825F4E20..0x825F4E90)
	// 825F4E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4E2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4E30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4E34: 38AAD8E4  addi r5, r10, -0x271c
	ctx.r[5].s64 = ctx.r[10].s64 + -10012;
	// 825F4E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4E3C: 390BBD20  addi r8, r11, -0x42e0
	ctx.r[8].s64 = ctx.r[11].s64 + -17120;
	// 825F4E40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F4E44: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 825F4E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4E4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4E58: 386AD914  addi r3, r10, -0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9964;
	// 825F4E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4E7C: 4BE71FA5  bl 0x82466e20
	ctx.lr = 0x825F4E80;
	sub_82466E20(ctx, base);
	// 825F4E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F4E90 size=24
    let mut pc: u32 = 0x825F4E90;
    'dispatch: loop {
        match pc {
            0x825F4E90 => {
    //   block [0x825F4E90..0x825F4EA8)
	// 825F4E90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4E94: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4E98: 394AF020  addi r10, r10, -0xfe0
	ctx.r[10].s64 = ctx.r[10].s64 + -4064;
	// 825F4E9C: 816BBD50  lwz r11, -0x42b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17072 as u32) ) } as u64;
	// 825F4EA0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825F4EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4EA8 size=116
    let mut pc: u32 = 0x825F4EA8;
    'dispatch: loop {
        match pc {
            0x825F4EA8 => {
    //   block [0x825F4EA8..0x825F4F1C)
	// 825F4EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4EB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4EBC: 390BF020  addi r8, r11, -0xfe0
	ctx.r[8].s64 = ctx.r[11].s64 + -4064;
	// 825F4EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4EC4: 392A1FE8  addi r9, r10, 0x1fe8
	ctx.r[9].s64 = ctx.r[10].s64 + 8168;
	// 825F4EC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4ECC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F4ED0: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F4ED4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4EDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4EEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F4EF0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 825F4EF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4EF8: 386BD944  addi r3, r11, -0x26bc
	ctx.r[3].s64 = ctx.r[11].s64 + -9916;
	// 825F4EFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F4F00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4F08: 4BE71F19  bl 0x82466e20
	ctx.lr = 0x825F4F0C;
	sub_82466E20(ctx, base);
	// 825F4F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4F20 size=112
    let mut pc: u32 = 0x825F4F20;
    'dispatch: loop {
        match pc {
            0x825F4F20 => {
    //   block [0x825F4F20..0x825F4F90)
	// 825F4F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4F2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4F30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4F34: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F4F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4F3C: 390BBD58  addi r8, r11, -0x42a8
	ctx.r[8].s64 = ctx.r[11].s64 + -17064;
	// 825F4F40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F4F44: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 825F4F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4F4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4F50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4F58: 386AD974  addi r3, r10, -0x268c
	ctx.r[3].s64 = ctx.r[10].s64 + -9868;
	// 825F4F5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4F7C: 4BE71EA5  bl 0x82466e20
	ctx.lr = 0x825F4F80;
	sub_82466E20(ctx, base);
	// 825F4F80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4F90 size=112
    let mut pc: u32 = 0x825F4F90;
    'dispatch: loop {
        match pc {
            0x825F4F90 => {
    //   block [0x825F4F90..0x825F5000)
	// 825F4F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4F9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4FA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4FA4: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F4FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4FAC: 390BBDB8  addi r8, r11, -0x4248
	ctx.r[8].s64 = ctx.r[11].s64 + -16968;
	// 825F4FB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F4FB4: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 825F4FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4FBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4FC8: 386AD9A4  addi r3, r10, -0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + -9820;
	// 825F4FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4FEC: 4BE71E35  bl 0x82466e20
	ctx.lr = 0x825F4FF0;
	sub_82466E20(ctx, base);
	// 825F4FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5000 size=112
    let mut pc: u32 = 0x825F5000;
    'dispatch: loop {
        match pc {
            0x825F5000 => {
    //   block [0x825F5000..0x825F5070)
	// 825F5000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F500C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5010: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5014: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F5018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F501C: 390BBDE8  addi r8, r11, -0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + -16920;
	// 825F5020: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F5024: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 825F5028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F502C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5038: 386AD9D4  addi r3, r10, -0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + -9772;
	// 825F503C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F504C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F505C: 4BE71DC5  bl 0x82466e20
	ctx.lr = 0x825F5060;
	sub_82466E20(ctx, base);
	// 825F5060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F506C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5070 size=108
    let mut pc: u32 = 0x825F5070;
    'dispatch: loop {
        match pc {
            0x825F5070 => {
    //   block [0x825F5070..0x825F50DC)
	// 825F5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F507C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5084: 38EBBE30  addi r7, r11, -0x41d0
	ctx.r[7].s64 = ctx.r[11].s64 + -16848;
	// 825F5088: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F508C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 825F5090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F509C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F50A0: 386ADA04  addi r3, r10, -0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + -9724;
	// 825F50A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F50A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F50AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F50B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F50B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F50B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F50BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F50C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F50C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F50C8: 4BE71D59  bl 0x82466e20
	ctx.lr = 0x825F50CC;
	sub_82466E20(ctx, base);
	// 825F50CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F50D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F50D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F50D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F50E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F50E0 size=112
    let mut pc: u32 = 0x825F50E0;
    'dispatch: loop {
        match pc {
            0x825F50E0 => {
    //   block [0x825F50E0..0x825F5150)
	// 825F50E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F50E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F50E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F50EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F50F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F50F4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F50F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F50FC: 390BBE60  addi r8, r11, -0x41a0
	ctx.r[8].s64 = ctx.r[11].s64 + -16800;
	// 825F5100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5104: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 825F5108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F510C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5118: 386ADA34  addi r3, r10, -0x25cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9676;
	// 825F511C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F512C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F513C: 4BE71CE5  bl 0x82466e20
	ctx.lr = 0x825F5140;
	sub_82466E20(ctx, base);
	// 825F5140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F514C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5150 size=108
    let mut pc: u32 = 0x825F5150;
    'dispatch: loop {
        match pc {
            0x825F5150 => {
    //   block [0x825F5150..0x825F51BC)
	// 825F5150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F515C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5164: 38EBBE78  addi r7, r11, -0x4188
	ctx.r[7].s64 = ctx.r[11].s64 + -16776;
	// 825F5168: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F516C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 825F5170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5174: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F517C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5180: 386ADA64  addi r3, r10, -0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + -9628;
	// 825F5184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F518C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F519C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F51A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F51A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F51A8: 4BE71C79  bl 0x82466e20
	ctx.lr = 0x825F51AC;
	sub_82466E20(ctx, base);
	// 825F51AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F51B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F51B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F51B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F51C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F51C0 size=108
    let mut pc: u32 = 0x825F51C0;
    'dispatch: loop {
        match pc {
            0x825F51C0 => {
    //   block [0x825F51C0..0x825F522C)
	// 825F51C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F51C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F51C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F51CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F51D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F51D4: 38EBBEC0  addi r7, r11, -0x4140
	ctx.r[7].s64 = ctx.r[11].s64 + -16704;
	// 825F51D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F51DC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 825F51E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F51E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F51E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F51EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F51F0: 386ADA94  addi r3, r10, -0x256c
	ctx.r[3].s64 = ctx.r[10].s64 + -9580;
	// 825F51F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F51F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F51FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F520C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5218: 4BE71C09  bl 0x82466e20
	ctx.lr = 0x825F521C;
	sub_82466E20(ctx, base);
	// 825F521C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5230 size=116
    let mut pc: u32 = 0x825F5230;
    'dispatch: loop {
        match pc {
            0x825F5230 => {
    //   block [0x825F5230..0x825F52A4)
	// 825F5230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F523C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F5240: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5244: 392B201C  addi r9, r11, 0x201c
	ctx.r[9].s64 = ctx.r[11].s64 + 8220;
	// 825F5248: 38AADF14  addi r5, r10, -0x20ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8428;
	// 825F524C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5250: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F5254: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 825F5258: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F525C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 825F5260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5264: 396BBF20  addi r11, r11, -0x40e0
	ctx.r[11].s64 = ctx.r[11].s64 + -16608;
	// 825F5268: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F526C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5270: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F5274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5278: 386ADAC4  addi r3, r10, -0x253c
	ctx.r[3].s64 = ctx.r[10].s64 + -9532;
	// 825F527C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5280: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F5284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5288: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F528C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5290: 4BE71B91  bl 0x82466e20
	ctx.lr = 0x825F5294;
	sub_82466E20(ctx, base);
	// 825F5294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F529C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F52A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F52A8 size=100
    let mut pc: u32 = 0x825F52A8;
    'dispatch: loop {
        match pc {
            0x825F52A8 => {
    //   block [0x825F52A8..0x825F530C)
	// 825F52A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F52AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F52B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F52B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F52B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F52BC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F52C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F52C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F52C8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 825F52CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F52D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F52D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F52D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F52DC: 386ADAF4  addi r3, r10, -0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + -9484;
	// 825F52E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F52E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F52E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F52EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F52F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F52F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F52F8: 4BE71B29  bl 0x82466e20
	ctx.lr = 0x825F52FC;
	sub_82466E20(ctx, base);
	// 825F52FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5310 size=100
    let mut pc: u32 = 0x825F5310;
    'dispatch: loop {
        match pc {
            0x825F5310 => {
    //   block [0x825F5310..0x825F5374)
	// 825F5310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F531C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5324: 38AADB84  addi r5, r10, -0x247c
	ctx.r[5].s64 = ctx.r[10].s64 + -9340;
	// 825F5328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F532C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5330: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 825F5334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F533C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5344: 386ADB24  addi r3, r10, -0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + -9436;
	// 825F5348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F534C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F535C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5360: 4BE71AC1  bl 0x82466e20
	ctx.lr = 0x825F5364;
	sub_82466E20(ctx, base);
	// 825F5364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F536C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5378 size=100
    let mut pc: u32 = 0x825F5378;
    'dispatch: loop {
        match pc {
            0x825F5378 => {
    //   block [0x825F5378..0x825F53DC)
	// 825F5378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F537C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F538C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 825F5390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5398: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 825F539C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F53A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F53A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F53A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F53AC: 386ADB54  addi r3, r10, -0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9388;
	// 825F53B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F53B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F53B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F53BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F53C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F53C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F53C8: 4BE71A59  bl 0x82466e20
	ctx.lr = 0x825F53CC;
	sub_82466E20(ctx, base);
	// 825F53CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F53D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F53D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F53D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F53E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F53E0 size=104
    let mut pc: u32 = 0x825F53E0;
    'dispatch: loop {
        match pc {
            0x825F53E0 => {
    //   block [0x825F53E0..0x825F5448)
	// 825F53E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F53E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F53E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F53EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F53F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F53F4: 392A2098  addi r9, r10, 0x2098
	ctx.r[9].s64 = ctx.r[10].s64 + 8344;
	// 825F53F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F53FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5400: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 825F5404: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F540C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5414: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 825F5418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F541C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F542C: 386ADB84  addi r3, r10, -0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + -9340;
	// 825F5430: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5434: 4BE719ED  bl 0x82466e20
	ctx.lr = 0x825F5438;
	sub_82466E20(ctx, base);
	// 825F5438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F543C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5448 size=108
    let mut pc: u32 = 0x825F5448;
    'dispatch: loop {
        match pc {
            0x825F5448 => {
    //   block [0x825F5448..0x825F54B4)
	// 825F5448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F544C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5454: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F545C: 38EBC0BC  addi r7, r11, -0x3f44
	ctx.r[7].s64 = ctx.r[11].s64 + -16196;
	// 825F5460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F5464: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 825F5468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F546C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5478: 386ADBB4  addi r3, r10, -0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + -9292;
	// 825F547C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F548C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F549C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F54A0: 4BE71981  bl 0x82466e20
	ctx.lr = 0x825F54A4;
	sub_82466E20(ctx, base);
	// 825F54A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F54A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F54AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F54B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F54B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F54B8 size=112
    let mut pc: u32 = 0x825F54B8;
    'dispatch: loop {
        match pc {
            0x825F54B8 => {
    //   block [0x825F54B8..0x825F5528)
	// 825F54B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F54BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F54C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F54C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F54C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F54CC: 38AADB84  addi r5, r10, -0x247c
	ctx.r[5].s64 = ctx.r[10].s64 + -9340;
	// 825F54D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F54D4: 390BC0F0  addi r8, r11, -0x3f10
	ctx.r[8].s64 = ctx.r[11].s64 + -16144;
	// 825F54D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F54DC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 825F54E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F54E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F54E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F54EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F54F0: 386ADBE4  addi r3, r10, -0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + -9244;
	// 825F54F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F54F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F54FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F550C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5514: 4BE7190D  bl 0x82466e20
	ctx.lr = 0x825F5518;
	sub_82466E20(ctx, base);
	// 825F5518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F551C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F5528 size=24
    let mut pc: u32 = 0x825F5528;
    'dispatch: loop {
        match pc {
            0x825F5528 => {
    //   block [0x825F5528..0x825F5540)
	// 825F5528: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F552C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5530: 394AF098  addi r10, r10, -0xf68
	ctx.r[10].s64 = ctx.r[10].s64 + -3944;
	// 825F5534: 816BC0EC  lwz r11, -0x3f14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16148 as u32) ) } as u64;
	// 825F5538: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F553C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5540 size=116
    let mut pc: u32 = 0x825F5540;
    'dispatch: loop {
        match pc {
            0x825F5540 => {
    //   block [0x825F5540..0x825F55B4)
	// 825F5540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F554C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5554: 390BF098  addi r8, r11, -0xf68
	ctx.r[8].s64 = ctx.r[11].s64 + -3944;
	// 825F5558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F555C: 392A2100  addi r9, r10, 0x2100
	ctx.r[9].s64 = ctx.r[10].s64 + 8448;
	// 825F5560: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5564: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825F5568: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F556C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5574: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F557C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5584: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F5588: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 825F558C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F5590: 386BDC14  addi r3, r11, -0x23ec
	ctx.r[3].s64 = ctx.r[11].s64 + -9196;
	// 825F5594: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5598: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F559C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F55A0: 4BE71881  bl 0x82466e20
	ctx.lr = 0x825F55A4;
	sub_82466E20(ctx, base);
	// 825F55A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F55A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F55AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F55B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F55B8 size=100
    let mut pc: u32 = 0x825F55B8;
    'dispatch: loop {
        match pc {
            0x825F55B8 => {
    //   block [0x825F55B8..0x825F561C)
	// 825F55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F55BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F55C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F55C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F55C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F55CC: 38AADC14  addi r5, r10, -0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9196;
	// 825F55D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F55D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F55D8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 825F55DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F55E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F55E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F55E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F55EC: 386ADC44  addi r3, r10, -0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9148;
	// 825F55F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F55F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F55F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F55FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5600: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5608: 4BE71819  bl 0x82466e20
	ctx.lr = 0x825F560C;
	sub_82466E20(ctx, base);
	// 825F560C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5620 size=100
    let mut pc: u32 = 0x825F5620;
    'dispatch: loop {
        match pc {
            0x825F5620 => {
    //   block [0x825F5620..0x825F5684)
	// 825F5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F562C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5634: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 825F5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F563C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5640: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 825F5644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F564C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5654: 386ADC74  addi r3, r10, -0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + -9100;
	// 825F5658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F565C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5660: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F566C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5670: 4BE717B1  bl 0x82466e20
	ctx.lr = 0x825F5674;
	sub_82466E20(ctx, base);
	// 825F5674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F567C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5688 size=112
    let mut pc: u32 = 0x825F5688;
    'dispatch: loop {
        match pc {
            0x825F5688 => {
    //   block [0x825F5688..0x825F56F8)
	// 825F5688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F568C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5694: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5698: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F569C: 38AADC14  addi r5, r10, -0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9196;
	// 825F56A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F56A4: 390BC198  addi r8, r11, -0x3e68
	ctx.r[8].s64 = ctx.r[11].s64 + -15976;
	// 825F56A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F56AC: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 825F56B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F56B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F56B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F56BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F56C0: 386ADCA4  addi r3, r10, -0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + -9052;
	// 825F56C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F56C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F56CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F56D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F56D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F56D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F56DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F56E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F56E4: 4BE7173D  bl 0x82466e20
	ctx.lr = 0x825F56E8;
	sub_82466E20(ctx, base);
	// 825F56E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F56EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F56F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F56F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F56F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F56F8 size=100
    let mut pc: u32 = 0x825F56F8;
    'dispatch: loop {
        match pc {
            0x825F56F8 => {
    //   block [0x825F56F8..0x825F575C)
	// 825F56F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F56FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5704: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F570C: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 825F5710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5718: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 825F571C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F572C: 386ADCD4  addi r3, r10, -0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + -9004;
	// 825F5730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F573C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5748: 4BE716D9  bl 0x82466e20
	ctx.lr = 0x825F574C;
	sub_82466E20(ctx, base);
	// 825F574C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5760 size=100
    let mut pc: u32 = 0x825F5760;
    'dispatch: loop {
        match pc {
            0x825F5760 => {
    //   block [0x825F5760..0x825F57C4)
	// 825F5760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F576C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5774: 38AADC14  addi r5, r10, -0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9196;
	// 825F5778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F577C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5780: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 825F5784: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F578C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5794: 386ADD04  addi r3, r10, -0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8956;
	// 825F5798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F579C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F57A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F57A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F57A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F57AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F57B0: 4BE71671  bl 0x82466e20
	ctx.lr = 0x825F57B4;
	sub_82466E20(ctx, base);
	// 825F57B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F57B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F57BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F57C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F57C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F57C8 size=100
    let mut pc: u32 = 0x825F57C8;
    'dispatch: loop {
        match pc {
            0x825F57C8 => {
    //   block [0x825F57C8..0x825F582C)
	// 825F57C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F57CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F57D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F57D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F57D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F57DC: 38AADC44  addi r5, r10, -0x23bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9148;
	// 825F57E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F57E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F57E8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 825F57EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F57F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F57F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F57F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F57FC: 386ADD34  addi r3, r10, -0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8908;
	// 825F5800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5804: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5808: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F580C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5818: 4BE71609  bl 0x82466e20
	ctx.lr = 0x825F581C;
	sub_82466E20(ctx, base);
	// 825F581C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5830 size=100
    let mut pc: u32 = 0x825F5830;
    'dispatch: loop {
        match pc {
            0x825F5830 => {
    //   block [0x825F5830..0x825F5894)
	// 825F5830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F583C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5844: 38AADD04  addi r5, r10, -0x22fc
	ctx.r[5].s64 = ctx.r[10].s64 + -8956;
	// 825F5848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F584C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5850: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 825F5854: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F585C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5864: 386ADD64  addi r3, r10, -0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + -8860;
	// 825F5868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F586C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5870: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5878: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F587C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5880: 4BE715A1  bl 0x82466e20
	ctx.lr = 0x825F5884;
	sub_82466E20(ctx, base);
	// 825F5884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F588C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5898 size=100
    let mut pc: u32 = 0x825F5898;
    'dispatch: loop {
        match pc {
            0x825F5898 => {
    //   block [0x825F5898..0x825F58FC)
	// 825F5898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F589C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F58A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F58A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F58A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F58AC: 38AADC44  addi r5, r10, -0x23bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9148;
	// 825F58B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F58B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F58B8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 825F58BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F58C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F58C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F58C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F58CC: 386ADD94  addi r3, r10, -0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + -8812;
	// 825F58D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F58D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F58D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F58DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F58E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F58E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F58E8: 4BE71539  bl 0x82466e20
	ctx.lr = 0x825F58EC;
	sub_82466E20(ctx, base);
	// 825F58EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F58F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F58F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F58F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5900 size=112
    let mut pc: u32 = 0x825F5900;
    'dispatch: loop {
        match pc {
            0x825F5900 => {
    //   block [0x825F5900..0x825F5970)
	// 825F5900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F590C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5910: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5914: 38AADE24  addi r5, r10, -0x21dc
	ctx.r[5].s64 = ctx.r[10].s64 + -8668;
	// 825F5918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F591C: 390BC1C8  addi r8, r11, -0x3e38
	ctx.r[8].s64 = ctx.r[11].s64 + -15928;
	// 825F5920: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F5924: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 825F5928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F592C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5938: 386ADDC4  addi r3, r10, -0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + -8764;
	// 825F593C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F594C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F595C: 4BE714C5  bl 0x82466e20
	ctx.lr = 0x825F5960;
	sub_82466E20(ctx, base);
	// 825F5960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F596C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5970 size=112
    let mut pc: u32 = 0x825F5970;
    'dispatch: loop {
        match pc {
            0x825F5970 => {
    //   block [0x825F5970..0x825F59E0)
	// 825F5970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F597C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5980: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5984: 38AADE54  addi r5, r10, -0x21ac
	ctx.r[5].s64 = ctx.r[10].s64 + -8620;
	// 825F5988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F598C: 390BC1F8  addi r8, r11, -0x3e08
	ctx.r[8].s64 = ctx.r[11].s64 + -15880;
	// 825F5990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5994: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 825F5998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F599C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F59A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F59A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F59A8: 386ADDF4  addi r3, r10, -0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + -8716;
	// 825F59AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F59B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F59B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F59B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F59BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F59C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F59C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F59C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F59CC: 4BE71455  bl 0x82466e20
	ctx.lr = 0x825F59D0;
	sub_82466E20(ctx, base);
	// 825F59D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F59D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F59D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F59DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F59E0 size=112
    let mut pc: u32 = 0x825F59E0;
    'dispatch: loop {
        match pc {
            0x825F59E0 => {
    //   block [0x825F59E0..0x825F5A50)
	// 825F59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F59EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F59F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F59F4: 38AADF14  addi r5, r10, -0x20ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8428;
	// 825F59F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F59FC: 390BC210  addi r8, r11, -0x3df0
	ctx.r[8].s64 = ctx.r[11].s64 + -15856;
	// 825F5A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F5A04: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 825F5A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5A18: 386ADE24  addi r3, r10, -0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8668;
	// 825F5A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5A3C: 4BE713E5  bl 0x82466e20
	ctx.lr = 0x825F5A40;
	sub_82466E20(ctx, base);
	// 825F5A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5A50 size=112
    let mut pc: u32 = 0x825F5A50;
    'dispatch: loop {
        match pc {
            0x825F5A50 => {
    //   block [0x825F5A50..0x825F5AC0)
	// 825F5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5A5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5A60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5A64: 38AADE24  addi r5, r10, -0x21dc
	ctx.r[5].s64 = ctx.r[10].s64 + -8668;
	// 825F5A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5A6C: 390BC240  addi r8, r11, -0x3dc0
	ctx.r[8].s64 = ctx.r[11].s64 + -15808;
	// 825F5A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5A74: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 825F5A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5A7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5A88: 386ADE54  addi r3, r10, -0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8620;
	// 825F5A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5AAC: 4BE71375  bl 0x82466e20
	ctx.lr = 0x825F5AB0;
	sub_82466E20(ctx, base);
	// 825F5AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5AC0 size=112
    let mut pc: u32 = 0x825F5AC0;
    'dispatch: loop {
        match pc {
            0x825F5AC0 => {
    //   block [0x825F5AC0..0x825F5B30)
	// 825F5AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5AD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5AD4: 38AADE54  addi r5, r10, -0x21ac
	ctx.r[5].s64 = ctx.r[10].s64 + -8620;
	// 825F5AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5ADC: 390BC258  addi r8, r11, -0x3da8
	ctx.r[8].s64 = ctx.r[11].s64 + -15784;
	// 825F5AE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5AE4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 825F5AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5AEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5AF8: 386ADE84  addi r3, r10, -0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + -8572;
	// 825F5AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5B1C: 4BE71305  bl 0x82466e20
	ctx.lr = 0x825F5B20;
	sub_82466E20(ctx, base);
	// 825F5B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5B30 size=116
    let mut pc: u32 = 0x825F5B30;
    'dispatch: loop {
        match pc {
            0x825F5B30 => {
    //   block [0x825F5B30..0x825F5BA4)
	// 825F5B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5B3C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5B40: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F5B44: 390AC270  addi r8, r10, -0x3d90
	ctx.r[8].s64 = ctx.r[10].s64 + -15760;
	// 825F5B48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5B4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F5B50: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5B54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5B58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F5B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5B64: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 825F5B68: 396B2114  addi r11, r11, 0x2114
	ctx.r[11].s64 = ctx.r[11].s64 + 8468;
	// 825F5B6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5B70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5B74: 386ADEB4  addi r3, r10, -0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + -8524;
	// 825F5B78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F5B7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5B80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F5B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5B90: 4BE71291  bl 0x82466e20
	ctx.lr = 0x825F5B94;
	sub_82466E20(ctx, base);
	// 825F5B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F5BA8 size=48
    let mut pc: u32 = 0x825F5BA8;
    'dispatch: loop {
        match pc {
            0x825F5BA8 => {
    //   block [0x825F5BA8..0x825F5BD8)
	// 825F5BA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5BAC: 814BC324  lwz r10, -0x3cdc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15580 as u32) ) } as u64;
	// 825F5BB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5BB4: 396BF158  addi r11, r11, -0xea8
	ctx.r[11].s64 = ctx.r[11].s64 + -3752;
	// 825F5BB8: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825F5BBC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5BC0: 814AC320  lwz r10, -0x3ce0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15584 as u32) ) } as u64;
	// 825F5BC4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 825F5BC8: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5BCC: 814AC31C  lwz r10, -0x3ce4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15588 as u32) ) } as u64;
	// 825F5BD0: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 825F5BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5BD8 size=116
    let mut pc: u32 = 0x825F5BD8;
    'dispatch: loop {
        match pc {
            0x825F5BD8 => {
    //   block [0x825F5BD8..0x825F5C4C)
	// 825F5BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5BE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F5BE8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5BEC: 392B21E8  addi r9, r11, 0x21e8
	ctx.r[9].s64 = ctx.r[11].s64 + 8680;
	// 825F5BF0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5BF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5BF8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 825F5BFC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 825F5C00: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5C04: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 825F5C08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5C0C: 396BF158  addi r11, r11, -0xea8
	ctx.r[11].s64 = ctx.r[11].s64 + -3752;
	// 825F5C10: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F5C14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5C18: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F5C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5C20: 386ADEE4  addi r3, r10, -0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + -8476;
	// 825F5C24: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 825F5C28: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F5C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5C30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F5C34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5C38: 4BE711E9  bl 0x82466e20
	ctx.lr = 0x825F5C3C;
	sub_82466E20(ctx, base);
	// 825F5C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5C50 size=116
    let mut pc: u32 = 0x825F5C50;
    'dispatch: loop {
        match pc {
            0x825F5C50 => {
    //   block [0x825F5C50..0x825F5CC4)
	// 825F5C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5C5C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5C64: 390BC330  addi r8, r11, -0x3cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -15568;
	// 825F5C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5C6C: 392A2364  addi r9, r10, 0x2364
	ctx.r[9].s64 = ctx.r[10].s64 + 9060;
	// 825F5C70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5C74: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F5C78: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5C7C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5C94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F5C98: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 825F5C9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F5CA0: 386BDF14  addi r3, r11, -0x20ec
	ctx.r[3].s64 = ctx.r[11].s64 + -8428;
	// 825F5CA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5CA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5CB0: 4BE71171  bl 0x82466e20
	ctx.lr = 0x825F5CB4;
	sub_82466E20(ctx, base);
	// 825F5CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5CC8 size=112
    let mut pc: u32 = 0x825F5CC8;
    'dispatch: loop {
        match pc {
            0x825F5CC8 => {
    //   block [0x825F5CC8..0x825F5D38)
	// 825F5CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5CD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5CD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5CDC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5CE4: 390BC3C0  addi r8, r11, -0x3c40
	ctx.r[8].s64 = ctx.r[11].s64 + -15424;
	// 825F5CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5CEC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 825F5CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5D00: 386ADF44  addi r3, r10, -0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8380;
	// 825F5D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5D24: 4BE710FD  bl 0x82466e20
	ctx.lr = 0x825F5D28;
	sub_82466E20(ctx, base);
	// 825F5D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5D38 size=112
    let mut pc: u32 = 0x825F5D38;
    'dispatch: loop {
        match pc {
            0x825F5D38 => {
    //   block [0x825F5D38..0x825F5DA8)
	// 825F5D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5D48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5D4C: 38AABF64  addi r5, r10, -0x409c
	ctx.r[5].s64 = ctx.r[10].s64 + -16540;
	// 825F5D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5D54: 390BC3D8  addi r8, r11, -0x3c28
	ctx.r[8].s64 = ctx.r[11].s64 + -15400;
	// 825F5D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5D5C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 825F5D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5D70: 386ADF74  addi r3, r10, -0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + -8332;
	// 825F5D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5D94: 4BE7108D  bl 0x82466e20
	ctx.lr = 0x825F5D98;
	sub_82466E20(ctx, base);
	// 825F5D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5DA8 size=108
    let mut pc: u32 = 0x825F5DA8;
    'dispatch: loop {
        match pc {
            0x825F5DA8 => {
    //   block [0x825F5DA8..0x825F5E14)
	// 825F5DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5DB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5DBC: 38EBC3F0  addi r7, r11, -0x3c10
	ctx.r[7].s64 = ctx.r[11].s64 + -15376;
	// 825F5DC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F5DC4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 825F5DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5DCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5DD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5DD8: 386ADFA4  addi r3, r10, -0x205c
	ctx.r[3].s64 = ctx.r[10].s64 + -8284;
	// 825F5DDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5E00: 4BE71021  bl 0x82466e20
	ctx.lr = 0x825F5E04;
	sub_82466E20(ctx, base);
	// 825F5E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5E18 size=112
    let mut pc: u32 = 0x825F5E18;
    'dispatch: loop {
        match pc {
            0x825F5E18 => {
    //   block [0x825F5E18..0x825F5E88)
	// 825F5E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5E24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5E28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5E2C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5E34: 390BC408  addi r8, r11, -0x3bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -15352;
	// 825F5E38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F5E3C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 825F5E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5E50: 386ADFD4  addi r3, r10, -0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + -8236;
	// 825F5E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5E74: 4BE70FAD  bl 0x82466e20
	ctx.lr = 0x825F5E78;
	sub_82466E20(ctx, base);
	// 825F5E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5E88 size=108
    let mut pc: u32 = 0x825F5E88;
    'dispatch: loop {
        match pc {
            0x825F5E88 => {
    //   block [0x825F5E88..0x825F5EF4)
	// 825F5E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5E94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5E9C: 38EBC450  addi r7, r11, -0x3bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -15280;
	// 825F5EA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F5EA4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 825F5EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5EB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5EB8: 386AE004  addi r3, r10, -0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -8188;
	// 825F5EBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5EE0: 4BE70F41  bl 0x82466e20
	ctx.lr = 0x825F5EE4;
	sub_82466E20(ctx, base);
	// 825F5EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5EF8 size=108
    let mut pc: u32 = 0x825F5EF8;
    'dispatch: loop {
        match pc {
            0x825F5EF8 => {
    //   block [0x825F5EF8..0x825F5F64)
	// 825F5EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5F04: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5F0C: 38EBC480  addi r7, r11, -0x3b80
	ctx.r[7].s64 = ctx.r[11].s64 + -15232;
	// 825F5F10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F5F14: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 825F5F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5F28: 386AE034  addi r3, r10, -0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -8140;
	// 825F5F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5F50: 4BE70ED1  bl 0x82466e20
	ctx.lr = 0x825F5F54;
	sub_82466E20(ctx, base);
	// 825F5F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5F68 size=112
    let mut pc: u32 = 0x825F5F68;
    'dispatch: loop {
        match pc {
            0x825F5F68 => {
    //   block [0x825F5F68..0x825F5FD8)
	// 825F5F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5F74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5F78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5F7C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5F84: 390BC498  addi r8, r11, -0x3b68
	ctx.r[8].s64 = ctx.r[11].s64 + -15208;
	// 825F5F88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F5F8C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 825F5F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5F94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5FA0: 386AE064  addi r3, r10, -0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -8092;
	// 825F5FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5FC4: 4BE70E5D  bl 0x82466e20
	ctx.lr = 0x825F5FC8;
	sub_82466E20(ctx, base);
	// 825F5FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5FD8 size=96
    let mut pc: u32 = 0x825F5FD8;
    'dispatch: loop {
        match pc {
            0x825F5FD8 => {
    //   block [0x825F5FD8..0x825F6038)
	// 825F5FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5FE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5FEC: 388A507C  addi r4, r10, 0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + 20604;
	// 825F5FF0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5FF8: 386AE094  addi r3, r10, -0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -8044;
	// 825F5FFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6004: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F6008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F600C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F601C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F6024: 4BE70DFD  bl 0x82466e20
	ctx.lr = 0x825F6028;
	sub_82466E20(ctx, base);
	// 825F6028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F602C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6038 size=112
    let mut pc: u32 = 0x825F6038;
    'dispatch: loop {
        match pc {
            0x825F6038 => {
    //   block [0x825F6038..0x825F60A8)
	// 825F6038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F603C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6044: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6048: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F604C: 392A23BC  addi r9, r10, 0x23bc
	ctx.r[9].s64 = ctx.r[10].s64 + 9148;
	// 825F6050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6054: 390BC4D0  addi r8, r11, -0x3b30
	ctx.r[8].s64 = ctx.r[11].s64 + -15152;
	// 825F6058: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F605C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 825F6060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F606C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6070: 386AE0C4  addi r3, r10, -0x1f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7996;
	// 825F6074: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F6078: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F607C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F608C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6094: 4BE70D8D  bl 0x82466e20
	ctx.lr = 0x825F6098;
	sub_82466E20(ctx, base);
	// 825F6098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F609C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F60A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F60A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F60A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F60A8 size=116
    let mut pc: u32 = 0x825F60A8;
    'dispatch: loop {
        match pc {
            0x825F60A8 => {
    //   block [0x825F60A8..0x825F611C)
	// 825F60A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F60AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F60B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F60B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F60B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F60BC: 390BC578  addi r8, r11, -0x3a88
	ctx.r[8].s64 = ctx.r[11].s64 + -14984;
	// 825F60C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F60C4: 392A2390  addi r9, r10, 0x2390
	ctx.r[9].s64 = ctx.r[10].s64 + 9104;
	// 825F60C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F60CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F60D0: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F60D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F60D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F60DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F60E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F60E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F60E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F60EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F60F0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 825F60F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F60F8: 386BE0F4  addi r3, r11, -0x1f0c
	ctx.r[3].s64 = ctx.r[11].s64 + -7948;
	// 825F60FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F6100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6108: 4BE70D19  bl 0x82466e20
	ctx.lr = 0x825F610C;
	sub_82466E20(ctx, base);
	// 825F610C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6120 size=112
    let mut pc: u32 = 0x825F6120;
    'dispatch: loop {
        match pc {
            0x825F6120 => {
    //   block [0x825F6120..0x825F6190)
	// 825F6120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F612C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6130: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6134: 392A23E8  addi r9, r10, 0x23e8
	ctx.r[9].s64 = ctx.r[10].s64 + 9192;
	// 825F6138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F613C: 390BC590  addi r8, r11, -0x3a70
	ctx.r[8].s64 = ctx.r[11].s64 + -14960;
	// 825F6140: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825F6144: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 825F6148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F614C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6158: 386AE124  addi r3, r10, -0x1edc
	ctx.r[3].s64 = ctx.r[10].s64 + -7900;
	// 825F615C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F6160: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F6164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F616C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F617C: 4BE70CA5  bl 0x82466e20
	ctx.lr = 0x825F6180;
	sub_82466E20(ctx, base);
	// 825F6180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F618C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6190 size=112
    let mut pc: u32 = 0x825F6190;
    'dispatch: loop {
        match pc {
            0x825F6190 => {
    //   block [0x825F6190..0x825F6200)
	// 825F6190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F619C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F61A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F61A4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F61A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F61AC: 390BC5F0  addi r8, r11, -0x3a10
	ctx.r[8].s64 = ctx.r[11].s64 + -14864;
	// 825F61B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F61B4: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 825F61B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F61BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F61C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F61C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F61C8: 386AE154  addi r3, r10, -0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + -7852;
	// 825F61CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F61D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F61D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F61D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F61DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F61E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F61E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F61E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F61EC: 4BE70C35  bl 0x82466e20
	ctx.lr = 0x825F61F0;
	sub_82466E20(ctx, base);
	// 825F61F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F61F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F61F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F61FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6200 size=112
    let mut pc: u32 = 0x825F6200;
    'dispatch: loop {
        match pc {
            0x825F6200 => {
    //   block [0x825F6200..0x825F6270)
	// 825F6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F620C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6210: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6214: 38AAD164  addi r5, r10, -0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11932;
	// 825F6218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F621C: 390BC608  addi r8, r11, -0x39f8
	ctx.r[8].s64 = ctx.r[11].s64 + -14840;
	// 825F6220: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F6224: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 825F6228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F622C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6238: 386AE184  addi r3, r10, -0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7804;
	// 825F623C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F624C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F625C: 4BE70BC5  bl 0x82466e20
	ctx.lr = 0x825F6260;
	sub_82466E20(ctx, base);
	// 825F6260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F626C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6270 size=112
    let mut pc: u32 = 0x825F6270;
    'dispatch: loop {
        match pc {
            0x825F6270 => {
    //   block [0x825F6270..0x825F62E0)
	// 825F6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F627C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6284: 38AAD164  addi r5, r10, -0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11932;
	// 825F6288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F628C: 390BC650  addi r8, r11, -0x39b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14768;
	// 825F6290: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F6294: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 825F6298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F629C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F62A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F62A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F62A8: 386AE1B4  addi r3, r10, -0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7756;
	// 825F62AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F62B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F62B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F62B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F62BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F62C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F62C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F62C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F62CC: 4BE70B55  bl 0x82466e20
	ctx.lr = 0x825F62D0;
	sub_82466E20(ctx, base);
	// 825F62D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F62D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F62D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F62DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F62E0 size=112
    let mut pc: u32 = 0x825F62E0;
    'dispatch: loop {
        match pc {
            0x825F62E0 => {
    //   block [0x825F62E0..0x825F6350)
	// 825F62E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F62E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F62E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F62EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F62F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F62F4: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F62F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F62FC: 390BC6B0  addi r8, r11, -0x3950
	ctx.r[8].s64 = ctx.r[11].s64 + -14672;
	// 825F6300: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F6304: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 825F6308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F630C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6318: 386AE1E4  addi r3, r10, -0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7708;
	// 825F631C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F632C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F633C: 4BE70AE5  bl 0x82466e20
	ctx.lr = 0x825F6340;
	sub_82466E20(ctx, base);
	// 825F6340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F634C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6350 size=112
    let mut pc: u32 = 0x825F6350;
    'dispatch: loop {
        match pc {
            0x825F6350 => {
    //   block [0x825F6350..0x825F63C0)
	// 825F6350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F635C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6360: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6364: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F6368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F636C: 390BC710  addi r8, r11, -0x38f0
	ctx.r[8].s64 = ctx.r[11].s64 + -14576;
	// 825F6370: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F6374: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 825F6378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F637C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6388: 386AE214  addi r3, r10, -0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + -7660;
	// 825F638C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F639C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F63A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F63A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F63A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F63AC: 4BE70A75  bl 0x82466e20
	ctx.lr = 0x825F63B0;
	sub_82466E20(ctx, base);
	// 825F63B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F63B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F63B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F63BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F63C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F63C0 size=112
    let mut pc: u32 = 0x825F63C0;
    'dispatch: loop {
        match pc {
            0x825F63C0 => {
    //   block [0x825F63C0..0x825F6430)
	// 825F63C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F63C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F63C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F63CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F63D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F63D4: 38AAD164  addi r5, r10, -0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11932;
	// 825F63D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F63DC: 390BC770  addi r8, r11, -0x3890
	ctx.r[8].s64 = ctx.r[11].s64 + -14480;
	// 825F63E0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825F63E4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 825F63E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F63EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F63F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F63F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F63F8: 386AE244  addi r3, r10, -0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7612;
	// 825F63FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F640C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F641C: 4BE70A05  bl 0x82466e20
	ctx.lr = 0x825F6420;
	sub_82466E20(ctx, base);
	// 825F6420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F642C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6430 size=112
    let mut pc: u32 = 0x825F6430;
    'dispatch: loop {
        match pc {
            0x825F6430 => {
    //   block [0x825F6430..0x825F64A0)
	// 825F6430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F643C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F6440: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 825F6444: 38EAC830  addi r7, r10, -0x37d0
	ctx.r[7].s64 = ctx.r[10].s64 + -14288;
	// 825F6448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F644C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F6450: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 825F6454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6458: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F645C: 396B2400  addi r11, r11, 0x2400
	ctx.r[11].s64 = ctx.r[11].s64 + 9216;
	// 825F6460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6464: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6468: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F646C: 386AE274  addi r3, r10, -0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7564;
	// 825F6470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6474: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F6478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F647C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F6480: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6484: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6488: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F648C: 4BE70995  bl 0x82466e20
	ctx.lr = 0x825F6490;
	sub_82466E20(ctx, base);
	// 825F6490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F649C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F64A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F64A0 size=112
    let mut pc: u32 = 0x825F64A0;
    'dispatch: loop {
        match pc {
            0x825F64A0 => {
    //   block [0x825F64A0..0x825F6510)
	// 825F64A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F64A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F64A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F64AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F64B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F64B4: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F64B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F64BC: 390BC9F8  addi r8, r11, -0x3608
	ctx.r[8].s64 = ctx.r[11].s64 + -13832;
	// 825F64C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F64C4: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 825F64C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F64CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F64D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F64D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F64D8: 386AE2A4  addi r3, r10, -0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7516;
	// 825F64DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F64E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F64E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F64E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F64EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F64F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F64F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F64F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F64FC: 4BE70925  bl 0x82466e20
	ctx.lr = 0x825F6500;
	sub_82466E20(ctx, base);
	// 825F6500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F650C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6510 size=112
    let mut pc: u32 = 0x825F6510;
    'dispatch: loop {
        match pc {
            0x825F6510 => {
    //   block [0x825F6510..0x825F6580)
	// 825F6510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F651C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6524: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F6528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F652C: 390BCA10  addi r8, r11, -0x35f0
	ctx.r[8].s64 = ctx.r[11].s64 + -13808;
	// 825F6530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6534: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 825F6538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F653C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6548: 386AE2D4  addi r3, r10, -0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7468;
	// 825F654C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F655C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F6560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F656C: 4BE708B5  bl 0x82466e20
	ctx.lr = 0x825F6570;
	sub_82466E20(ctx, base);
	// 825F6570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F657C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6580 size=112
    let mut pc: u32 = 0x825F6580;
    'dispatch: loop {
        match pc {
            0x825F6580 => {
    //   block [0x825F6580..0x825F65F0)
	// 825F6580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F658C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6590: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6594: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F6598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F659C: 390BCA28  addi r8, r11, -0x35d8
	ctx.r[8].s64 = ctx.r[11].s64 + -13784;
	// 825F65A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F65A4: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 825F65A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F65AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F65B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F65B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F65B8: 386AE304  addi r3, r10, -0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7420;
	// 825F65BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F65C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F65C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F65C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F65CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F65D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F65D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F65D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F65DC: 4BE70845  bl 0x82466e20
	ctx.lr = 0x825F65E0;
	sub_82466E20(ctx, base);
	// 825F65E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F65E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F65E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F65EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F65F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F65F0 size=108
    let mut pc: u32 = 0x825F65F0;
    'dispatch: loop {
        match pc {
            0x825F65F0 => {
    //   block [0x825F65F0..0x825F665C)
	// 825F65F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F65F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F65F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F65FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6604: 38EBCA58  addi r7, r11, -0x35a8
	ctx.r[7].s64 = ctx.r[11].s64 + -13736;
	// 825F6608: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F660C: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 825F6610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F661C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6620: 386AE334  addi r3, r10, -0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -7372;
	// 825F6624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F663C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6648: 4BE707D9  bl 0x82466e20
	ctx.lr = 0x825F664C;
	sub_82466E20(ctx, base);
	// 825F664C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6660 size=112
    let mut pc: u32 = 0x825F6660;
    'dispatch: loop {
        match pc {
            0x825F6660 => {
    //   block [0x825F6660..0x825F66D0)
	// 825F6660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F666C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6670: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6674: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F6678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F667C: 390BCA88  addi r8, r11, -0x3578
	ctx.r[8].s64 = ctx.r[11].s64 + -13688;
	// 825F6680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6684: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 825F6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F668C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6698: 386AE364  addi r3, r10, -0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7324;
	// 825F669C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F66A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F66A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F66A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F66AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F66B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F66B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F66B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F66BC: 4BE70765  bl 0x82466e20
	ctx.lr = 0x825F66C0;
	sub_82466E20(ctx, base);
	// 825F66C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F66C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F66C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F66CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F66D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F66D0 size=108
    let mut pc: u32 = 0x825F66D0;
    'dispatch: loop {
        match pc {
            0x825F66D0 => {
    //   block [0x825F66D0..0x825F673C)
	// 825F66D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F66D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F66D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F66DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F66E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F66E4: 38EBCAA0  addi r7, r11, -0x3560
	ctx.r[7].s64 = ctx.r[11].s64 + -13664;
	// 825F66E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F66EC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 825F66F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F66F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F66F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F66FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6700: 386AE394  addi r3, r10, -0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -7276;
	// 825F6704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F670C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F671C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6728: 4BE706F9  bl 0x82466e20
	ctx.lr = 0x825F672C;
	sub_82466E20(ctx, base);
	// 825F672C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6740 size=108
    let mut pc: u32 = 0x825F6740;
    'dispatch: loop {
        match pc {
            0x825F6740 => {
    //   block [0x825F6740..0x825F67AC)
	// 825F6740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F674C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6754: 38EBCAD0  addi r7, r11, -0x3530
	ctx.r[7].s64 = ctx.r[11].s64 + -13616;
	// 825F6758: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F675C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 825F6760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F676C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6770: 386AE3C4  addi r3, r10, -0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7228;
	// 825F6774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F677C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F678C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6798: 4BE70689  bl 0x82466e20
	ctx.lr = 0x825F679C;
	sub_82466E20(ctx, base);
	// 825F679C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F67A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F67A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F67A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F67B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F67B0 size=112
    let mut pc: u32 = 0x825F67B0;
    'dispatch: loop {
        match pc {
            0x825F67B0 => {
    //   block [0x825F67B0..0x825F6820)
	// 825F67B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F67B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F67B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F67BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F67C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F67C4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F67C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F67CC: 390BCB18  addi r8, r11, -0x34e8
	ctx.r[8].s64 = ctx.r[11].s64 + -13544;
	// 825F67D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F67D4: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 825F67D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F67DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F67E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F67E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F67E8: 386AE3F4  addi r3, r10, -0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -7180;
	// 825F67EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F67F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F67F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F67F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F67FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F680C: 4BE70615  bl 0x82466e20
	ctx.lr = 0x825F6810;
	sub_82466E20(ctx, base);
	// 825F6810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F681C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6820 size=112
    let mut pc: u32 = 0x825F6820;
    'dispatch: loop {
        match pc {
            0x825F6820 => {
    //   block [0x825F6820..0x825F6890)
	// 825F6820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F682C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6830: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6834: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F6838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F683C: 390BCB60  addi r8, r11, -0x34a0
	ctx.r[8].s64 = ctx.r[11].s64 + -13472;
	// 825F6840: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F6844: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 825F6848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F684C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6858: 386AE424  addi r3, r10, -0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -7132;
	// 825F685C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F686C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F687C: 4BE705A5  bl 0x82466e20
	ctx.lr = 0x825F6880;
	sub_82466E20(ctx, base);
	// 825F6880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F688C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6890 size=108
    let mut pc: u32 = 0x825F6890;
    'dispatch: loop {
        match pc {
            0x825F6890 => {
    //   block [0x825F6890..0x825F68FC)
	// 825F6890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F689C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F68A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F68A4: 38EBCBF0  addi r7, r11, -0x3410
	ctx.r[7].s64 = ctx.r[11].s64 + -13328;
	// 825F68A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F68AC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 825F68B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F68B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F68B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F68BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F68C0: 386AE454  addi r3, r10, -0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + -7084;
	// 825F68C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F68C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F68CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F68D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F68D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F68D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F68DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F68E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F68E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F68E8: 4BE70539  bl 0x82466e20
	ctx.lr = 0x825F68EC;
	sub_82466E20(ctx, base);
	// 825F68EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F68F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F68F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F68F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6900 size=108
    let mut pc: u32 = 0x825F6900;
    'dispatch: loop {
        match pc {
            0x825F6900 => {
    //   block [0x825F6900..0x825F696C)
	// 825F6900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F690C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6914: 38EBCC38  addi r7, r11, -0x33c8
	ctx.r[7].s64 = ctx.r[11].s64 + -13256;
	// 825F6918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F691C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 825F6920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F692C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6930: 386AE484  addi r3, r10, -0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7036;
	// 825F6934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F693C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F694C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6958: 4BE704C9  bl 0x82466e20
	ctx.lr = 0x825F695C;
	sub_82466E20(ctx, base);
	// 825F695C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6970 size=108
    let mut pc: u32 = 0x825F6970;
    'dispatch: loop {
        match pc {
            0x825F6970 => {
    //   block [0x825F6970..0x825F69DC)
	// 825F6970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F697C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6984: 38EBCC68  addi r7, r11, -0x3398
	ctx.r[7].s64 = ctx.r[11].s64 + -13208;
	// 825F6988: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F698C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 825F6990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F699C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F69A0: 386AE4B4  addi r3, r10, -0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -6988;
	// 825F69A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F69A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F69AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F69B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F69B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F69B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F69BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F69C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F69C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F69C8: 4BE70459  bl 0x82466e20
	ctx.lr = 0x825F69CC;
	sub_82466E20(ctx, base);
	// 825F69CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F69D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F69D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F69D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F69E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F69E0 size=112
    let mut pc: u32 = 0x825F69E0;
    'dispatch: loop {
        match pc {
            0x825F69E0 => {
    //   block [0x825F69E0..0x825F6A50)
	// 825F69E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F69E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F69E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F69EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F69F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F69F4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F69F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F69FC: 390BCC98  addi r8, r11, -0x3368
	ctx.r[8].s64 = ctx.r[11].s64 + -13160;
	// 825F6A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F6A04: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 825F6A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6A18: 386AE4E4  addi r3, r10, -0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -6940;
	// 825F6A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6A3C: 4BE703E5  bl 0x82466e20
	ctx.lr = 0x825F6A40;
	sub_82466E20(ctx, base);
	// 825F6A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6A50 size=112
    let mut pc: u32 = 0x825F6A50;
    'dispatch: loop {
        match pc {
            0x825F6A50 => {
    //   block [0x825F6A50..0x825F6AC0)
	// 825F6A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6A5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6A60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6A64: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6A6C: 390BCCC8  addi r8, r11, -0x3338
	ctx.r[8].s64 = ctx.r[11].s64 + -13112;
	// 825F6A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6A74: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 825F6A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6A7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6A88: 386AE514  addi r3, r10, -0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + -6892;
	// 825F6A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6AAC: 4BE70375  bl 0x82466e20
	ctx.lr = 0x825F6AB0;
	sub_82466E20(ctx, base);
	// 825F6AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6AC0 size=112
    let mut pc: u32 = 0x825F6AC0;
    'dispatch: loop {
        match pc {
            0x825F6AC0 => {
    //   block [0x825F6AC0..0x825F6B30)
	// 825F6AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6AD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6AD4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6ADC: 390BCCE0  addi r8, r11, -0x3320
	ctx.r[8].s64 = ctx.r[11].s64 + -13088;
	// 825F6AE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6AE4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 825F6AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6AEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6AF8: 386AE544  addi r3, r10, -0x1abc
	ctx.r[3].s64 = ctx.r[10].s64 + -6844;
	// 825F6AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6B1C: 4BE70305  bl 0x82466e20
	ctx.lr = 0x825F6B20;
	sub_82466E20(ctx, base);
	// 825F6B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6B30 size=108
    let mut pc: u32 = 0x825F6B30;
    'dispatch: loop {
        match pc {
            0x825F6B30 => {
    //   block [0x825F6B30..0x825F6B9C)
	// 825F6B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6B3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6B44: 38EBCCF8  addi r7, r11, -0x3308
	ctx.r[7].s64 = ctx.r[11].s64 + -13064;
	// 825F6B48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F6B4C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 825F6B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6B60: 386AE574  addi r3, r10, -0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -6796;
	// 825F6B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6B88: 4BE70299  bl 0x82466e20
	ctx.lr = 0x825F6B8C;
	sub_82466E20(ctx, base);
	// 825F6B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6BA0 size=112
    let mut pc: u32 = 0x825F6BA0;
    'dispatch: loop {
        match pc {
            0x825F6BA0 => {
    //   block [0x825F6BA0..0x825F6C10)
	// 825F6BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6BB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6BB4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6BBC: 390BCD28  addi r8, r11, -0x32d8
	ctx.r[8].s64 = ctx.r[11].s64 + -13016;
	// 825F6BC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F6BC4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 825F6BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6BCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6BD8: 386AE5A4  addi r3, r10, -0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -6748;
	// 825F6BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6BFC: 4BE70225  bl 0x82466e20
	ctx.lr = 0x825F6C00;
	sub_82466E20(ctx, base);
	// 825F6C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6C10 size=108
    let mut pc: u32 = 0x825F6C10;
    'dispatch: loop {
        match pc {
            0x825F6C10 => {
    //   block [0x825F6C10..0x825F6C7C)
	// 825F6C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6C1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6C24: 38EBCD40  addi r7, r11, -0x32c0
	ctx.r[7].s64 = ctx.r[11].s64 + -12992;
	// 825F6C28: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 825F6C2C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 825F6C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6C40: 386AE5D4  addi r3, r10, -0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6700;
	// 825F6C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6C68: 4BE701B9  bl 0x82466e20
	ctx.lr = 0x825F6C6C;
	sub_82466E20(ctx, base);
	// 825F6C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6C80 size=112
    let mut pc: u32 = 0x825F6C80;
    'dispatch: loop {
        match pc {
            0x825F6C80 => {
    //   block [0x825F6C80..0x825F6CF0)
	// 825F6C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6C8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6C90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6C94: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6C9C: 390BCE30  addi r8, r11, -0x31d0
	ctx.r[8].s64 = ctx.r[11].s64 + -12752;
	// 825F6CA0: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 825F6CA4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 825F6CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6CAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6CB8: 386AE604  addi r3, r10, -0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6652;
	// 825F6CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6CDC: 4BE70145  bl 0x82466e20
	ctx.lr = 0x825F6CE0;
	sub_82466E20(ctx, base);
	// 825F6CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6CF0 size=108
    let mut pc: u32 = 0x825F6CF0;
    'dispatch: loop {
        match pc {
            0x825F6CF0 => {
    //   block [0x825F6CF0..0x825F6D5C)
	// 825F6CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6CFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6D04: 38EBCFE0  addi r7, r11, -0x3020
	ctx.r[7].s64 = ctx.r[11].s64 + -12320;
	// 825F6D08: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 825F6D0C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 825F6D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6D14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6D20: 386AE634  addi r3, r10, -0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6604;
	// 825F6D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6D48: 4BE700D9  bl 0x82466e20
	ctx.lr = 0x825F6D4C;
	sub_82466E20(ctx, base);
	// 825F6D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6D60 size=112
    let mut pc: u32 = 0x825F6D60;
    'dispatch: loop {
        match pc {
            0x825F6D60 => {
    //   block [0x825F6D60..0x825F6DD0)
	// 825F6D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6D6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6D70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6D74: 38AAD194  addi r5, r10, -0x2e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -11884;
	// 825F6D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6D7C: 390BD178  addi r8, r11, -0x2e88
	ctx.r[8].s64 = ctx.r[11].s64 + -11912;
	// 825F6D80: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 825F6D84: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 825F6D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6D8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6D98: 386AE664  addi r3, r10, -0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + -6556;
	// 825F6D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6DBC: 4BE70065  bl 0x82466e20
	ctx.lr = 0x825F6DC0;
	sub_82466E20(ctx, base);
	// 825F6DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6DD0 size=100
    let mut pc: u32 = 0x825F6DD0;
    'dispatch: loop {
        match pc {
            0x825F6DD0 => {
    //   block [0x825F6DD0..0x825F6E34)
	// 825F6DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6DDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6DE4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6DF0: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 825F6DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6E04: 386AE694  addi r3, r10, -0x196c
	ctx.r[3].s64 = ctx.r[10].s64 + -6508;
	// 825F6E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6E10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F6E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6E18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F6E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6E20: 4BE70001  bl 0x82466e20
	ctx.lr = 0x825F6E24;
	sub_82466E20(ctx, base);
	// 825F6E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6E38 size=112
    let mut pc: u32 = 0x825F6E38;
    'dispatch: loop {
        match pc {
            0x825F6E38 => {
    //   block [0x825F6E38..0x825F6EA8)
	// 825F6E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6E48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6E4C: 38AAE694  addi r5, r10, -0x196c
	ctx.r[5].s64 = ctx.r[10].s64 + -6508;
	// 825F6E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6E54: 390BD3D0  addi r8, r11, -0x2c30
	ctx.r[8].s64 = ctx.r[11].s64 + -11312;
	// 825F6E58: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F6E5C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 825F6E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6E64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6E70: 386AE6C4  addi r3, r10, -0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + -6460;
	// 825F6E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6E94: 4BE6FF8D  bl 0x82466e20
	ctx.lr = 0x825F6E98;
	sub_82466E20(ctx, base);
	// 825F6E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6EA8 size=100
    let mut pc: u32 = 0x825F6EA8;
    'dispatch: loop {
        match pc {
            0x825F6EA8 => {
    //   block [0x825F6EA8..0x825F6F0C)
	// 825F6EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6EB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6EBC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F6EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6EC8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 825F6ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6EDC: 386AE6F4  addi r3, r10, -0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + -6412;
	// 825F6EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6EE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6EE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F6EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6EF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F6EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6EF8: 4BE6FF29  bl 0x82466e20
	ctx.lr = 0x825F6EFC;
	sub_82466E20(ctx, base);
	// 825F6EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6F10 size=108
    let mut pc: u32 = 0x825F6F10;
    'dispatch: loop {
        match pc {
            0x825F6F10 => {
    //   block [0x825F6F10..0x825F6F7C)
	// 825F6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6F1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6F24: 38EBD448  addi r7, r11, -0x2bb8
	ctx.r[7].s64 = ctx.r[11].s64 + -11192;
	// 825F6F28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F6F2C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 825F6F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6F34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F6F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6F40: 386AE724  addi r3, r10, -0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6364;
	// 825F6F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F6F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6F68: 4BE6FEB9  bl 0x82466e20
	ctx.lr = 0x825F6F6C;
	sub_82466E20(ctx, base);
	// 825F6F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6F80 size=112
    let mut pc: u32 = 0x825F6F80;
    'dispatch: loop {
        match pc {
            0x825F6F80 => {
    //   block [0x825F6F80..0x825F6FF0)
	// 825F6F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6F90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6F94: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 825F6F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6F9C: 390BD490  addi r8, r11, -0x2b70
	ctx.r[8].s64 = ctx.r[11].s64 + -11120;
	// 825F6FA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F6FA4: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 825F6FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6FAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6FB8: 386AE754  addi r3, r10, -0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6316;
	// 825F6FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F6FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F6FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6FDC: 4BE6FE45  bl 0x82466e20
	ctx.lr = 0x825F6FE0;
	sub_82466E20(ctx, base);
	// 825F6FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6FF0 size=100
    let mut pc: u32 = 0x825F6FF0;
    'dispatch: loop {
        match pc {
            0x825F6FF0 => {
    //   block [0x825F6FF0..0x825F7054)
	// 825F6FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6FFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7004: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F700C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7010: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 825F7014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F701C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7024: 386AE784  addi r3, r10, -0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + -6268;
	// 825F7028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F702C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7030: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F7034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7038: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F703C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7040: 4BE6FDE1  bl 0x82466e20
	ctx.lr = 0x825F7044;
	sub_82466E20(ctx, base);
	// 825F7044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F704C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7058 size=100
    let mut pc: u32 = 0x825F7058;
    'dispatch: loop {
        match pc {
            0x825F7058 => {
    //   block [0x825F7058..0x825F70BC)
	// 825F7058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F705C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F706C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7078: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 825F707C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F708C: 386AE7B4  addi r3, r10, -0x184c
	ctx.r[3].s64 = ctx.r[10].s64 + -6220;
	// 825F7090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7098: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F709C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F70A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F70A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F70A8: 4BE6FD79  bl 0x82466e20
	ctx.lr = 0x825F70AC;
	sub_82466E20(ctx, base);
	// 825F70AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F70B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F70B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F70B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F70C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F70C0 size=112
    let mut pc: u32 = 0x825F70C0;
    'dispatch: loop {
        match pc {
            0x825F70C0 => {
    //   block [0x825F70C0..0x825F7130)
	// 825F70C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F70C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F70C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F70CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F70D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F70D4: 38AAE784  addi r5, r10, -0x187c
	ctx.r[5].s64 = ctx.r[10].s64 + -6268;
	// 825F70D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F70DC: 390BD4C0  addi r8, r11, -0x2b40
	ctx.r[8].s64 = ctx.r[11].s64 + -11072;
	// 825F70E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F70E4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 825F70E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F70EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F70F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F70F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F70F8: 386AE7E4  addi r3, r10, -0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + -6172;
	// 825F70FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F710C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F711C: 4BE6FD05  bl 0x82466e20
	ctx.lr = 0x825F7120;
	sub_82466E20(ctx, base);
	// 825F7120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F712C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7130 size=112
    let mut pc: u32 = 0x825F7130;
    'dispatch: loop {
        match pc {
            0x825F7130 => {
    //   block [0x825F7130..0x825F71A0)
	// 825F7130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F713C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7140: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7144: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 825F7148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F714C: 390BD520  addi r8, r11, -0x2ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -10976;
	// 825F7150: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F7154: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 825F7158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F715C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7168: 386AE814  addi r3, r10, -0x17ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6124;
	// 825F716C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F717C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F718C: 4BE6FC95  bl 0x82466e20
	ctx.lr = 0x825F7190;
	sub_82466E20(ctx, base);
	// 825F7190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F719C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F71A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F71A0 size=100
    let mut pc: u32 = 0x825F71A0;
    'dispatch: loop {
        match pc {
            0x825F71A0 => {
    //   block [0x825F71A0..0x825F7204)
	// 825F71A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F71A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F71A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F71AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F71B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F71B4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F71B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F71BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F71C0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 825F71C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F71C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F71CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F71D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F71D4: 386AE844  addi r3, r10, -0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6076;
	// 825F71D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F71DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F71E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F71E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F71E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F71EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F71F0: 4BE6FC31  bl 0x82466e20
	ctx.lr = 0x825F71F4;
	sub_82466E20(ctx, base);
	// 825F71F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F71F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F71FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7208 size=112
    let mut pc: u32 = 0x825F7208;
    'dispatch: loop {
        match pc {
            0x825F7208 => {
    //   block [0x825F7208..0x825F7278)
	// 825F7208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7214: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7218: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F721C: 38AAE844  addi r5, r10, -0x17bc
	ctx.r[5].s64 = ctx.r[10].s64 + -6076;
	// 825F7220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7224: 390BD580  addi r8, r11, -0x2a80
	ctx.r[8].s64 = ctx.r[11].s64 + -10880;
	// 825F7228: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825F722C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 825F7230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F723C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7240: 386AE874  addi r3, r10, -0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + -6028;
	// 825F7244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F724C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F725C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7264: 4BE6FBBD  bl 0x82466e20
	ctx.lr = 0x825F7268;
	sub_82466E20(ctx, base);
	// 825F7268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F726C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7278 size=108
    let mut pc: u32 = 0x825F7278;
    'dispatch: loop {
        match pc {
            0x825F7278 => {
    //   block [0x825F7278..0x825F72E4)
	// 825F7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7284: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F728C: 38EBD670  addi r7, r11, -0x2990
	ctx.r[7].s64 = ctx.r[11].s64 + -10640;
	// 825F7290: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 825F7294: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 825F7298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F729C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F72A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F72A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F72A8: 386AE8A4  addi r3, r10, -0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + -5980;
	// 825F72AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F72B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F72B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F72B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F72BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F72C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F72C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F72C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F72CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F72D0: 4BE6FB51  bl 0x82466e20
	ctx.lr = 0x825F72D4;
	sub_82466E20(ctx, base);
	// 825F72D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F72D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F72DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F72E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F72E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F72E8 size=108
    let mut pc: u32 = 0x825F72E8;
    'dispatch: loop {
        match pc {
            0x825F72E8 => {
    //   block [0x825F72E8..0x825F7354)
	// 825F72E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F72EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F72F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F72F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F72F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F72FC: 38EBD760  addi r7, r11, -0x28a0
	ctx.r[7].s64 = ctx.r[11].s64 + -10400;
	// 825F7300: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F7304: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 825F7308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F730C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7318: 386AE8D4  addi r3, r10, -0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + -5932;
	// 825F731C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F732C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F733C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7340: 4BE6FAE1  bl 0x82466e20
	ctx.lr = 0x825F7344;
	sub_82466E20(ctx, base);
	// 825F7344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F734C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7358 size=108
    let mut pc: u32 = 0x825F7358;
    'dispatch: loop {
        match pc {
            0x825F7358 => {
    //   block [0x825F7358..0x825F73C4)
	// 825F7358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F735C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7364: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F736C: 38EBD7A8  addi r7, r11, -0x2858
	ctx.r[7].s64 = ctx.r[11].s64 + -10328;
	// 825F7370: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F7374: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 825F7378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F737C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7388: 386AE904  addi r3, r10, -0x16fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5884;
	// 825F738C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F739C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F73A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F73A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F73A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F73AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F73B0: 4BE6FA71  bl 0x82466e20
	ctx.lr = 0x825F73B4;
	sub_82466E20(ctx, base);
	// 825F73B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F73B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F73BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F73C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F73C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F73C8 size=108
    let mut pc: u32 = 0x825F73C8;
    'dispatch: loop {
        match pc {
            0x825F73C8 => {
    //   block [0x825F73C8..0x825F7434)
	// 825F73C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F73CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F73D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F73D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F73D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F73DC: 38EBD880  addi r7, r11, -0x2780
	ctx.r[7].s64 = ctx.r[11].s64 + -10112;
	// 825F73E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F73E4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 825F73E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F73EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F73F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F73F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F73F8: 386AE934  addi r3, r10, -0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5836;
	// 825F73FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F740C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F741C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7420: 4BE6FA01  bl 0x82466e20
	ctx.lr = 0x825F7424;
	sub_82466E20(ctx, base);
	// 825F7424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F742C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7438 size=100
    let mut pc: u32 = 0x825F7438;
    'dispatch: loop {
        match pc {
            0x825F7438 => {
    //   block [0x825F7438..0x825F749C)
	// 825F7438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F743C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7444: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F744C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7458: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 825F745C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F746C: 386AE964  addi r3, r10, -0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + -5788;
	// 825F7470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F747C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F7484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7488: 4BE6F999  bl 0x82466e20
	ctx.lr = 0x825F748C;
	sub_82466E20(ctx, base);
	// 825F748C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F74A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F74A0 size=112
    let mut pc: u32 = 0x825F74A0;
    'dispatch: loop {
        match pc {
            0x825F74A0 => {
    //   block [0x825F74A0..0x825F7510)
	// 825F74A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F74A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F74A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F74AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F74B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F74B4: 38AAE964  addi r5, r10, -0x169c
	ctx.r[5].s64 = ctx.r[10].s64 + -5788;
	// 825F74B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F74BC: 390BD898  addi r8, r11, -0x2768
	ctx.r[8].s64 = ctx.r[11].s64 + -10088;
	// 825F74C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F74C4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 825F74C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F74CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F74D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F74D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F74D8: 386AE994  addi r3, r10, -0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + -5740;
	// 825F74DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F74E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F74E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F74E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F74EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F74F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F74F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F74F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F74FC: 4BE6F925  bl 0x82466e20
	ctx.lr = 0x825F7500;
	sub_82466E20(ctx, base);
	// 825F7500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F750C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7510 size=108
    let mut pc: u32 = 0x825F7510;
    'dispatch: loop {
        match pc {
            0x825F7510 => {
    //   block [0x825F7510..0x825F757C)
	// 825F7510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F751C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7524: 38EBD8E0  addi r7, r11, -0x2720
	ctx.r[7].s64 = ctx.r[11].s64 + -10016;
	// 825F7528: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F752C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 825F7530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F753C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7540: 386AE9C4  addi r3, r10, -0x163c
	ctx.r[3].s64 = ctx.r[10].s64 + -5692;
	// 825F7544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F754C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F755C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7568: 4BE6F8B9  bl 0x82466e20
	ctx.lr = 0x825F756C;
	sub_82466E20(ctx, base);
	// 825F756C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7580 size=112
    let mut pc: u32 = 0x825F7580;
    'dispatch: loop {
        match pc {
            0x825F7580 => {
    //   block [0x825F7580..0x825F75F0)
	// 825F7580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F758C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7590: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7594: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F759C: 390BD928  addi r8, r11, -0x26d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9944;
	// 825F75A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F75A4: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 825F75A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F75AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F75B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F75B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F75B8: 386AE9F4  addi r3, r10, -0x160c
	ctx.r[3].s64 = ctx.r[10].s64 + -5644;
	// 825F75BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F75C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F75C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F75C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F75CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F75D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F75D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F75D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F75DC: 4BE6F845  bl 0x82466e20
	ctx.lr = 0x825F75E0;
	sub_82466E20(ctx, base);
	// 825F75E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F75E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F75E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F75EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F75F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F75F0 size=108
    let mut pc: u32 = 0x825F75F0;
    'dispatch: loop {
        match pc {
            0x825F75F0 => {
    //   block [0x825F75F0..0x825F765C)
	// 825F75F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F75F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F75F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F75FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7604: 38EBD940  addi r7, r11, -0x26c0
	ctx.r[7].s64 = ctx.r[11].s64 + -9920;
	// 825F7608: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F760C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 825F7610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F761C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7620: 386AEA24  addi r3, r10, -0x15dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5596;
	// 825F7624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F762C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F763C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7648: 4BE6F7D9  bl 0x82466e20
	ctx.lr = 0x825F764C;
	sub_82466E20(ctx, base);
	// 825F764C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7660 size=112
    let mut pc: u32 = 0x825F7660;
    'dispatch: loop {
        match pc {
            0x825F7660 => {
    //   block [0x825F7660..0x825F76D0)
	// 825F7660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F766C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7670: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7674: 38AAE9F4  addi r5, r10, -0x160c
	ctx.r[5].s64 = ctx.r[10].s64 + -5644;
	// 825F7678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F767C: 390BD988  addi r8, r11, -0x2678
	ctx.r[8].s64 = ctx.r[11].s64 + -9848;
	// 825F7680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F7684: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 825F7688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F768C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7698: 386AEA54  addi r3, r10, -0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5548;
	// 825F769C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F76A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F76A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F76A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F76AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F76B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F76B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F76B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F76BC: 4BE6F765  bl 0x82466e20
	ctx.lr = 0x825F76C0;
	sub_82466E20(ctx, base);
	// 825F76C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F76C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F76C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F76CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F76D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F76D0 size=100
    let mut pc: u32 = 0x825F76D0;
    'dispatch: loop {
        match pc {
            0x825F76D0 => {
    //   block [0x825F76D0..0x825F7734)
	// 825F76D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F76D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F76D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F76DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F76E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F76E4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F76E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F76EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F76F0: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 825F76F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F76F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F76FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7704: 386AEA84  addi r3, r10, -0x157c
	ctx.r[3].s64 = ctx.r[10].s64 + -5500;
	// 825F7708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F770C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7710: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F7714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7718: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F771C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7720: 4BE6F701  bl 0x82466e20
	ctx.lr = 0x825F7724;
	sub_82466E20(ctx, base);
	// 825F7724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F772C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7738 size=112
    let mut pc: u32 = 0x825F7738;
    'dispatch: loop {
        match pc {
            0x825F7738 => {
    //   block [0x825F7738..0x825F77A8)
	// 825F7738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F773C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7748: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F774C: 38AAEA84  addi r5, r10, -0x157c
	ctx.r[5].s64 = ctx.r[10].s64 + -5500;
	// 825F7750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7754: 390BD9A0  addi r8, r11, -0x2660
	ctx.r[8].s64 = ctx.r[11].s64 + -9824;
	// 825F7758: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F775C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 825F7760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F776C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7770: 386AEAB4  addi r3, r10, -0x154c
	ctx.r[3].s64 = ctx.r[10].s64 + -5452;
	// 825F7774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F777C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F778C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7794: 4BE6F68D  bl 0x82466e20
	ctx.lr = 0x825F7798;
	sub_82466E20(ctx, base);
	// 825F7798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F779C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F77A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F77A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F77A8 size=108
    let mut pc: u32 = 0x825F77A8;
    'dispatch: loop {
        match pc {
            0x825F77A8 => {
    //   block [0x825F77A8..0x825F7814)
	// 825F77A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F77AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F77B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F77B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F77B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F77BC: 38EBDA48  addi r7, r11, -0x25b8
	ctx.r[7].s64 = ctx.r[11].s64 + -9656;
	// 825F77C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F77C4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 825F77C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F77CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F77D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F77D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F77D8: 386AEAE4  addi r3, r10, -0x151c
	ctx.r[3].s64 = ctx.r[10].s64 + -5404;
	// 825F77DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F77E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F77E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F77E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F77EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F77F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F77F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F77F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F77FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7800: 4BE6F621  bl 0x82466e20
	ctx.lr = 0x825F7804;
	sub_82466E20(ctx, base);
	// 825F7804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F780C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7818 size=112
    let mut pc: u32 = 0x825F7818;
    'dispatch: loop {
        match pc {
            0x825F7818 => {
    //   block [0x825F7818..0x825F7888)
	// 825F7818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F781C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7824: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7828: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F782C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7834: 390BDA78  addi r8, r11, -0x2588
	ctx.r[8].s64 = ctx.r[11].s64 + -9608;
	// 825F7838: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F783C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 825F7840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F784C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7850: 386AEB14  addi r3, r10, -0x14ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5356;
	// 825F7854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F785C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F786C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7874: 4BE6F5AD  bl 0x82466e20
	ctx.lr = 0x825F7878;
	sub_82466E20(ctx, base);
	// 825F7878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F787C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7888 size=112
    let mut pc: u32 = 0x825F7888;
    'dispatch: loop {
        match pc {
            0x825F7888 => {
    //   block [0x825F7888..0x825F78F8)
	// 825F7888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F788C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7898: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F789C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F78A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F78A4: 390BDAC0  addi r8, r11, -0x2540
	ctx.r[8].s64 = ctx.r[11].s64 + -9536;
	// 825F78A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F78AC: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 825F78B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F78B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F78B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F78BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F78C0: 386AEB44  addi r3, r10, -0x14bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5308;
	// 825F78C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F78C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F78CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F78D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F78D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F78D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F78DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F78E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F78E4: 4BE6F53D  bl 0x82466e20
	ctx.lr = 0x825F78E8;
	sub_82466E20(ctx, base);
	// 825F78E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F78EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F78F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F78F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F78F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F78F8 size=100
    let mut pc: u32 = 0x825F78F8;
    'dispatch: loop {
        match pc {
            0x825F78F8 => {
    //   block [0x825F78F8..0x825F795C)
	// 825F78F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F78FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F790C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7918: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 825F791C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F792C: 386AEB74  addi r3, r10, -0x148c
	ctx.r[3].s64 = ctx.r[10].s64 + -5260;
	// 825F7930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7938: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F793C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7940: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F7944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7948: 4BE6F4D9  bl 0x82466e20
	ctx.lr = 0x825F794C;
	sub_82466E20(ctx, base);
	// 825F794C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7960 size=112
    let mut pc: u32 = 0x825F7960;
    'dispatch: loop {
        match pc {
            0x825F7960 => {
    //   block [0x825F7960..0x825F79D0)
	// 825F7960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F796C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7970: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7974: 38AAEB74  addi r5, r10, -0x148c
	ctx.r[5].s64 = ctx.r[10].s64 + -5260;
	// 825F7978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F797C: 390BDB08  addi r8, r11, -0x24f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9464;
	// 825F7980: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F7984: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 825F7988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F798C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7998: 386AEBA4  addi r3, r10, -0x145c
	ctx.r[3].s64 = ctx.r[10].s64 + -5212;
	// 825F799C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F79A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F79A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F79A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F79AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F79B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F79B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F79B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F79BC: 4BE6F465  bl 0x82466e20
	ctx.lr = 0x825F79C0;
	sub_82466E20(ctx, base);
	// 825F79C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F79C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F79C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F79CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F79D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F79D0 size=112
    let mut pc: u32 = 0x825F79D0;
    'dispatch: loop {
        match pc {
            0x825F79D0 => {
    //   block [0x825F79D0..0x825F7A40)
	// 825F79D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F79D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F79D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F79DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F79E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F79E4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F79E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F79EC: 390BDB50  addi r8, r11, -0x24b0
	ctx.r[8].s64 = ctx.r[11].s64 + -9392;
	// 825F79F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F79F4: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 825F79F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F79FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7A08: 386AEBD4  addi r3, r10, -0x142c
	ctx.r[3].s64 = ctx.r[10].s64 + -5164;
	// 825F7A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7A2C: 4BE6F3F5  bl 0x82466e20
	ctx.lr = 0x825F7A30;
	sub_82466E20(ctx, base);
	// 825F7A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7A40 size=112
    let mut pc: u32 = 0x825F7A40;
    'dispatch: loop {
        match pc {
            0x825F7A40 => {
    //   block [0x825F7A40..0x825F7AB0)
	// 825F7A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7A4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7A50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7A54: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F7A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7A5C: 390BDB68  addi r8, r11, -0x2498
	ctx.r[8].s64 = ctx.r[11].s64 + -9368;
	// 825F7A60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F7A64: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 825F7A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7A6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7A78: 386AEC04  addi r3, r10, -0x13fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5116;
	// 825F7A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7A8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F7A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7A9C: 4BE6F385  bl 0x82466e20
	ctx.lr = 0x825F7AA0;
	sub_82466E20(ctx, base);
	// 825F7AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7AB0 size=112
    let mut pc: u32 = 0x825F7AB0;
    'dispatch: loop {
        match pc {
            0x825F7AB0 => {
    //   block [0x825F7AB0..0x825F7B20)
	// 825F7AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7ABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7AC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7AC4: 38AAEBD4  addi r5, r10, -0x142c
	ctx.r[5].s64 = ctx.r[10].s64 + -5164;
	// 825F7AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7ACC: 390BDB80  addi r8, r11, -0x2480
	ctx.r[8].s64 = ctx.r[11].s64 + -9344;
	// 825F7AD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F7AD4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 825F7AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7ADC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7AE8: 386AEC34  addi r3, r10, -0x13cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5068;
	// 825F7AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7B0C: 4BE6F315  bl 0x82466e20
	ctx.lr = 0x825F7B10;
	sub_82466E20(ctx, base);
	// 825F7B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7B20 size=72
    let mut pc: u32 = 0x825F7B20;
    'dispatch: loop {
        match pc {
            0x825F7B20 => {
    //   block [0x825F7B20..0x825F7B68)
	// 825F7B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7B2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F7B30: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 825F7B34: 38CB0DA8  addi r6, r11, 0xda8
	ctx.r[6].s64 = ctx.r[11].s64 + 3496;
	// 825F7B38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F7B3C: 388B2458  addi r4, r11, 0x2458
	ctx.r[4].s64 = ctx.r[11].s64 + 9304;
	// 825F7B40: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F7B44: 386BEC64  addi r3, r11, -0x139c
	ctx.r[3].s64 = ctx.r[11].s64 + -5020;
	// 825F7B48: 4BE83F41  bl 0x8247ba88
	ctx.lr = 0x825F7B4C;
	sub_8247BA88(ctx, base);
	// 825F7B4C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825F7B50: 386BCCF0  addi r3, r11, -0x3310
	ctx.r[3].s64 = ctx.r[11].s64 + -13072;
	// 825F7B54: 4BF3AFE5  bl 0x82532b38
	ctx.lr = 0x825F7B58;
	sub_82532B38(ctx, base);
	// 825F7B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825F7B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7B68 size=108
    let mut pc: u32 = 0x825F7B68;
    'dispatch: loop {
        match pc {
            0x825F7B68 => {
    //   block [0x825F7B68..0x825F7BD4)
	// 825F7B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7B74: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7B78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7B7C: 38EBF518  addi r7, r11, -0xae8
	ctx.r[7].s64 = ctx.r[11].s64 + -2792;
	// 825F7B80: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F7B84: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 825F7B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7B98: 386AEC7C  addi r3, r10, -0x1384
	ctx.r[3].s64 = ctx.r[10].s64 + -4996;
	// 825F7B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7BC0: 4BE6F261  bl 0x82466e20
	ctx.lr = 0x825F7BC4;
	sub_82466E20(ctx, base);
	// 825F7BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F7BD8 size=24
    let mut pc: u32 = 0x825F7BD8;
    'dispatch: loop {
        match pc {
            0x825F7BD8 => {
    //   block [0x825F7BD8..0x825F7BF0)
	// 825F7BD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7BDC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F7BE0: 394A8B68  addi r10, r10, -0x7498
	ctx.r[10].s64 = ctx.r[10].s64 + -29848;
	// 825F7BE4: 816BF590  lwz r11, -0xa70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2672 as u32) ) } as u64;
	// 825F7BE8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825F7BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7BF0 size=112
    let mut pc: u32 = 0x825F7BF0;
    'dispatch: loop {
        match pc {
            0x825F7BF0 => {
    //   block [0x825F7BF0..0x825F7C60)
	// 825F7BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7BFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7C00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F7C04: 392A7C04  addi r9, r10, 0x7c04
	ctx.r[9].s64 = ctx.r[10].s64 + 31748;
	// 825F7C08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7C0C: 390B8B68  addi r8, r11, -0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + -29848;
	// 825F7C10: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F7C14: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 825F7C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7C1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7C28: 386AECAC  addi r3, r10, -0x1354
	ctx.r[3].s64 = ctx.r[10].s64 + -4948;
	// 825F7C2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F7C30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F7C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7C4C: 4BE6F1D5  bl 0x82466e20
	ctx.lr = 0x825F7C50;
	sub_82466E20(ctx, base);
	// 825F7C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7C60 size=108
    let mut pc: u32 = 0x825F7C60;
    'dispatch: loop {
        match pc {
            0x825F7C60 => {
    //   block [0x825F7C60..0x825F7CCC)
	// 825F7C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7C6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7C70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7C74: 38EBF594  addi r7, r11, -0xa6c
	ctx.r[7].s64 = ctx.r[11].s64 + -2668;
	// 825F7C78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F7C7C: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 825F7C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7C90: 386AECDC  addi r3, r10, -0x1324
	ctx.r[3].s64 = ctx.r[10].s64 + -4900;
	// 825F7C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7CB8: 4BE6F169  bl 0x82466e20
	ctx.lr = 0x825F7CBC;
	sub_82466E20(ctx, base);
	// 825F7CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7CD0 size=108
    let mut pc: u32 = 0x825F7CD0;
    'dispatch: loop {
        match pc {
            0x825F7CD0 => {
    //   block [0x825F7CD0..0x825F7D3C)
	// 825F7CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7CDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7CE4: 38EBF5C4  addi r7, r11, -0xa3c
	ctx.r[7].s64 = ctx.r[11].s64 + -2620;
	// 825F7CE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F7CEC: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 825F7CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7D00: 386AED0C  addi r3, r10, -0x12f4
	ctx.r[3].s64 = ctx.r[10].s64 + -4852;
	// 825F7D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7D28: 4BE6F0F9  bl 0x82466e20
	ctx.lr = 0x825F7D2C;
	sub_82466E20(ctx, base);
	// 825F7D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F7D40 size=24
    let mut pc: u32 = 0x825F7D40;
    'dispatch: loop {
        match pc {
            0x825F7D40 => {
    //   block [0x825F7D40..0x825F7D58)
	// 825F7D40: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7D44: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F7D48: 394A8BB0  addi r10, r10, -0x7450
	ctx.r[10].s64 = ctx.r[10].s64 + -29776;
	// 825F7D4C: 816BF5F4  lwz r11, -0xa0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2572 as u32) ) } as u64;
	// 825F7D50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F7D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7D58 size=116
    let mut pc: u32 = 0x825F7D58;
    'dispatch: loop {
        match pc {
            0x825F7D58 => {
    //   block [0x825F7D58..0x825F7DCC)
	// 825F7D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7D64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F7D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F7D6C: 390B8BB0  addi r8, r11, -0x7450
	ctx.r[8].s64 = ctx.r[11].s64 + -29776;
	// 825F7D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7D74: 392A7C40  addi r9, r10, 0x7c40
	ctx.r[9].s64 = ctx.r[10].s64 + 31808;
	// 825F7D78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7D7C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F7D80: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F7D84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7D8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7D9C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F7DA0: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 825F7DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F7DA8: 386BED3C  addi r3, r11, -0x12c4
	ctx.r[3].s64 = ctx.r[11].s64 + -4804;
	// 825F7DAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F7DB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7DB8: 4BE6F069  bl 0x82466e20
	ctx.lr = 0x825F7DBC;
	sub_82466E20(ctx, base);
	// 825F7DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7DD0 size=108
    let mut pc: u32 = 0x825F7DD0;
    'dispatch: loop {
        match pc {
            0x825F7DD0 => {
    //   block [0x825F7DD0..0x825F7E3C)
	// 825F7DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7DDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7DE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7DE4: 38EBF5F8  addi r7, r11, -0xa08
	ctx.r[7].s64 = ctx.r[11].s64 + -2568;
	// 825F7DE8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F7DEC: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 825F7DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7E00: 386AED6C  addi r3, r10, -0x1294
	ctx.r[3].s64 = ctx.r[10].s64 + -4756;
	// 825F7E04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7E28: 4BE6EFF9  bl 0x82466e20
	ctx.lr = 0x825F7E2C;
	sub_82466E20(ctx, base);
	// 825F7E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7E40 size=112
    let mut pc: u32 = 0x825F7E40;
    'dispatch: loop {
        match pc {
            0x825F7E40 => {
    //   block [0x825F7E40..0x825F7EB0)
	// 825F7E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7E4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7E50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7E54: 38AAED3C  addi r5, r10, -0x12c4
	ctx.r[5].s64 = ctx.r[10].s64 + -4804;
	// 825F7E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7E5C: 390BF688  addi r8, r11, -0x978
	ctx.r[8].s64 = ctx.r[11].s64 + -2424;
	// 825F7E60: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825F7E64: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825F7E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7E6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7E78: 386AED9C  addi r3, r10, -0x1264
	ctx.r[3].s64 = ctx.r[10].s64 + -4708;
	// 825F7E7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7E9C: 4BE6EF85  bl 0x82466e20
	ctx.lr = 0x825F7EA0;
	sub_82466E20(ctx, base);
	// 825F7EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7EB0 size=112
    let mut pc: u32 = 0x825F7EB0;
    'dispatch: loop {
        match pc {
            0x825F7EB0 => {
    //   block [0x825F7EB0..0x825F7F20)
	// 825F7EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7EBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7EC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7EC4: 38AAED3C  addi r5, r10, -0x12c4
	ctx.r[5].s64 = ctx.r[10].s64 + -4804;
	// 825F7EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7ECC: 390BF7A8  addi r8, r11, -0x858
	ctx.r[8].s64 = ctx.r[11].s64 + -2136;
	// 825F7ED0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F7ED4: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 825F7ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7EDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F7EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7EE8: 386AEDCC  addi r3, r10, -0x1234
	ctx.r[3].s64 = ctx.r[10].s64 + -4660;
	// 825F7EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F7EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7F0C: 4BE6EF15  bl 0x82466e20
	ctx.lr = 0x825F7F10;
	sub_82466E20(ctx, base);
	// 825F7F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7F20 size=108
    let mut pc: u32 = 0x825F7F20;
    'dispatch: loop {
        match pc {
            0x825F7F20 => {
    //   block [0x825F7F20..0x825F7F8C)
	// 825F7F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7F2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7F34: 38EBF7C0  addi r7, r11, -0x840
	ctx.r[7].s64 = ctx.r[11].s64 + -2112;
	// 825F7F38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F7F3C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825F7F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7F44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7F50: 386AEDFC  addi r3, r10, -0x1204
	ctx.r[3].s64 = ctx.r[10].s64 + -4612;
	// 825F7F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7F78: 4BE6EEA9  bl 0x82466e20
	ctx.lr = 0x825F7F7C;
	sub_82466E20(ctx, base);
	// 825F7F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F7F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F7F90 size=108
    let mut pc: u32 = 0x825F7F90;
    'dispatch: loop {
        match pc {
            0x825F7F90 => {
    //   block [0x825F7F90..0x825F7FFC)
	// 825F7F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F7F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F7F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F7F9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F7FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F7FA4: 38EBF898  addi r7, r11, -0x768
	ctx.r[7].s64 = ctx.r[11].s64 + -1896;
	// 825F7FA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F7FAC: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 825F7FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F7FB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F7FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F7FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F7FC0: 386AEE2C  addi r3, r10, -0x11d4
	ctx.r[3].s64 = ctx.r[10].s64 + -4564;
	// 825F7FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F7FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F7FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F7FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F7FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F7FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F7FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F7FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F7FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F7FE8: 4BE6EE39  bl 0x82466e20
	ctx.lr = 0x825F7FEC;
	sub_82466E20(ctx, base);
	// 825F7FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F7FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F7FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F7FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8000 size=112
    let mut pc: u32 = 0x825F8000;
    'dispatch: loop {
        match pc {
            0x825F8000 => {
    //   block [0x825F8000..0x825F8070)
	// 825F8000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F800C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8010: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8014: 38AAED3C  addi r5, r10, -0x12c4
	ctx.r[5].s64 = ctx.r[10].s64 + -4804;
	// 825F8018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F801C: 390BF928  addi r8, r11, -0x6d8
	ctx.r[8].s64 = ctx.r[11].s64 + -1752;
	// 825F8020: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825F8024: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 825F8028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F802C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8038: 386AEE5C  addi r3, r10, -0x11a4
	ctx.r[3].s64 = ctx.r[10].s64 + -4516;
	// 825F803C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F804C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F805C: 4BE6EDC5  bl 0x82466e20
	ctx.lr = 0x825F8060;
	sub_82466E20(ctx, base);
	// 825F8060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8070 size=108
    let mut pc: u32 = 0x825F8070;
    'dispatch: loop {
        match pc {
            0x825F8070 => {
    //   block [0x825F8070..0x825F80DC)
	// 825F8070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F807C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8084: 38EBFA18  addi r7, r11, -0x5e8
	ctx.r[7].s64 = ctx.r[11].s64 + -1512;
	// 825F8088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F808C: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 825F8090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F809C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F80A0: 386AEE8C  addi r3, r10, -0x1174
	ctx.r[3].s64 = ctx.r[10].s64 + -4468;
	// 825F80A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F80A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F80AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F80B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F80B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F80B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F80BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F80C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F80C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F80C8: 4BE6ED59  bl 0x82466e20
	ctx.lr = 0x825F80CC;
	sub_82466E20(ctx, base);
	// 825F80CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F80D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F80D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F80D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F80E0 size=108
    let mut pc: u32 = 0x825F80E0;
    'dispatch: loop {
        match pc {
            0x825F80E0 => {
    //   block [0x825F80E0..0x825F814C)
	// 825F80E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F80E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F80E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F80EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F80F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F80F4: 38EBFA30  addi r7, r11, -0x5d0
	ctx.r[7].s64 = ctx.r[11].s64 + -1488;
	// 825F80F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F80FC: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 825F8100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F810C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8110: 386AEEBC  addi r3, r10, -0x1144
	ctx.r[3].s64 = ctx.r[10].s64 + -4420;
	// 825F8114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F811C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F812C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8138: 4BE6ECE9  bl 0x82466e20
	ctx.lr = 0x825F813C;
	sub_82466E20(ctx, base);
	// 825F813C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8150 size=116
    let mut pc: u32 = 0x825F8150;
    'dispatch: loop {
        match pc {
            0x825F8150 => {
    //   block [0x825F8150..0x825F81C4)
	// 825F8150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F815C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8164: 390BFA94  addi r8, r11, -0x56c
	ctx.r[8].s64 = ctx.r[11].s64 + -1388;
	// 825F8168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F816C: 392A7C6C  addi r9, r10, 0x7c6c
	ctx.r[9].s64 = ctx.r[10].s64 + 31852;
	// 825F8170: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8174: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F8178: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F817C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F818C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8194: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F8198: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 825F819C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F81A0: 386BEEEC  addi r3, r11, -0x1114
	ctx.r[3].s64 = ctx.r[11].s64 + -4372;
	// 825F81A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F81A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F81AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F81B0: 4BE6EC71  bl 0x82466e20
	ctx.lr = 0x825F81B4;
	sub_82466E20(ctx, base);
	// 825F81B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F81B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F81BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F81C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F81C8 size=108
    let mut pc: u32 = 0x825F81C8;
    'dispatch: loop {
        match pc {
            0x825F81C8 => {
    //   block [0x825F81C8..0x825F8234)
	// 825F81C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F81CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F81D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F81D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F81D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F81DC: 38EBFAB0  addi r7, r11, -0x550
	ctx.r[7].s64 = ctx.r[11].s64 + -1360;
	// 825F81E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F81E4: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 825F81E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F81EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F81F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F81F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F81F8: 386AEF1C  addi r3, r10, -0x10e4
	ctx.r[3].s64 = ctx.r[10].s64 + -4324;
	// 825F81FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F820C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F821C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8220: 4BE6EC01  bl 0x82466e20
	ctx.lr = 0x825F8224;
	sub_82466E20(ctx, base);
	// 825F8224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F822C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8238 size=108
    let mut pc: u32 = 0x825F8238;
    'dispatch: loop {
        match pc {
            0x825F8238 => {
    //   block [0x825F8238..0x825F82A4)
	// 825F8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F823C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8244: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8248: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F824C: 38EBFAF8  addi r7, r11, -0x508
	ctx.r[7].s64 = ctx.r[11].s64 + -1288;
	// 825F8250: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F8254: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 825F8258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F825C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8260: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8268: 386AEF4C  addi r3, r10, -0x10b4
	ctx.r[3].s64 = ctx.r[10].s64 + -4276;
	// 825F826C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F827C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F828C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8290: 4BE6EB91  bl 0x82466e20
	ctx.lr = 0x825F8294;
	sub_82466E20(ctx, base);
	// 825F8294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F829C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F82A8 size=108
    let mut pc: u32 = 0x825F82A8;
    'dispatch: loop {
        match pc {
            0x825F82A8 => {
    //   block [0x825F82A8..0x825F8314)
	// 825F82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F82AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F82B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F82B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F82B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F82BC: 38EBFB88  addi r7, r11, -0x478
	ctx.r[7].s64 = ctx.r[11].s64 + -1144;
	// 825F82C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F82C4: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 825F82C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F82CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F82D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F82D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F82D8: 386AEF7C  addi r3, r10, -0x1084
	ctx.r[3].s64 = ctx.r[10].s64 + -4228;
	// 825F82DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F82E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F82E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F82E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F82EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F82F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F82F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F82F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F82FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8300: 4BE6EB21  bl 0x82466e20
	ctx.lr = 0x825F8304;
	sub_82466E20(ctx, base);
	// 825F8304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F830C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8318 size=100
    let mut pc: u32 = 0x825F8318;
    'dispatch: loop {
        match pc {
            0x825F8318 => {
    //   block [0x825F8318..0x825F837C)
	// 825F8318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F832C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8338: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 825F833C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F834C: 386AEFAC  addi r3, r10, -0x1054
	ctx.r[3].s64 = ctx.r[10].s64 + -4180;
	// 825F8350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8358: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F835C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F8364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8368: 4BE6EAB9  bl 0x82466e20
	ctx.lr = 0x825F836C;
	sub_82466E20(ctx, base);
	// 825F836C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8380 size=112
    let mut pc: u32 = 0x825F8380;
    'dispatch: loop {
        match pc {
            0x825F8380 => {
    //   block [0x825F8380..0x825F83F0)
	// 825F8380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F838C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8390: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8394: 38AAEFAC  addi r5, r10, -0x1054
	ctx.r[5].s64 = ctx.r[10].s64 + -4180;
	// 825F8398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F839C: 390BFC18  addi r8, r11, -0x3e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1000;
	// 825F83A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F83A4: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 825F83A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F83AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F83B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F83B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F83B8: 386AEFDC  addi r3, r10, -0x1024
	ctx.r[3].s64 = ctx.r[10].s64 + -4132;
	// 825F83BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F83C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F83C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F83C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F83CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F83D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F83D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F83D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F83DC: 4BE6EA45  bl 0x82466e20
	ctx.lr = 0x825F83E0;
	sub_82466E20(ctx, base);
	// 825F83E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F83E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F83E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F83EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F83F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F83F0 size=108
    let mut pc: u32 = 0x825F83F0;
    'dispatch: loop {
        match pc {
            0x825F83F0 => {
    //   block [0x825F83F0..0x825F845C)
	// 825F83F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F83F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F83F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F83FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8404: 38EBFC78  addi r7, r11, -0x388
	ctx.r[7].s64 = ctx.r[11].s64 + -904;
	// 825F8408: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F840C: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 825F8410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8414: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8420: 386AF00C  addi r3, r10, -0xff4
	ctx.r[3].s64 = ctx.r[10].s64 + -4084;
	// 825F8424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F842C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F843C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8448: 4BE6E9D9  bl 0x82466e20
	ctx.lr = 0x825F844C;
	sub_82466E20(ctx, base);
	// 825F844C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8460 size=108
    let mut pc: u32 = 0x825F8460;
    'dispatch: loop {
        match pc {
            0x825F8460 => {
    //   block [0x825F8460..0x825F84CC)
	// 825F8460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F846C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8474: 38EBFCA8  addi r7, r11, -0x358
	ctx.r[7].s64 = ctx.r[11].s64 + -856;
	// 825F8478: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F847C: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 825F8480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F848C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8490: 386AF03C  addi r3, r10, -0xfc4
	ctx.r[3].s64 = ctx.r[10].s64 + -4036;
	// 825F8494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F849C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F84A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F84A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F84A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F84AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F84B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F84B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F84B8: 4BE6E969  bl 0x82466e20
	ctx.lr = 0x825F84BC;
	sub_82466E20(ctx, base);
	// 825F84BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F84C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F84C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F84C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F84D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F84D0 size=108
    let mut pc: u32 = 0x825F84D0;
    'dispatch: loop {
        match pc {
            0x825F84D0 => {
    //   block [0x825F84D0..0x825F853C)
	// 825F84D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F84D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F84D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F84DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F84E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F84E4: 38EBFD08  addi r7, r11, -0x2f8
	ctx.r[7].s64 = ctx.r[11].s64 + -760;
	// 825F84E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F84EC: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 825F84F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F84F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F84F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F84FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8500: 386AF06C  addi r3, r10, -0xf94
	ctx.r[3].s64 = ctx.r[10].s64 + -3988;
	// 825F8504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F850C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F851C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8528: 4BE6E8F9  bl 0x82466e20
	ctx.lr = 0x825F852C;
	sub_82466E20(ctx, base);
	// 825F852C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8540 size=112
    let mut pc: u32 = 0x825F8540;
    'dispatch: loop {
        match pc {
            0x825F8540 => {
    //   block [0x825F8540..0x825F85B0)
	// 825F8540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F854C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F8550: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825F8554: 38EAFD68  addi r7, r10, -0x298
	ctx.r[7].s64 = ctx.r[10].s64 + -664;
	// 825F8558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F855C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8560: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 825F8564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8568: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F856C: 396B7C80  addi r11, r11, 0x7c80
	ctx.r[11].s64 = ctx.r[11].s64 + 31872;
	// 825F8570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F8574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8578: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F857C: 386AF09C  addi r3, r10, -0xf64
	ctx.r[3].s64 = ctx.r[10].s64 + -3940;
	// 825F8580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8584: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F8588: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F858C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F8590: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8594: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8598: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F859C: 4BE6E885  bl 0x82466e20
	ctx.lr = 0x825F85A0;
	sub_82466E20(ctx, base);
	// 825F85A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F85A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F85A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F85AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F85B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F85B0 size=96
    let mut pc: u32 = 0x825F85B0;
    'dispatch: loop {
        match pc {
            0x825F85B0 => {
    //   block [0x825F85B0..0x825F8610)
	// 825F85B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F85B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F85B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F85BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F85C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F85C4: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 825F85C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F85CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F85D0: 386AF0CC  addi r3, r10, -0xf34
	ctx.r[3].s64 = ctx.r[10].s64 + -3892;
	// 825F85D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F85D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F85DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F85E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F85E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F85E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F85EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F85F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F85F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F85F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F85FC: 4BE6E825  bl 0x82466e20
	ctx.lr = 0x825F8600;
	sub_82466E20(ctx, base);
	// 825F8600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8610 size=112
    let mut pc: u32 = 0x825F8610;
    'dispatch: loop {
        match pc {
            0x825F8610 => {
    //   block [0x825F8610..0x825F8680)
	// 825F8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F861C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8620: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8624: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F862C: 390BFE88  addi r8, r11, -0x178
	ctx.r[8].s64 = ctx.r[11].s64 + -376;
	// 825F8630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F8634: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 825F8638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F863C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8648: 386AF0FC  addi r3, r10, -0xf04
	ctx.r[3].s64 = ctx.r[10].s64 + -3844;
	// 825F864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F866C: 4BE6E7B5  bl 0x82466e20
	ctx.lr = 0x825F8670;
	sub_82466E20(ctx, base);
	// 825F8670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8680 size=24
    let mut pc: u32 = 0x825F8680;
    'dispatch: loop {
        match pc {
            0x825F8680 => {
    //   block [0x825F8680..0x825F8698)
	// 825F8680: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8684: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8688: 394A8C28  addi r10, r10, -0x73d8
	ctx.r[10].s64 = ctx.r[10].s64 + -29656;
	// 825F868C: 816BFAAC  lwz r11, -0x554(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1364 as u32) ) } as u64;
	// 825F8690: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825F8694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8698 size=116
    let mut pc: u32 = 0x825F8698;
    'dispatch: loop {
        match pc {
            0x825F8698 => {
    //   block [0x825F8698..0x825F870C)
	// 825F8698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F869C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F86A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F86A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F86A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F86AC: 390B8C28  addi r8, r11, -0x73d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29656;
	// 825F86B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F86B4: 392A7CFC  addi r9, r10, 0x7cfc
	ctx.r[9].s64 = ctx.r[10].s64 + 31996;
	// 825F86B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F86BC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825F86C0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F86C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F86C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F86CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F86D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F86D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F86D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F86DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F86E0: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 825F86E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F86E8: 386BF12C  addi r3, r11, -0xed4
	ctx.r[3].s64 = ctx.r[11].s64 + -3796;
	// 825F86EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F86F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F86F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F86F8: 4BE6E729  bl 0x82466e20
	ctx.lr = 0x825F86FC;
	sub_82466E20(ctx, base);
	// 825F86FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F8710 size=24
    let mut pc: u32 = 0x825F8710;
    'dispatch: loop {
        match pc {
            0x825F8710 => {
    //   block [0x825F8710..0x825F8728)
	// 825F8710: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8714: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825F8718: 394A8D00  addi r10, r10, -0x7300
	ctx.r[10].s64 = ctx.r[10].s64 + -29440;
	// 825F871C: 816BFEF0  lwz r11, -0x110(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-272 as u32) ) } as u64;
	// 825F8720: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F8724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8728 size=116
    let mut pc: u32 = 0x825F8728;
    'dispatch: loop {
        match pc {
            0x825F8728 => {
    //   block [0x825F8728..0x825F879C)
	// 825F8728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8734: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825F8738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F873C: 390B8D00  addi r8, r11, -0x7300
	ctx.r[8].s64 = ctx.r[11].s64 + -29440;
	// 825F8740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8744: 392A7DB0  addi r9, r10, 0x7db0
	ctx.r[9].s64 = ctx.r[10].s64 + 32176;
	// 825F8748: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F874C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825F8750: 38AAF39C  addi r5, r10, -0xc64
	ctx.r[5].s64 = ctx.r[10].s64 + -3172;
	// 825F8754: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F875C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F8760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F876C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F8770: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 825F8774: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F8778: 386BF15C  addi r3, r11, -0xea4
	ctx.r[3].s64 = ctx.r[11].s64 + -3748;
	// 825F877C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F8780: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8788: 4BE6E699  bl 0x82466e20
	ctx.lr = 0x825F878C;
	sub_82466E20(ctx, base);
	// 825F878C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F87A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F87A0 size=112
    let mut pc: u32 = 0x825F87A0;
    'dispatch: loop {
        match pc {
            0x825F87A0 => {
    //   block [0x825F87A0..0x825F8810)
	// 825F87A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F87A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F87A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F87AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F87B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F87B4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F87B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F87BC: 390BFEF8  addi r8, r11, -0x108
	ctx.r[8].s64 = ctx.r[11].s64 + -264;
	// 825F87C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F87C4: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 825F87C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F87CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F87D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F87D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F87D8: 386AF18C  addi r3, r10, -0xe74
	ctx.r[3].s64 = ctx.r[10].s64 + -3700;
	// 825F87DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F87E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F87E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F87E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F87EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F87F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F87F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F87F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F87FC: 4BE6E625  bl 0x82466e20
	ctx.lr = 0x825F8800;
	sub_82466E20(ctx, base);
	// 825F8800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F880C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8810 size=112
    let mut pc: u32 = 0x825F8810;
    'dispatch: loop {
        match pc {
            0x825F8810 => {
    //   block [0x825F8810..0x825F8880)
	// 825F8810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F881C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8820: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8824: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F882C: 390BFF40  addi r8, r11, -0xc0
	ctx.r[8].s64 = ctx.r[11].s64 + -192;
	// 825F8830: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F8834: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 825F8838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F883C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8848: 386AF1BC  addi r3, r10, -0xe44
	ctx.r[3].s64 = ctx.r[10].s64 + -3652;
	// 825F884C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F885C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F886C: 4BE6E5B5  bl 0x82466e20
	ctx.lr = 0x825F8870;
	sub_82466E20(ctx, base);
	// 825F8870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F887C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8880 size=108
    let mut pc: u32 = 0x825F8880;
    'dispatch: loop {
        match pc {
            0x825F8880 => {
    //   block [0x825F8880..0x825F88EC)
	// 825F8880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F888C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8894: 38EBFF88  addi r7, r11, -0x78
	ctx.r[7].s64 = ctx.r[11].s64 + -120;
	// 825F8898: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F889C: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 825F88A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F88A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F88A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F88AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F88B0: 386AF1EC  addi r3, r10, -0xe14
	ctx.r[3].s64 = ctx.r[10].s64 + -3604;
	// 825F88B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F88B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F88BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F88C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F88C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F88C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F88CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F88D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F88D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F88D8: 4BE6E549  bl 0x82466e20
	ctx.lr = 0x825F88DC;
	sub_82466E20(ctx, base);
	// 825F88DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F88E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F88E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F88E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F88F0 size=108
    let mut pc: u32 = 0x825F88F0;
    'dispatch: loop {
        match pc {
            0x825F88F0 => {
    //   block [0x825F88F0..0x825F895C)
	// 825F88F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F88F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F88F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F88FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8904: 38EBFFE8  addi r7, r11, -0x18
	ctx.r[7].s64 = ctx.r[11].s64 + -24;
	// 825F8908: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F890C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 825F8910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8914: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8918: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F891C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8920: 386AF21C  addi r3, r10, -0xde4
	ctx.r[3].s64 = ctx.r[10].s64 + -3556;
	// 825F8924: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F8928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F892C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8944: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F8948: 4BE6E4D9  bl 0x82466e20
	ctx.lr = 0x825F894C;
	sub_82466E20(ctx, base);
	// 825F894C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8960 size=112
    let mut pc: u32 = 0x825F8960;
    'dispatch: loop {
        match pc {
            0x825F8960 => {
    //   block [0x825F8960..0x825F89D0)
	// 825F8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F896C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F8970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8974: 392B7DE4  addi r9, r11, 0x7de4
	ctx.r[9].s64 = ctx.r[11].s64 + 32228;
	// 825F8978: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825F897C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F8980: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8984: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 825F8988: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F898C: 396B0090  addi r11, r11, 0x90
	ctx.r[11].s64 = ctx.r[11].s64 + 144;
	// 825F8990: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F8994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8998: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F89A0: 386AF24C  addi r3, r10, -0xdb4
	ctx.r[3].s64 = ctx.r[10].s64 + -3508;
	// 825F89A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F89A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F89AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F89B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F89B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F89B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F89BC: 4BE6E465  bl 0x82466e20
	ctx.lr = 0x825F89C0;
	sub_82466E20(ctx, base);
	// 825F89C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F89C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F89C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F89CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F89D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F89D0 size=116
    let mut pc: u32 = 0x825F89D0;
    'dispatch: loop {
        match pc {
            0x825F89D0 => {
    //   block [0x825F89D0..0x825F8A44)
	// 825F89D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F89D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F89D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F89DC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F89E0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F89E4: 390A01E0  addi r8, r10, 0x1e0
	ctx.r[8].s64 = ctx.r[10].s64 + 480;
	// 825F89E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F89EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F89F0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F89F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825F89F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F89FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8A04: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 825F8A08: 396B7E38  addi r11, r11, 0x7e38
	ctx.r[11].s64 = ctx.r[11].s64 + 32312;
	// 825F8A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8A10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8A14: 386AF27C  addi r3, r10, -0xd84
	ctx.r[3].s64 = ctx.r[10].s64 + -3460;
	// 825F8A18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F8A1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8A20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F8A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8A30: 4BE6E3F1  bl 0x82466e20
	ctx.lr = 0x825F8A34;
	sub_82466E20(ctx, base);
	// 825F8A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8A48 size=112
    let mut pc: u32 = 0x825F8A48;
    'dispatch: loop {
        match pc {
            0x825F8A48 => {
    //   block [0x825F8A48..0x825F8AB8)
	// 825F8A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8A54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8A58: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8A5C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8A64: 390B0270  addi r8, r11, 0x270
	ctx.r[8].s64 = ctx.r[11].s64 + 624;
	// 825F8A68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F8A6C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 825F8A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8A80: 386AF2AC  addi r3, r10, -0xd54
	ctx.r[3].s64 = ctx.r[10].s64 + -3412;
	// 825F8A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8AA4: 4BE6E37D  bl 0x82466e20
	ctx.lr = 0x825F8AA8;
	sub_82466E20(ctx, base);
	// 825F8AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F8AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F8AB8 size=112
    let mut pc: u32 = 0x825F8AB8;
    'dispatch: loop {
        match pc {
            0x825F8AB8 => {
    //   block [0x825F8AB8..0x825F8B28)
	// 825F8AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F8ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F8AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F8AC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8AC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F8ACC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825F8AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F8AD4: 390B0318  addi r8, r11, 0x318
	ctx.r[8].s64 = ctx.r[11].s64 + 792;
	// 825F8AD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F8ADC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 825F8AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F8AE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F8AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F8AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F8AF0: 386AF2DC  addi r3, r10, -0xd24
	ctx.r[3].s64 = ctx.r[10].s64 + -3364;
	// 825F8AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F8AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F8AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F8B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F8B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F8B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F8B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F8B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F8B14: 4BE6E30D  bl 0x82466e20
	ctx.lr = 0x825F8B18;
	sub_82466E20(ctx, base);
	// 825F8B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F8B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F8B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F8B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


