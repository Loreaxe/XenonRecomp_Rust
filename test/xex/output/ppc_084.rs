pub fn sub_82625858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625858 size=116
    let mut pc: u32 = 0x82625858;
    'dispatch: loop {
        match pc {
            0x82625858 => {
    //   block [0x82625858..0x826258CC)
	// 82625858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262585C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625864: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625868: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262586C: 390A5490  addi r8, r10, 0x5490
	ctx.r[8].s64 = ctx.r[10].s64 + 21648;
	// 82625870: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625878: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 8262587C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625880: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262588C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82625890: 396B2DC8  addi r11, r11, 0x2dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 11720;
	// 82625894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625898: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262589C: 386A205C  addi r3, r10, 0x205c
	ctx.r[3].s64 = ctx.r[10].s64 + 8284;
	// 826258A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826258A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826258A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826258AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826258B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826258B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826258B8: 4BE41569  bl 0x82466e20
	ctx.lr = 0x826258BC;
	sub_82466E20(ctx, base);
	// 826258BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826258C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826258C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826258C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826258D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826258D0 size=108
    let mut pc: u32 = 0x826258D0;
    'dispatch: loop {
        match pc {
            0x826258D0 => {
    //   block [0x826258D0..0x8262593C)
	// 826258D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826258D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826258D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826258DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826258E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826258E4: 38EB5520  addi r7, r11, 0x5520
	ctx.r[7].s64 = ctx.r[11].s64 + 21792;
	// 826258E8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826258EC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826258F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826258F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826258F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826258FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625900: 386A208C  addi r3, r10, 0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + 8332;
	// 82625904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262590C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262591C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625928: 4BE414F9  bl 0x82466e20
	ctx.lr = 0x8262592C;
	sub_82466E20(ctx, base);
	// 8262592C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625940 size=116
    let mut pc: u32 = 0x82625940;
    'dispatch: loop {
        match pc {
            0x82625940 => {
    //   block [0x82625940..0x826259B4)
	// 82625940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262594C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625950: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82625954: 390A5670  addi r8, r10, 0x5670
	ctx.r[8].s64 = ctx.r[10].s64 + 22128;
	// 82625958: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262595C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625960: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625964: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625968: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262596C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625974: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82625978: 396B2DEC  addi r11, r11, 0x2dec
	ctx.r[11].s64 = ctx.r[11].s64 + 11756;
	// 8262597C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625980: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625984: 386A20BC  addi r3, r10, 0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8380;
	// 82625988: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262598C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625990: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262599C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826259A0: 4BE41481  bl 0x82466e20
	ctx.lr = 0x826259A4;
	sub_82466E20(ctx, base);
	// 826259A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826259A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826259AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826259B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826259B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826259B8 size=112
    let mut pc: u32 = 0x826259B8;
    'dispatch: loop {
        match pc {
            0x826259B8 => {
    //   block [0x826259B8..0x82625A28)
	// 826259B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826259BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826259C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826259C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826259C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826259CC: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 826259D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826259D4: 390B5730  addi r8, r11, 0x5730
	ctx.r[8].s64 = ctx.r[11].s64 + 22320;
	// 826259D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826259DC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826259E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826259E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826259E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826259EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826259F0: 386A20EC  addi r3, r10, 0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8428;
	// 826259F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826259F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826259FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625A14: 4BE4140D  bl 0x82466e20
	ctx.lr = 0x82625A18;
	sub_82466E20(ctx, base);
	// 82625A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625A28 size=112
    let mut pc: u32 = 0x82625A28;
    'dispatch: loop {
        match pc {
            0x82625A28 => {
    //   block [0x82625A28..0x82625A98)
	// 82625A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625A34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625A38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625A3C: 38AA1EDC  addi r5, r10, 0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + 7900;
	// 82625A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625A44: 390B5748  addi r8, r11, 0x5748
	ctx.r[8].s64 = ctx.r[11].s64 + 22344;
	// 82625A48: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82625A4C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82625A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625A54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625A58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625A60: 386A211C  addi r3, r10, 0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + 8476;
	// 82625A64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625A84: 4BE4139D  bl 0x82466e20
	ctx.lr = 0x82625A88;
	sub_82466E20(ctx, base);
	// 82625A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625A98 size=112
    let mut pc: u32 = 0x82625A98;
    'dispatch: loop {
        match pc {
            0x82625A98 => {
    //   block [0x82625A98..0x82625B08)
	// 82625A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625AA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625AAC: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625AB4: 390B57D8  addi r8, r11, 0x57d8
	ctx.r[8].s64 = ctx.r[11].s64 + 22488;
	// 82625AB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82625ABC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 82625AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625AD0: 386A214C  addi r3, r10, 0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + 8524;
	// 82625AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625AF4: 4BE4132D  bl 0x82466e20
	ctx.lr = 0x82625AF8;
	sub_82466E20(ctx, base);
	// 82625AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625B08 size=116
    let mut pc: u32 = 0x82625B08;
    'dispatch: loop {
        match pc {
            0x82625B08 => {
    //   block [0x82625B08..0x82625B7C)
	// 82625B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625B14: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625B18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82625B1C: 390B57F4  addi r8, r11, 0x57f4
	ctx.r[8].s64 = ctx.r[11].s64 + 22516;
	// 82625B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625B24: 392A2E34  addi r9, r10, 0x2e34
	ctx.r[9].s64 = ctx.r[10].s64 + 11828;
	// 82625B28: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625B2C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82625B30: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82625B34: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625B3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625B4C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82625B50: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82625B54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625B58: 386B217C  addi r3, r11, 0x217c
	ctx.r[3].s64 = ctx.r[11].s64 + 8572;
	// 82625B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82625B60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625B68: 4BE412B9  bl 0x82466e20
	ctx.lr = 0x82625B6C;
	sub_82466E20(ctx, base);
	// 82625B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625B80 size=100
    let mut pc: u32 = 0x82625B80;
    'dispatch: loop {
        match pc {
            0x82625B80 => {
    //   block [0x82625B80..0x82625BE4)
	// 82625B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625B8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625B94: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82625B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625BA0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82625BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625BB4: 386A21AC  addi r3, r10, 0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8620;
	// 82625BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625BC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82625BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625BC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625BD0: 4BE41251  bl 0x82466e20
	ctx.lr = 0x82625BD4;
	sub_82466E20(ctx, base);
	// 82625BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625BE8 size=112
    let mut pc: u32 = 0x82625BE8;
    'dispatch: loop {
        match pc {
            0x82625BE8 => {
    //   block [0x82625BE8..0x82625C58)
	// 82625BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625BF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625BFC: 38AA21AC  addi r5, r10, 0x21ac
	ctx.r[5].s64 = ctx.r[10].s64 + 8620;
	// 82625C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625C04: 390B580C  addi r8, r11, 0x580c
	ctx.r[8].s64 = ctx.r[11].s64 + 22540;
	// 82625C08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82625C0C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82625C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625C14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625C20: 386A21DC  addi r3, r10, 0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + 8668;
	// 82625C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625C44: 4BE411DD  bl 0x82466e20
	ctx.lr = 0x82625C48;
	sub_82466E20(ctx, base);
	// 82625C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625C58 size=108
    let mut pc: u32 = 0x82625C58;
    'dispatch: loop {
        match pc {
            0x82625C58 => {
    //   block [0x82625C58..0x82625CC4)
	// 82625C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625C64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82625C6C: 38EB5828  addi r7, r11, 0x5828
	ctx.r[7].s64 = ctx.r[11].s64 + 22568;
	// 82625C70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82625C74: 388A446C  addi r4, r10, 0x446c
	ctx.r[4].s64 = ctx.r[10].s64 + 17516;
	// 82625C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625C88: 386A220C  addi r3, r10, 0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + 8716;
	// 82625C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625CB0: 4BE41171  bl 0x82466e20
	ctx.lr = 0x82625CB4;
	sub_82466E20(ctx, base);
	// 82625CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625CC8 size=112
    let mut pc: u32 = 0x82625CC8;
    'dispatch: loop {
        match pc {
            0x82625CC8 => {
    //   block [0x82625CC8..0x82625D38)
	// 82625CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625CD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625CDC: 392B2EA0  addi r9, r11, 0x2ea0
	ctx.r[9].s64 = ctx.r[11].s64 + 11936;
	// 82625CE0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82625CE4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82625CE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625CEC: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82625CF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625CF4: 396B5888  addi r11, r11, 0x5888
	ctx.r[11].s64 = ctx.r[11].s64 + 22664;
	// 82625CF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82625CFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625D00: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82625D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625D08: 386A223C  addi r3, r10, 0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + 8764;
	// 82625D0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82625D10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82625D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625D18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82625D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625D24: 4BE410FD  bl 0x82466e20
	ctx.lr = 0x82625D28;
	sub_82466E20(ctx, base);
	// 82625D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625D38 size=108
    let mut pc: u32 = 0x82625D38;
    'dispatch: loop {
        match pc {
            0x82625D38 => {
    //   block [0x82625D38..0x82625DA4)
	// 82625D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625D44: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625D4C: 38EB5948  addi r7, r11, 0x5948
	ctx.r[7].s64 = ctx.r[11].s64 + 22856;
	// 82625D50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82625D54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82625D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625D68: 386A226C  addi r3, r10, 0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + 8812;
	// 82625D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625D90: 4BE41091  bl 0x82466e20
	ctx.lr = 0x82625D94;
	sub_82466E20(ctx, base);
	// 82625D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625DA8 size=116
    let mut pc: u32 = 0x82625DA8;
    'dispatch: loop {
        match pc {
            0x82625DA8 => {
    //   block [0x82625DA8..0x82625E1C)
	// 82625DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625DB4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625DB8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82625DBC: 390A5990  addi r8, r10, 0x5990
	ctx.r[8].s64 = ctx.r[10].s64 + 22928;
	// 82625DC0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625DC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625DC8: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82625DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625DD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625DDC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82625DE0: 396B2ED8  addi r11, r11, 0x2ed8
	ctx.r[11].s64 = ctx.r[11].s64 + 11992;
	// 82625DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625DEC: 386A229C  addi r3, r10, 0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + 8860;
	// 82625DF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82625DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625DF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625E08: 4BE41019  bl 0x82466e20
	ctx.lr = 0x82625E0C;
	sub_82466E20(ctx, base);
	// 82625E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625E20 size=100
    let mut pc: u32 = 0x82625E20;
    'dispatch: loop {
        match pc {
            0x82625E20 => {
    //   block [0x82625E20..0x82625E84)
	// 82625E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625E2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625E34: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82625E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625E40: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82625E44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625E54: 386A22CC  addi r3, r10, 0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8908;
	// 82625E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625E60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82625E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625E70: 4BE40FB1  bl 0x82466e20
	ctx.lr = 0x82625E74;
	sub_82466E20(ctx, base);
	// 82625E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82625E88 size=24
    let mut pc: u32 = 0x82625E88;
    'dispatch: loop {
        match pc {
            0x82625E88 => {
    //   block [0x82625E88..0x82625EA0)
	// 82625E88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625E8C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82625E90: 394AAD38  addi r10, r10, -0x52c8
	ctx.r[10].s64 = ctx.r[10].s64 + -21192;
	// 82625E94: 816B5B14  lwz r11, 0x5b14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23316 as u32) ) } as u64;
	// 82625E98: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82625E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625EA0 size=116
    let mut pc: u32 = 0x82625EA0;
    'dispatch: loop {
        match pc {
            0x82625EA0 => {
    //   block [0x82625EA0..0x82625F14)
	// 82625EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625EAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625EB0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625EB4: 392B2F58  addi r9, r11, 0x2f58
	ctx.r[9].s64 = ctx.r[11].s64 + 12120;
	// 82625EB8: 38AA22CC  addi r5, r10, 0x22cc
	ctx.r[5].s64 = ctx.r[10].s64 + 8908;
	// 82625EBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625EC0: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 82625EC4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82625EC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82625ECC: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82625ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625ED4: 396BAD38  addi r11, r11, -0x52c8
	ctx.r[11].s64 = ctx.r[11].s64 + -21192;
	// 82625ED8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82625EDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625EE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82625EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625EE8: 386A22FC  addi r3, r10, 0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + 8956;
	// 82625EEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82625EF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82625EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625EF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82625EFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625F00: 4BE40F21  bl 0x82466e20
	ctx.lr = 0x82625F04;
	sub_82466E20(ctx, base);
	// 82625F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625F18 size=112
    let mut pc: u32 = 0x82625F18;
    'dispatch: loop {
        match pc {
            0x82625F18 => {
    //   block [0x82625F18..0x82625F88)
	// 82625F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625F24: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625F28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82625F2C: 38EA5B20  addi r7, r10, 0x5b20
	ctx.r[7].s64 = ctx.r[10].s64 + 23328;
	// 82625F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625F34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625F38: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82625F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625F40: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625F44: 396B2FE4  addi r11, r11, 0x2fe4
	ctx.r[11].s64 = ctx.r[11].s64 + 12260;
	// 82625F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625F4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625F50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625F54: 386A232C  addi r3, r10, 0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + 9004;
	// 82625F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625F5C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82625F60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625F64: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625F68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625F6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625F70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625F74: 4BE40EAD  bl 0x82466e20
	ctx.lr = 0x82625F78;
	sub_82466E20(ctx, base);
	// 82625F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625F88 size=108
    let mut pc: u32 = 0x82625F88;
    'dispatch: loop {
        match pc {
            0x82625F88 => {
    //   block [0x82625F88..0x82625FF4)
	// 82625F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625F94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625F9C: 38EB5B80  addi r7, r11, 0x5b80
	ctx.r[7].s64 = ctx.r[11].s64 + 23424;
	// 82625FA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82625FA4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82625FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625FAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625FB8: 386A235C  addi r3, r10, 0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + 9052;
	// 82625FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625FE0: 4BE40E41  bl 0x82466e20
	ctx.lr = 0x82625FE4;
	sub_82466E20(ctx, base);
	// 82625FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625FF8 size=116
    let mut pc: u32 = 0x82625FF8;
    'dispatch: loop {
        match pc {
            0x82625FF8 => {
    //   block [0x82625FF8..0x8262606C)
	// 82625FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626004: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262600C: 390B5BB0  addi r8, r11, 0x5bb0
	ctx.r[8].s64 = ctx.r[11].s64 + 23472;
	// 82626010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626014: 392A2FD0  addi r9, r10, 0x2fd0
	ctx.r[9].s64 = ctx.r[10].s64 + 12240;
	// 82626018: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262601C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82626020: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82626024: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262602C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262603C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82626040: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82626044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626048: 386B238C  addi r3, r11, 0x238c
	ctx.r[3].s64 = ctx.r[11].s64 + 9100;
	// 8262604C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626050: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626058: 4BE40DC9  bl 0x82466e20
	ctx.lr = 0x8262605C;
	sub_82466E20(ctx, base);
	// 8262605C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626070 size=96
    let mut pc: u32 = 0x82626070;
    'dispatch: loop {
        match pc {
            0x82626070 => {
    //   block [0x82626070..0x826260D0)
	// 82626070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262607C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626084: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82626088: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262608C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626090: 386A23BC  addi r3, r10, 0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9148;
	// 82626094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262609C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826260A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826260A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826260A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826260AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826260B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826260B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826260B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826260BC: 4BE40D65  bl 0x82466e20
	ctx.lr = 0x826260C0;
	sub_82466E20(ctx, base);
	// 826260C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826260C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826260C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826260CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826260D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826260D0 size=112
    let mut pc: u32 = 0x826260D0;
    'dispatch: loop {
        match pc {
            0x826260D0 => {
    //   block [0x826260D0..0x82626140)
	// 826260D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826260D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826260D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826260DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826260E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826260E4: 38AA23BC  addi r5, r10, 0x23bc
	ctx.r[5].s64 = ctx.r[10].s64 + 9148;
	// 826260E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826260EC: 390B5BC8  addi r8, r11, 0x5bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 23496;
	// 826260F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826260F4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826260F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826260FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626108: 386A23EC  addi r3, r10, 0x23ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9196;
	// 8262610C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82626110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262611C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262612C: 4BE40CF5  bl 0x82466e20
	ctx.lr = 0x82626130;
	sub_82466E20(ctx, base);
	// 82626130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262613C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626140 size=112
    let mut pc: u32 = 0x82626140;
    'dispatch: loop {
        match pc {
            0x82626140 => {
    //   block [0x82626140..0x826261B0)
	// 82626140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262614C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626150: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626154: 392A3000  addi r9, r10, 0x3000
	ctx.r[9].s64 = ctx.r[10].s64 + 12288;
	// 82626158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262615C: 390B5BF8  addi r8, r11, 0x5bf8
	ctx.r[8].s64 = ctx.r[11].s64 + 23544;
	// 82626160: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82626164: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82626168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262616C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626178: 386A241C  addi r3, r10, 0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + 9244;
	// 8262617C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626180: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262618C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262619C: 4BE40C85  bl 0x82466e20
	ctx.lr = 0x826261A0;
	sub_82466E20(ctx, base);
	// 826261A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826261A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826261A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826261AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826261B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826261B0 size=108
    let mut pc: u32 = 0x826261B0;
    'dispatch: loop {
        match pc {
            0x826261B0 => {
    //   block [0x826261B0..0x8262621C)
	// 826261B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826261B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826261B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826261BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826261C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826261C4: 38EB5CA0  addi r7, r11, 0x5ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 23712;
	// 826261C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826261CC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826261D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826261D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826261D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826261DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826261E0: 386A244C  addi r3, r10, 0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + 9292;
	// 826261E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826261E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826261EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826261F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826261F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826261F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826261FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626208: 4BE40C19  bl 0x82466e20
	ctx.lr = 0x8262620C;
	sub_82466E20(ctx, base);
	// 8262620C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626220 size=108
    let mut pc: u32 = 0x82626220;
    'dispatch: loop {
        match pc {
            0x82626220 => {
    //   block [0x82626220..0x8262628C)
	// 82626220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262622C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626234: 38EB5CD0  addi r7, r11, 0x5cd0
	ctx.r[7].s64 = ctx.r[11].s64 + 23760;
	// 82626238: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262623C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82626240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262624C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626250: 386A247C  addi r3, r10, 0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + 9340;
	// 82626254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262625C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262626C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626278: 4BE40BA9  bl 0x82466e20
	ctx.lr = 0x8262627C;
	sub_82466E20(ctx, base);
	// 8262627C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626290 size=28
    let mut pc: u32 = 0x82626290;
    'dispatch: loop {
        match pc {
            0x82626290 => {
    //   block [0x82626290..0x826262AC)
	// 82626290: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626294: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626298: 394AAE58  addi r10, r10, -0x51a8
	ctx.r[10].s64 = ctx.r[10].s64 + -20904;
	// 8262629C: 816B5D00  lwz r11, 0x5d00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23808 as u32) ) } as u64;
	// 826262A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826262A4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826262A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826262B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826262B0 size=112
    let mut pc: u32 = 0x826262B0;
    'dispatch: loop {
        match pc {
            0x826262B0 => {
    //   block [0x826262B0..0x82626320)
	// 826262B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826262B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826262B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826262BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826262C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826262C4: 392A3180  addi r9, r10, 0x3180
	ctx.r[9].s64 = ctx.r[10].s64 + 12672;
	// 826262C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826262CC: 390BAE58  addi r8, r11, -0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + -20904;
	// 826262D0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826262D4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826262D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826262DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826262E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826262E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826262E8: 386A24AC  addi r3, r10, 0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9388;
	// 826262EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826262F0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826262F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826262F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826262FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262630C: 4BE40B15  bl 0x82466e20
	ctx.lr = 0x82626310;
	sub_82466E20(ctx, base);
	// 82626310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626320 size=108
    let mut pc: u32 = 0x82626320;
    'dispatch: loop {
        match pc {
            0x82626320 => {
    //   block [0x82626320..0x8262638C)
	// 82626320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262632C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626330: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626334: 38EB5D0C  addi r7, r11, 0x5d0c
	ctx.r[7].s64 = ctx.r[11].s64 + 23820;
	// 82626338: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262633C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82626340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626350: 386A24DC  addi r3, r10, 0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9436;
	// 82626354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262635C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262636C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626378: 4BE40AA9  bl 0x82466e20
	ctx.lr = 0x8262637C;
	sub_82466E20(ctx, base);
	// 8262637C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626390 size=108
    let mut pc: u32 = 0x82626390;
    'dispatch: loop {
        match pc {
            0x82626390 => {
    //   block [0x82626390..0x826263FC)
	// 82626390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262639C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826263A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826263A4: 38EB5D24  addi r7, r11, 0x5d24
	ctx.r[7].s64 = ctx.r[11].s64 + 23844;
	// 826263A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826263AC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826263B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826263B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826263B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826263BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826263C0: 386A250C  addi r3, r10, 0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + 9484;
	// 826263C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826263C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826263CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826263D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826263D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826263D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826263DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826263E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826263E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826263E8: 4BE40A39  bl 0x82466e20
	ctx.lr = 0x826263EC;
	sub_82466E20(ctx, base);
	// 826263EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826263F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826263F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826263F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626400 size=108
    let mut pc: u32 = 0x82626400;
    'dispatch: loop {
        match pc {
            0x82626400 => {
    //   block [0x82626400..0x8262646C)
	// 82626400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262640C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626410: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626414: 38EB5D54  addi r7, r11, 0x5d54
	ctx.r[7].s64 = ctx.r[11].s64 + 23892;
	// 82626418: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262641C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82626420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262642C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626430: 386A253C  addi r3, r10, 0x253c
	ctx.r[3].s64 = ctx.r[10].s64 + 9532;
	// 82626434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262643C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262644C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626458: 4BE409C9  bl 0x82466e20
	ctx.lr = 0x8262645C;
	sub_82466E20(ctx, base);
	// 8262645C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626470 size=24
    let mut pc: u32 = 0x82626470;
    'dispatch: loop {
        match pc {
            0x82626470 => {
    //   block [0x82626470..0x82626488)
	// 82626470: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626474: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626478: 394AAF18  addi r10, r10, -0x50e8
	ctx.r[10].s64 = ctx.r[10].s64 + -20712;
	// 8262647C: 816B5D6C  lwz r11, 0x5d6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23916 as u32) ) } as u64;
	// 82626480: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82626484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626488 size=112
    let mut pc: u32 = 0x82626488;
    'dispatch: loop {
        match pc {
            0x82626488 => {
    //   block [0x82626488..0x826264F8)
	// 82626488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626494: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626498: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262649C: 392A31D4  addi r9, r10, 0x31d4
	ctx.r[9].s64 = ctx.r[10].s64 + 12756;
	// 826264A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826264A4: 390BAF18  addi r8, r11, -0x50e8
	ctx.r[8].s64 = ctx.r[11].s64 + -20712;
	// 826264A8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826264AC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826264B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826264B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826264B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826264BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826264C0: 386A256C  addi r3, r10, 0x256c
	ctx.r[3].s64 = ctx.r[10].s64 + 9580;
	// 826264C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826264C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826264CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826264D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826264D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826264D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826264DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826264E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826264E4: 4BE4093D  bl 0x82466e20
	ctx.lr = 0x826264E8;
	sub_82466E20(ctx, base);
	// 826264E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826264EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826264F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826264F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826264F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826264F8 size=112
    let mut pc: u32 = 0x826264F8;
    'dispatch: loop {
        match pc {
            0x826264F8 => {
    //   block [0x826264F8..0x82626568)
	// 826264F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826264FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626504: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626508: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262650C: 392A3210  addi r9, r10, 0x3210
	ctx.r[9].s64 = ctx.r[10].s64 + 12816;
	// 82626510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626514: 390B5D78  addi r8, r11, 0x5d78
	ctx.r[8].s64 = ctx.r[11].s64 + 23928;
	// 82626518: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262651C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82626520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262652C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626530: 386A259C  addi r3, r10, 0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + 9628;
	// 82626534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626538: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8262653C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262654C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626554: 4BE408CD  bl 0x82466e20
	ctx.lr = 0x82626558;
	sub_82466E20(ctx, base);
	// 82626558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262655C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626568 size=108
    let mut pc: u32 = 0x82626568;
    'dispatch: loop {
        match pc {
            0x82626568 => {
    //   block [0x82626568..0x826265D4)
	// 82626568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262656C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626574: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262657C: 38EB5DC0  addi r7, r11, 0x5dc0
	ctx.r[7].s64 = ctx.r[11].s64 + 24000;
	// 82626580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626584: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82626588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262658C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626598: 386A25CC  addi r3, r10, 0x25cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9676;
	// 8262659C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826265A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826265A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826265A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826265AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826265B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826265B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826265B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826265BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826265C0: 4BE40861  bl 0x82466e20
	ctx.lr = 0x826265C4;
	sub_82466E20(ctx, base);
	// 826265C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826265C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826265CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826265D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826265D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826265D8 size=108
    let mut pc: u32 = 0x826265D8;
    'dispatch: loop {
        match pc {
            0x826265D8 => {
    //   block [0x826265D8..0x82626644)
	// 826265D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826265DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826265E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826265E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826265E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826265EC: 38EB5DF0  addi r7, r11, 0x5df0
	ctx.r[7].s64 = ctx.r[11].s64 + 24048;
	// 826265F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826265F4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826265F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826265FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626608: 386A25FC  addi r3, r10, 0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9724;
	// 8262660C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262661C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262662C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626630: 4BE407F1  bl 0x82466e20
	ctx.lr = 0x82626634;
	sub_82466E20(ctx, base);
	// 82626634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262663C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626648 size=112
    let mut pc: u32 = 0x82626648;
    'dispatch: loop {
        match pc {
            0x82626648 => {
    //   block [0x82626648..0x826266B8)
	// 82626648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262664C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626658: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262665C: 392A3248  addi r9, r10, 0x3248
	ctx.r[9].s64 = ctx.r[10].s64 + 12872;
	// 82626660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626664: 390B5E28  addi r8, r11, 0x5e28
	ctx.r[8].s64 = ctx.r[11].s64 + 24104;
	// 82626668: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262666C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82626670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262667C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626680: 386A262C  addi r3, r10, 0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + 9772;
	// 82626684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262668C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262669C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826266A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826266A4: 4BE4077D  bl 0x82466e20
	ctx.lr = 0x826266A8;
	sub_82466E20(ctx, base);
	// 826266A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826266AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826266B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826266B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826266B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826266B8 size=108
    let mut pc: u32 = 0x826266B8;
    'dispatch: loop {
        match pc {
            0x826266B8 => {
    //   block [0x826266B8..0x82626724)
	// 826266B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826266BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826266C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826266C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826266C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826266CC: 38EB5E88  addi r7, r11, 0x5e88
	ctx.r[7].s64 = ctx.r[11].s64 + 24200;
	// 826266D0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826266D4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826266D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826266DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826266E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826266E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826266E8: 386A265C  addi r3, r10, 0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + 9820;
	// 826266EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826266F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826266F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826266F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826266FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262670C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626710: 4BE40711  bl 0x82466e20
	ctx.lr = 0x82626714;
	sub_82466E20(ctx, base);
	// 82626714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262671C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626728 size=108
    let mut pc: u32 = 0x82626728;
    'dispatch: loop {
        match pc {
            0x82626728 => {
    //   block [0x82626728..0x82626794)
	// 82626728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626734: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626738: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262673C: 38EB5F90  addi r7, r11, 0x5f90
	ctx.r[7].s64 = ctx.r[11].s64 + 24464;
	// 82626740: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626744: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82626748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262674C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626758: 386A268C  addi r3, r10, 0x268c
	ctx.r[3].s64 = ctx.r[10].s64 + 9868;
	// 8262675C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262676C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262677C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626780: 4BE406A1  bl 0x82466e20
	ctx.lr = 0x82626784;
	sub_82466E20(ctx, base);
	// 82626784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262678C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626798 size=108
    let mut pc: u32 = 0x82626798;
    'dispatch: loop {
        match pc {
            0x82626798 => {
    //   block [0x82626798..0x82626804)
	// 82626798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262679C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826267A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826267A4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826267A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826267AC: 38EB5FA8  addi r7, r11, 0x5fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 24488;
	// 826267B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826267B4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826267B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826267BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826267C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826267C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826267C8: 386A26BC  addi r3, r10, 0x26bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9916;
	// 826267CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826267D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826267D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826267D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826267DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826267E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826267E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826267E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826267EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826267F0: 4BE40631  bl 0x82466e20
	ctx.lr = 0x826267F4;
	sub_82466E20(ctx, base);
	// 826267F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826267F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826267FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626808 size=24
    let mut pc: u32 = 0x82626808;
    'dispatch: loop {
        match pc {
            0x82626808 => {
    //   block [0x82626808..0x82626820)
	// 82626808: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262680C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626810: 394AAFA8  addi r10, r10, -0x5058
	ctx.r[10].s64 = ctx.r[10].s64 + -20568;
	// 82626814: 816B5E24  lwz r11, 0x5e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24100 as u32) ) } as u64;
	// 82626818: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8262681C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626820 size=108
    let mut pc: u32 = 0x82626820;
    'dispatch: loop {
        match pc {
            0x82626820 => {
    //   block [0x82626820..0x8262688C)
	// 82626820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262682C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82626830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626834: 38EBAFA8  addi r7, r11, -0x5058
	ctx.r[7].s64 = ctx.r[11].s64 + -20568;
	// 82626838: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262683C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82626840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626844: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262684C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626850: 386A26EC  addi r3, r10, 0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9964;
	// 82626854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262685C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262686C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626878: 4BE405A9  bl 0x82466e20
	ctx.lr = 0x8262687C;
	sub_82466E20(ctx, base);
	// 8262687C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626890 size=24
    let mut pc: u32 = 0x82626890;
    'dispatch: loop {
        match pc {
            0x82626890 => {
    //   block [0x82626890..0x826268A8)
	// 82626890: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626894: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626898: 394AAFD8  addi r10, r10, -0x5028
	ctx.r[10].s64 = ctx.r[10].s64 + -20520;
	// 8262689C: 816B5E24  lwz r11, 0x5e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24100 as u32) ) } as u64;
	// 826268A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826268A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826268A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826268A8 size=108
    let mut pc: u32 = 0x826268A8;
    'dispatch: loop {
        match pc {
            0x826268A8 => {
    //   block [0x826268A8..0x82626914)
	// 826268A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826268AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826268B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826268B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826268B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826268BC: 38EBAFD8  addi r7, r11, -0x5028
	ctx.r[7].s64 = ctx.r[11].s64 + -20520;
	// 826268C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826268C4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826268C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826268CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826268D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826268D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826268D8: 386A271C  addi r3, r10, 0x271c
	ctx.r[3].s64 = ctx.r[10].s64 + 10012;
	// 826268DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826268E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826268E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826268E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826268EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826268F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826268F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826268F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826268FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626900: 4BE40521  bl 0x82466e20
	ctx.lr = 0x82626904;
	sub_82466E20(ctx, base);
	// 82626904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262690C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626918 size=108
    let mut pc: u32 = 0x82626918;
    'dispatch: loop {
        match pc {
            0x82626918 => {
    //   block [0x82626918..0x82626984)
	// 82626918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262691C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626924: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262692C: 38EB6020  addi r7, r11, 0x6020
	ctx.r[7].s64 = ctx.r[11].s64 + 24608;
	// 82626930: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626934: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82626938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262693C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626940: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626948: 386A274C  addi r3, r10, 0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + 10060;
	// 8262694C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262695C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262696C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626970: 4BE404B1  bl 0x82466e20
	ctx.lr = 0x82626974;
	sub_82466E20(ctx, base);
	// 82626974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262697C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626988 size=24
    let mut pc: u32 = 0x82626988;
    'dispatch: loop {
        match pc {
            0x82626988 => {
    //   block [0x82626988..0x826269A0)
	// 82626988: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262698C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626990: 394AB008  addi r10, r10, -0x4ff8
	ctx.r[10].s64 = ctx.r[10].s64 + -20472;
	// 82626994: 816B5E24  lwz r11, 0x5e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24100 as u32) ) } as u64;
	// 82626998: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8262699C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826269A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826269A0 size=108
    let mut pc: u32 = 0x826269A0;
    'dispatch: loop {
        match pc {
            0x826269A0 => {
    //   block [0x826269A0..0x82626A0C)
	// 826269A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826269A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826269A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826269AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826269B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826269B4: 38EBB008  addi r7, r11, -0x4ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -20472;
	// 826269B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826269BC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826269C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826269C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826269C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826269CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826269D0: 386A277C  addi r3, r10, 0x277c
	ctx.r[3].s64 = ctx.r[10].s64 + 10108;
	// 826269D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826269D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826269DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826269E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826269E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826269E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826269EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826269F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826269F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826269F8: 4BE40429  bl 0x82466e20
	ctx.lr = 0x826269FC;
	sub_82466E20(ctx, base);
	// 826269FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626A10 size=112
    let mut pc: u32 = 0x82626A10;
    'dispatch: loop {
        match pc {
            0x82626A10 => {
    //   block [0x82626A10..0x82626A80)
	// 82626A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626A1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626A20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626A24: 392A328C  addi r9, r10, 0x328c
	ctx.r[9].s64 = ctx.r[10].s64 + 12940;
	// 82626A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626A2C: 390B6038  addi r8, r11, 0x6038
	ctx.r[8].s64 = ctx.r[11].s64 + 24632;
	// 82626A30: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82626A34: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82626A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626A3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626A48: 386A27AC  addi r3, r10, 0x27ac
	ctx.r[3].s64 = ctx.r[10].s64 + 10156;
	// 82626A4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626A50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626A6C: 4BE403B5  bl 0x82466e20
	ctx.lr = 0x82626A70;
	sub_82466E20(ctx, base);
	// 82626A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626A80 size=108
    let mut pc: u32 = 0x82626A80;
    'dispatch: loop {
        match pc {
            0x82626A80 => {
    //   block [0x82626A80..0x82626AEC)
	// 82626A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626A8C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626A94: 38EB6068  addi r7, r11, 0x6068
	ctx.r[7].s64 = ctx.r[11].s64 + 24680;
	// 82626A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626A9C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82626AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626AB0: 386A27DC  addi r3, r10, 0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + 10204;
	// 82626AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626AD8: 4BE40349  bl 0x82466e20
	ctx.lr = 0x82626ADC;
	sub_82466E20(ctx, base);
	// 82626ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626AF0 size=108
    let mut pc: u32 = 0x82626AF0;
    'dispatch: loop {
        match pc {
            0x82626AF0 => {
    //   block [0x82626AF0..0x82626B5C)
	// 82626AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626AFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626B04: 38EB6098  addi r7, r11, 0x6098
	ctx.r[7].s64 = ctx.r[11].s64 + 24728;
	// 82626B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626B0C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82626B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626B14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626B20: 386A280C  addi r3, r10, 0x280c
	ctx.r[3].s64 = ctx.r[10].s64 + 10252;
	// 82626B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626B48: 4BE402D9  bl 0x82466e20
	ctx.lr = 0x82626B4C;
	sub_82466E20(ctx, base);
	// 82626B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626B60 size=112
    let mut pc: u32 = 0x82626B60;
    'dispatch: loop {
        match pc {
            0x82626B60 => {
    //   block [0x82626B60..0x82626BD0)
	// 82626B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626B6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626B70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626B74: 38AA286C  addi r5, r10, 0x286c
	ctx.r[5].s64 = ctx.r[10].s64 + 10348;
	// 82626B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626B7C: 390B60C8  addi r8, r11, 0x60c8
	ctx.r[8].s64 = ctx.r[11].s64 + 24776;
	// 82626B80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82626B84: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82626B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626B8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626B98: 386A283C  addi r3, r10, 0x283c
	ctx.r[3].s64 = ctx.r[10].s64 + 10300;
	// 82626B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82626BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626BBC: 4BE40265  bl 0x82466e20
	ctx.lr = 0x82626BC0;
	sub_82466E20(ctx, base);
	// 82626BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626BD0 size=108
    let mut pc: u32 = 0x82626BD0;
    'dispatch: loop {
        match pc {
            0x82626BD0 => {
    //   block [0x82626BD0..0x82626C3C)
	// 82626BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626BDC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626BE4: 38EB60E0  addi r7, r11, 0x60e0
	ctx.r[7].s64 = ctx.r[11].s64 + 24800;
	// 82626BE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626BEC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82626BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626C00: 386A286C  addi r3, r10, 0x286c
	ctx.r[3].s64 = ctx.r[10].s64 + 10348;
	// 82626C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626C28: 4BE401F9  bl 0x82466e20
	ctx.lr = 0x82626C2C;
	sub_82466E20(ctx, base);
	// 82626C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626C40 size=108
    let mut pc: u32 = 0x82626C40;
    'dispatch: loop {
        match pc {
            0x82626C40 => {
    //   block [0x82626C40..0x82626CAC)
	// 82626C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626C4C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626C54: 38EB6110  addi r7, r11, 0x6110
	ctx.r[7].s64 = ctx.r[11].s64 + 24848;
	// 82626C58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626C5C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82626C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626C64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626C70: 386A289C  addi r3, r10, 0x289c
	ctx.r[3].s64 = ctx.r[10].s64 + 10396;
	// 82626C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626C98: 4BE40189  bl 0x82466e20
	ctx.lr = 0x82626C9C;
	sub_82466E20(ctx, base);
	// 82626C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626CB0 size=108
    let mut pc: u32 = 0x82626CB0;
    'dispatch: loop {
        match pc {
            0x82626CB0 => {
    //   block [0x82626CB0..0x82626D1C)
	// 82626CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626CBC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626CC4: 38EB6128  addi r7, r11, 0x6128
	ctx.r[7].s64 = ctx.r[11].s64 + 24872;
	// 82626CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626CCC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82626CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626CD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626CE0: 386A28CC  addi r3, r10, 0x28cc
	ctx.r[3].s64 = ctx.r[10].s64 + 10444;
	// 82626CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626D08: 4BE40119  bl 0x82466e20
	ctx.lr = 0x82626D0C;
	sub_82466E20(ctx, base);
	// 82626D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626D20 size=108
    let mut pc: u32 = 0x82626D20;
    'dispatch: loop {
        match pc {
            0x82626D20 => {
    //   block [0x82626D20..0x82626D8C)
	// 82626D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626D2C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626D34: 38EB6158  addi r7, r11, 0x6158
	ctx.r[7].s64 = ctx.r[11].s64 + 24920;
	// 82626D38: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82626D3C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82626D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626D50: 386A28FC  addi r3, r10, 0x28fc
	ctx.r[3].s64 = ctx.r[10].s64 + 10492;
	// 82626D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626D78: 4BE400A9  bl 0x82466e20
	ctx.lr = 0x82626D7C;
	sub_82466E20(ctx, base);
	// 82626D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626D90 size=108
    let mut pc: u32 = 0x82626D90;
    'dispatch: loop {
        match pc {
            0x82626D90 => {
    //   block [0x82626D90..0x82626DFC)
	// 82626D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626D9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626DA4: 38EB6200  addi r7, r11, 0x6200
	ctx.r[7].s64 = ctx.r[11].s64 + 25088;
	// 82626DA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626DAC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82626DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626DC0: 386A292C  addi r3, r10, 0x292c
	ctx.r[3].s64 = ctx.r[10].s64 + 10540;
	// 82626DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626DE8: 4BE40039  bl 0x82466e20
	ctx.lr = 0x82626DEC;
	sub_82466E20(ctx, base);
	// 82626DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626E00 size=108
    let mut pc: u32 = 0x82626E00;
    'dispatch: loop {
        match pc {
            0x82626E00 => {
    //   block [0x82626E00..0x82626E6C)
	// 82626E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626E0C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626E14: 38EB6230  addi r7, r11, 0x6230
	ctx.r[7].s64 = ctx.r[11].s64 + 25136;
	// 82626E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626E1C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 82626E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626E24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626E30: 386A295C  addi r3, r10, 0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + 10588;
	// 82626E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626E58: 4BE3FFC9  bl 0x82466e20
	ctx.lr = 0x82626E5C;
	sub_82466E20(ctx, base);
	// 82626E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626E70 size=108
    let mut pc: u32 = 0x82626E70;
    'dispatch: loop {
        match pc {
            0x82626E70 => {
    //   block [0x82626E70..0x82626EDC)
	// 82626E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626E7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626E84: 38EB6248  addi r7, r11, 0x6248
	ctx.r[7].s64 = ctx.r[11].s64 + 25160;
	// 82626E88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626E8C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82626E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626E94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626EA0: 386A298C  addi r3, r10, 0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + 10636;
	// 82626EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626EC8: 4BE3FF59  bl 0x82466e20
	ctx.lr = 0x82626ECC;
	sub_82466E20(ctx, base);
	// 82626ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626EE0 size=108
    let mut pc: u32 = 0x82626EE0;
    'dispatch: loop {
        match pc {
            0x82626EE0 => {
    //   block [0x82626EE0..0x82626F4C)
	// 82626EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626EEC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626EF4: 38EB6278  addi r7, r11, 0x6278
	ctx.r[7].s64 = ctx.r[11].s64 + 25208;
	// 82626EF8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82626EFC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82626F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626F04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626F10: 386A29BC  addi r3, r10, 0x29bc
	ctx.r[3].s64 = ctx.r[10].s64 + 10684;
	// 82626F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626F38: 4BE3FEE9  bl 0x82466e20
	ctx.lr = 0x82626F3C;
	sub_82466E20(ctx, base);
	// 82626F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626F50 size=24
    let mut pc: u32 = 0x82626F50;
    'dispatch: loop {
        match pc {
            0x82626F50 => {
    //   block [0x82626F50..0x82626F68)
	// 82626F50: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626F54: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626F58: 394AB038  addi r10, r10, -0x4fc8
	ctx.r[10].s64 = ctx.r[10].s64 + -20424;
	// 82626F5C: 816B6338  lwz r11, 0x6338(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25400 as u32) ) } as u64;
	// 82626F60: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82626F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626F68 size=112
    let mut pc: u32 = 0x82626F68;
    'dispatch: loop {
        match pc {
            0x82626F68 => {
    //   block [0x82626F68..0x82626FD8)
	// 82626F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626F74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626F78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82626F7C: 392A32B8  addi r9, r10, 0x32b8
	ctx.r[9].s64 = ctx.r[10].s64 + 12984;
	// 82626F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626F84: 390BB038  addi r8, r11, -0x4fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -20424;
	// 82626F88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82626F8C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82626F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626F94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626FA0: 386A29EC  addi r3, r10, 0x29ec
	ctx.r[3].s64 = ctx.r[10].s64 + 10732;
	// 82626FA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626FA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626FC4: 4BE3FE5D  bl 0x82466e20
	ctx.lr = 0x82626FC8;
	sub_82466E20(ctx, base);
	// 82626FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626FD8 size=108
    let mut pc: u32 = 0x82626FD8;
    'dispatch: loop {
        match pc {
            0x82626FD8 => {
    //   block [0x82626FD8..0x82627044)
	// 82626FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626FE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626FEC: 38EB6340  addi r7, r11, 0x6340
	ctx.r[7].s64 = ctx.r[11].s64 + 25408;
	// 82626FF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626FF4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82626FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627008: 386A2A1C  addi r3, r10, 0x2a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 10780;
	// 8262700C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262701C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262702C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627030: 4BE3FDF1  bl 0x82466e20
	ctx.lr = 0x82627034;
	sub_82466E20(ctx, base);
	// 82627034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262703C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627048 size=112
    let mut pc: u32 = 0x82627048;
    'dispatch: loop {
        match pc {
            0x82627048 => {
    //   block [0x82627048..0x826270B8)
	// 82627048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262704C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627054: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82627058: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262705C: 392A32FC  addi r9, r10, 0x32fc
	ctx.r[9].s64 = ctx.r[10].s64 + 13052;
	// 82627060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627064: 390B6370  addi r8, r11, 0x6370
	ctx.r[8].s64 = ctx.r[11].s64 + 25456;
	// 82627068: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262706C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82627070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262707C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627080: 386A2A4C  addi r3, r10, 0x2a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 10828;
	// 82627084: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82627088: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262708C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262709C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826270A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826270A4: 4BE3FD7D  bl 0x82466e20
	ctx.lr = 0x826270A8;
	sub_82466E20(ctx, base);
	// 826270A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826270AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826270B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826270B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826270B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826270B8 size=24
    let mut pc: u32 = 0x826270B8;
    'dispatch: loop {
        match pc {
            0x826270B8 => {
    //   block [0x826270B8..0x826270D0)
	// 826270B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826270BC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826270C0: 394AB0B0  addi r10, r10, -0x4f50
	ctx.r[10].s64 = ctx.r[10].s64 + -20304;
	// 826270C4: 816B6430  lwz r11, 0x6430(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25648 as u32) ) } as u64;
	// 826270C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826270CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826270D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826270D0 size=112
    let mut pc: u32 = 0x826270D0;
    'dispatch: loop {
        match pc {
            0x826270D0 => {
    //   block [0x826270D0..0x82627140)
	// 826270D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826270D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826270D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826270DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826270E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826270E4: 392A3338  addi r9, r10, 0x3338
	ctx.r[9].s64 = ctx.r[10].s64 + 13112;
	// 826270E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826270EC: 390BB0B0  addi r8, r11, -0x4f50
	ctx.r[8].s64 = ctx.r[11].s64 + -20304;
	// 826270F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826270F4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826270F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826270FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627108: 386A2A7C  addi r3, r10, 0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 10876;
	// 8262710C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82627110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82627114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262711C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262712C: 4BE3FCF5  bl 0x82466e20
	ctx.lr = 0x82627130;
	sub_82466E20(ctx, base);
	// 82627130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262713C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627140 size=108
    let mut pc: u32 = 0x82627140;
    'dispatch: loop {
        match pc {
            0x82627140 => {
    //   block [0x82627140..0x826271AC)
	// 82627140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262714C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627154: 38EB6434  addi r7, r11, 0x6434
	ctx.r[7].s64 = ctx.r[11].s64 + 25652;
	// 82627158: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262715C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82627160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262716C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627170: 386A2AAC  addi r3, r10, 0x2aac
	ctx.r[3].s64 = ctx.r[10].s64 + 10924;
	// 82627174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262717C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262718C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627198: 4BE3FC89  bl 0x82466e20
	ctx.lr = 0x8262719C;
	sub_82466E20(ctx, base);
	// 8262719C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826271A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826271A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826271A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826271B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826271B0 size=108
    let mut pc: u32 = 0x826271B0;
    'dispatch: loop {
        match pc {
            0x826271B0 => {
    //   block [0x826271B0..0x8262721C)
	// 826271B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826271B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826271B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826271BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826271C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826271C4: 38EB644C  addi r7, r11, 0x644c
	ctx.r[7].s64 = ctx.r[11].s64 + 25676;
	// 826271C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826271CC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826271D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826271D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826271D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826271DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826271E0: 386A2ADC  addi r3, r10, 0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + 10972;
	// 826271E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826271E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826271EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826271F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826271F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826271F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826271FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627208: 4BE3FC19  bl 0x82466e20
	ctx.lr = 0x8262720C;
	sub_82466E20(ctx, base);
	// 8262720C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82627220 size=24
    let mut pc: u32 = 0x82627220;
    'dispatch: loop {
        match pc {
            0x82627220 => {
    //   block [0x82627220..0x82627238)
	// 82627220: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627224: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82627228: 394AB0F8  addi r10, r10, -0x4f08
	ctx.r[10].s64 = ctx.r[10].s64 + -20232;
	// 8262722C: 816B647C  lwz r11, 0x647c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25724 as u32) ) } as u64;
	// 82627230: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82627234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627238 size=112
    let mut pc: u32 = 0x82627238;
    'dispatch: loop {
        match pc {
            0x82627238 => {
    //   block [0x82627238..0x826272A8)
	// 82627238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262723C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627244: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82627248: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262724C: 392A3374  addi r9, r10, 0x3374
	ctx.r[9].s64 = ctx.r[10].s64 + 13172;
	// 82627250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627254: 390BB0F8  addi r8, r11, -0x4f08
	ctx.r[8].s64 = ctx.r[11].s64 + -20232;
	// 82627258: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8262725C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82627260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262726C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627270: 386A2B0C  addi r3, r10, 0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11020;
	// 82627274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82627278: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262727C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262728C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627294: 4BE3FB8D  bl 0x82466e20
	ctx.lr = 0x82627298;
	sub_82466E20(ctx, base);
	// 82627298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262729C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826272A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826272A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826272A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826272A8 size=108
    let mut pc: u32 = 0x826272A8;
    'dispatch: loop {
        match pc {
            0x826272A8 => {
    //   block [0x826272A8..0x82627314)
	// 826272A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826272AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826272B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826272B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826272B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826272BC: 38EB6480  addi r7, r11, 0x6480
	ctx.r[7].s64 = ctx.r[11].s64 + 25728;
	// 826272C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826272C4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826272C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826272CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826272D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826272D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826272D8: 386A2B3C  addi r3, r10, 0x2b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11068;
	// 826272DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826272E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826272E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826272E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826272EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826272F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826272F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826272F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826272FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627300: 4BE3FB21  bl 0x82466e20
	ctx.lr = 0x82627304;
	sub_82466E20(ctx, base);
	// 82627304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262730C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627318 size=108
    let mut pc: u32 = 0x82627318;
    'dispatch: loop {
        match pc {
            0x82627318 => {
    //   block [0x82627318..0x82627384)
	// 82627318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627324: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262732C: 38EB6498  addi r7, r11, 0x6498
	ctx.r[7].s64 = ctx.r[11].s64 + 25752;
	// 82627330: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82627334: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82627338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262733C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627348: 386A2B6C  addi r3, r10, 0x2b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11116;
	// 8262734C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262736C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627370: 4BE3FAB1  bl 0x82466e20
	ctx.lr = 0x82627374;
	sub_82466E20(ctx, base);
	// 82627374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262737C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627388 size=108
    let mut pc: u32 = 0x82627388;
    'dispatch: loop {
        match pc {
            0x82627388 => {
    //   block [0x82627388..0x826273F4)
	// 82627388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627394: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262739C: 38EB64E0  addi r7, r11, 0x64e0
	ctx.r[7].s64 = ctx.r[11].s64 + 25824;
	// 826273A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826273A4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826273A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826273AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826273B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826273B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826273B8: 386A2B9C  addi r3, r10, 0x2b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11164;
	// 826273BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826273C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826273C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826273C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826273CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826273D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826273D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826273D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826273DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826273E0: 4BE3FA41  bl 0x82466e20
	ctx.lr = 0x826273E4;
	sub_82466E20(ctx, base);
	// 826273E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826273E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826273EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826273F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826273F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826273F8 size=108
    let mut pc: u32 = 0x826273F8;
    'dispatch: loop {
        match pc {
            0x826273F8 => {
    //   block [0x826273F8..0x82627464)
	// 826273F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826273FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627404: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262740C: 38EB6510  addi r7, r11, 0x6510
	ctx.r[7].s64 = ctx.r[11].s64 + 25872;
	// 82627410: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82627414: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82627418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262741C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627428: 386A2BCC  addi r3, r10, 0x2bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 11212;
	// 8262742C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262743C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262744C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627450: 4BE3F9D1  bl 0x82466e20
	ctx.lr = 0x82627454;
	sub_82466E20(ctx, base);
	// 82627454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262745C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627468 size=108
    let mut pc: u32 = 0x82627468;
    'dispatch: loop {
        match pc {
            0x82627468 => {
    //   block [0x82627468..0x826274D4)
	// 82627468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627474: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262747C: 38EB6630  addi r7, r11, 0x6630
	ctx.r[7].s64 = ctx.r[11].s64 + 26160;
	// 82627480: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82627484: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82627488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262748C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627498: 386A2BFC  addi r3, r10, 0x2bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 11260;
	// 8262749C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826274A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826274A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826274A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826274AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826274B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826274B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826274B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826274BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826274C0: 4BE3F961  bl 0x82466e20
	ctx.lr = 0x826274C4;
	sub_82466E20(ctx, base);
	// 826274C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826274C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826274CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826274D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826274D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826274D8 size=108
    let mut pc: u32 = 0x826274D8;
    'dispatch: loop {
        match pc {
            0x826274D8 => {
    //   block [0x826274D8..0x82627544)
	// 826274D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826274DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826274E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826274E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826274E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826274EC: 38EB66C0  addi r7, r11, 0x66c0
	ctx.r[7].s64 = ctx.r[11].s64 + 26304;
	// 826274F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826274F4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826274F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826274FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627508: 386A2C2C  addi r3, r10, 0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 11308;
	// 8262750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627530: 4BE3F8F1  bl 0x82466e20
	ctx.lr = 0x82627534;
	sub_82466E20(ctx, base);
	// 82627534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627548 size=108
    let mut pc: u32 = 0x82627548;
    'dispatch: loop {
        match pc {
            0x82627548 => {
    //   block [0x82627548..0x826275B4)
	// 82627548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627554: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262755C: 38EB6780  addi r7, r11, 0x6780
	ctx.r[7].s64 = ctx.r[11].s64 + 26496;
	// 82627560: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82627564: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82627568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262756C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627578: 386A2C5C  addi r3, r10, 0x2c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 11356;
	// 8262757C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262758C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262759C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826275A0: 4BE3F881  bl 0x82466e20
	ctx.lr = 0x826275A4;
	sub_82466E20(ctx, base);
	// 826275A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826275A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826275AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826275B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826275B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826275B8 size=108
    let mut pc: u32 = 0x826275B8;
    'dispatch: loop {
        match pc {
            0x826275B8 => {
    //   block [0x826275B8..0x82627624)
	// 826275B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826275BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826275C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826275C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826275C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826275CC: 38EB6858  addi r7, r11, 0x6858
	ctx.r[7].s64 = ctx.r[11].s64 + 26712;
	// 826275D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826275D4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826275D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826275DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826275E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826275E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826275E8: 386A2C8C  addi r3, r10, 0x2c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 11404;
	// 826275EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826275F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826275F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826275F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826275FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262760C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627610: 4BE3F811  bl 0x82466e20
	ctx.lr = 0x82627614;
	sub_82466E20(ctx, base);
	// 82627614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627628 size=108
    let mut pc: u32 = 0x82627628;
    'dispatch: loop {
        match pc {
            0x82627628 => {
    //   block [0x82627628..0x82627694)
	// 82627628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627634: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262763C: 38EB6918  addi r7, r11, 0x6918
	ctx.r[7].s64 = ctx.r[11].s64 + 26904;
	// 82627640: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82627644: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82627648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262764C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627658: 386A2CBC  addi r3, r10, 0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 11452;
	// 8262765C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262766C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262767C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627680: 4BE3F7A1  bl 0x82466e20
	ctx.lr = 0x82627684;
	sub_82466E20(ctx, base);
	// 82627684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262768C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627698 size=112
    let mut pc: u32 = 0x82627698;
    'dispatch: loop {
        match pc {
            0x82627698 => {
    //   block [0x82627698..0x82627708)
	// 82627698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826276A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826276A4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826276A8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826276AC: 38EA69C0  addi r7, r10, 0x69c0
	ctx.r[7].s64 = ctx.r[10].s64 + 27072;
	// 826276B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826276B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826276B8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826276BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826276C0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826276C4: 396B3388  addi r11, r11, 0x3388
	ctx.r[11].s64 = ctx.r[11].s64 + 13192;
	// 826276C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826276CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826276D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826276D4: 386A2CEC  addi r3, r10, 0x2cec
	ctx.r[3].s64 = ctx.r[10].s64 + 11500;
	// 826276D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826276DC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826276E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826276E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826276E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826276EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826276F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826276F4: 4BE3F72D  bl 0x82466e20
	ctx.lr = 0x826276F8;
	sub_82466E20(ctx, base);
	// 826276F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826276FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627708 size=108
    let mut pc: u32 = 0x82627708;
    'dispatch: loop {
        match pc {
            0x82627708 => {
    //   block [0x82627708..0x82627774)
	// 82627708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262770C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627714: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262771C: 38EB6AE0  addi r7, r11, 0x6ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 27360;
	// 82627720: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82627724: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82627728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262772C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627738: 386A2D1C  addi r3, r10, 0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11548;
	// 8262773C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262774C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262775C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627760: 4BE3F6C1  bl 0x82466e20
	ctx.lr = 0x82627764;
	sub_82466E20(ctx, base);
	// 82627764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627778 size=108
    let mut pc: u32 = 0x82627778;
    'dispatch: loop {
        match pc {
            0x82627778 => {
    //   block [0x82627778..0x826277E4)
	// 82627778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627784: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262778C: 38EB6B40  addi r7, r11, 0x6b40
	ctx.r[7].s64 = ctx.r[11].s64 + 27456;
	// 82627790: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82627794: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82627798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262779C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826277A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826277A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826277A8: 386A2D4C  addi r3, r10, 0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11596;
	// 826277AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826277B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826277B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826277B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826277BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826277C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826277C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826277C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826277CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826277D0: 4BE3F651  bl 0x82466e20
	ctx.lr = 0x826277D4;
	sub_82466E20(ctx, base);
	// 826277D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826277D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826277DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826277E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826277E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826277E8 size=108
    let mut pc: u32 = 0x826277E8;
    'dispatch: loop {
        match pc {
            0x826277E8 => {
    //   block [0x826277E8..0x82627854)
	// 826277E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826277EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826277F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826277F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826277F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826277FC: 38EB6C48  addi r7, r11, 0x6c48
	ctx.r[7].s64 = ctx.r[11].s64 + 27720;
	// 82627800: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82627804: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82627808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262780C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627818: 386A2D7C  addi r3, r10, 0x2d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11644;
	// 8262781C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262782C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262783C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627840: 4BE3F5E1  bl 0x82466e20
	ctx.lr = 0x82627844;
	sub_82466E20(ctx, base);
	// 82627844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262784C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627858 size=108
    let mut pc: u32 = 0x82627858;
    'dispatch: loop {
        match pc {
            0x82627858 => {
    //   block [0x82627858..0x826278C4)
	// 82627858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627864: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262786C: 38EB6D20  addi r7, r11, 0x6d20
	ctx.r[7].s64 = ctx.r[11].s64 + 27936;
	// 82627870: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82627874: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82627878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262787C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627888: 386A2DAC  addi r3, r10, 0x2dac
	ctx.r[3].s64 = ctx.r[10].s64 + 11692;
	// 8262788C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262789C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826278A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826278A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826278A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826278AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826278B0: 4BE3F571  bl 0x82466e20
	ctx.lr = 0x826278B4;
	sub_82466E20(ctx, base);
	// 826278B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826278B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826278BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826278C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826278C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826278C8 size=108
    let mut pc: u32 = 0x826278C8;
    'dispatch: loop {
        match pc {
            0x826278C8 => {
    //   block [0x826278C8..0x82627934)
	// 826278C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826278CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826278D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826278D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826278D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826278DC: 38EB6D68  addi r7, r11, 0x6d68
	ctx.r[7].s64 = ctx.r[11].s64 + 28008;
	// 826278E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826278E4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826278E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826278EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826278F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826278F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826278F8: 386A2DDC  addi r3, r10, 0x2ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 11740;
	// 826278FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262790C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262791C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627920: 4BE3F501  bl 0x82466e20
	ctx.lr = 0x82627924;
	sub_82466E20(ctx, base);
	// 82627924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262792C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627938 size=108
    let mut pc: u32 = 0x82627938;
    'dispatch: loop {
        match pc {
            0x82627938 => {
    //   block [0x82627938..0x826279A4)
	// 82627938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262793C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627944: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262794C: 38EB6D80  addi r7, r11, 0x6d80
	ctx.r[7].s64 = ctx.r[11].s64 + 28032;
	// 82627950: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82627954: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82627958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262795C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627968: 386A2E0C  addi r3, r10, 0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11788;
	// 8262796C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262797C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262798C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627990: 4BE3F491  bl 0x82466e20
	ctx.lr = 0x82627994;
	sub_82466E20(ctx, base);
	// 82627994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262799C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826279A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826279A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826279A8 size=108
    let mut pc: u32 = 0x826279A8;
    'dispatch: loop {
        match pc {
            0x826279A8 => {
    //   block [0x826279A8..0x82627A14)
	// 826279A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826279AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826279B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826279B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826279B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826279BC: 38EB6DC8  addi r7, r11, 0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 28104;
	// 826279C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826279C4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826279C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826279CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826279D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826279D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826279D8: 386A2E3C  addi r3, r10, 0x2e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11836;
	// 826279DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826279E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826279E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826279E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826279EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826279F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826279F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826279F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826279FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627A00: 4BE3F421  bl 0x82466e20
	ctx.lr = 0x82627A04;
	sub_82466E20(ctx, base);
	// 82627A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627A18 size=112
    let mut pc: u32 = 0x82627A18;
    'dispatch: loop {
        match pc {
            0x82627A18 => {
    //   block [0x82627A18..0x82627A88)
	// 82627A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627A24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627A28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627A2C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627A34: 390B6DE0  addi r8, r11, 0x6de0
	ctx.r[8].s64 = ctx.r[11].s64 + 28128;
	// 82627A38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82627A3C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82627A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627A44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627A50: 386A2E6C  addi r3, r10, 0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11884;
	// 82627A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627A74: 4BE3F3AD  bl 0x82466e20
	ctx.lr = 0x82627A78;
	sub_82466E20(ctx, base);
	// 82627A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627A88 size=108
    let mut pc: u32 = 0x82627A88;
    'dispatch: loop {
        match pc {
            0x82627A88 => {
    //   block [0x82627A88..0x82627AF4)
	// 82627A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627A94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627A9C: 38EB6E28  addi r7, r11, 0x6e28
	ctx.r[7].s64 = ctx.r[11].s64 + 28200;
	// 82627AA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82627AA4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82627AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627AAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627AB8: 386A2E9C  addi r3, r10, 0x2e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11932;
	// 82627ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627AE0: 4BE3F341  bl 0x82466e20
	ctx.lr = 0x82627AE4;
	sub_82466E20(ctx, base);
	// 82627AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627AF8 size=112
    let mut pc: u32 = 0x82627AF8;
    'dispatch: loop {
        match pc {
            0x82627AF8 => {
    //   block [0x82627AF8..0x82627B68)
	// 82627AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627B04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627B08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627B0C: 38AA2E9C  addi r5, r10, 0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + 11932;
	// 82627B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627B14: 390B6E88  addi r8, r11, 0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + 28296;
	// 82627B18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82627B1C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82627B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627B24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627B30: 386A2ECC  addi r3, r10, 0x2ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 11980;
	// 82627B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627B54: 4BE3F2CD  bl 0x82466e20
	ctx.lr = 0x82627B58;
	sub_82466E20(ctx, base);
	// 82627B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627B68 size=96
    let mut pc: u32 = 0x82627B68;
    'dispatch: loop {
        match pc {
            0x82627B68 => {
    //   block [0x82627B68..0x82627BC8)
	// 82627B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627B74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627B7C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82627B80: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627B88: 386A2EFC  addi r3, r10, 0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + 12028;
	// 82627B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627B94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627BA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627BB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627BB4: 4BE3F26D  bl 0x82466e20
	ctx.lr = 0x82627BB8;
	sub_82466E20(ctx, base);
	// 82627BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627BC8 size=112
    let mut pc: u32 = 0x82627BC8;
    'dispatch: loop {
        match pc {
            0x82627BC8 => {
    //   block [0x82627BC8..0x82627C38)
	// 82627BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627BD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627BD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627BDC: 38AA4CFC  addi r5, r10, 0x4cfc
	ctx.r[5].s64 = ctx.r[10].s64 + 19708;
	// 82627BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627BE4: 390B6ED0  addi r8, r11, 0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 28368;
	// 82627BE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82627BEC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82627BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627C00: 386A2F2C  addi r3, r10, 0x2f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 12076;
	// 82627C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627C24: 4BE3F1FD  bl 0x82466e20
	ctx.lr = 0x82627C28;
	sub_82466E20(ctx, base);
	// 82627C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627C38 size=96
    let mut pc: u32 = 0x82627C38;
    'dispatch: loop {
        match pc {
            0x82627C38 => {
    //   block [0x82627C38..0x82627C98)
	// 82627C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627C44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627C4C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82627C50: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627C58: 386A2F5C  addi r3, r10, 0x2f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 12124;
	// 82627C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627C64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627C78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627C80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627C84: 4BE3F19D  bl 0x82466e20
	ctx.lr = 0x82627C88;
	sub_82466E20(ctx, base);
	// 82627C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627C98 size=100
    let mut pc: u32 = 0x82627C98;
    'dispatch: loop {
        match pc {
            0x82627C98 => {
    //   block [0x82627C98..0x82627CFC)
	// 82627C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627CA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627CAC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627CB8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82627CBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627CC4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82627CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627CCC: 386A2F8C  addi r3, r10, 0x2f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 12172;
	// 82627CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627CD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627CE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627CE8: 4BE3F139  bl 0x82466e20
	ctx.lr = 0x82627CEC;
	sub_82466E20(ctx, base);
	// 82627CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627D00 size=104
    let mut pc: u32 = 0x82627D00;
    'dispatch: loop {
        match pc {
            0x82627D00 => {
    //   block [0x82627D00..0x82627D68)
	// 82627D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627D0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82627D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627D14: 392A3400  addi r9, r10, 0x3400
	ctx.r[9].s64 = ctx.r[10].s64 + 13312;
	// 82627D18: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627D20: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627D34: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82627D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627D3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627D40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627D48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627D4C: 386A2FBC  addi r3, r10, 0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 12220;
	// 82627D50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82627D54: 4BE3F0CD  bl 0x82466e20
	ctx.lr = 0x82627D58;
	sub_82466E20(ctx, base);
	// 82627D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627D68 size=96
    let mut pc: u32 = 0x82627D68;
    'dispatch: loop {
        match pc {
            0x82627D68 => {
    //   block [0x82627D68..0x82627DC8)
	// 82627D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627D7C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82627D80: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627D88: 386A2FEC  addi r3, r10, 0x2fec
	ctx.r[3].s64 = ctx.r[10].s64 + 12268;
	// 82627D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627D94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627DA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627DB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627DB4: 4BE3F06D  bl 0x82466e20
	ctx.lr = 0x82627DB8;
	sub_82466E20(ctx, base);
	// 82627DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627DC8 size=100
    let mut pc: u32 = 0x82627DC8;
    'dispatch: loop {
        match pc {
            0x82627DC8 => {
    //   block [0x82627DC8..0x82627E2C)
	// 82627DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627DD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627DDC: 38AA2FBC  addi r5, r10, 0x2fbc
	ctx.r[5].s64 = ctx.r[10].s64 + 12220;
	// 82627DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627DE8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82627DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627DFC: 386A301C  addi r3, r10, 0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + 12316;
	// 82627E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627E04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627E08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627E10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627E18: 4BE3F009  bl 0x82466e20
	ctx.lr = 0x82627E1C;
	sub_82466E20(ctx, base);
	// 82627E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627E30 size=112
    let mut pc: u32 = 0x82627E30;
    'dispatch: loop {
        match pc {
            0x82627E30 => {
    //   block [0x82627E30..0x82627EA0)
	// 82627E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627E40: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627E44: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 82627E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627E4C: 390B6F34  addi r8, r11, 0x6f34
	ctx.r[8].s64 = ctx.r[11].s64 + 28468;
	// 82627E50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82627E54: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82627E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627E68: 386A304C  addi r3, r10, 0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + 12364;
	// 82627E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627E8C: 4BE3EF95  bl 0x82466e20
	ctx.lr = 0x82627E90;
	sub_82466E20(ctx, base);
	// 82627E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627EA0 size=112
    let mut pc: u32 = 0x82627EA0;
    'dispatch: loop {
        match pc {
            0x82627EA0 => {
    //   block [0x82627EA0..0x82627F10)
	// 82627EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627EAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627EB0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627EB4: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 82627EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627EBC: 390B6F64  addi r8, r11, 0x6f64
	ctx.r[8].s64 = ctx.r[11].s64 + 28516;
	// 82627EC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82627EC4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82627EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627ECC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627ED8: 386A307C  addi r3, r10, 0x307c
	ctx.r[3].s64 = ctx.r[10].s64 + 12412;
	// 82627EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627EFC: 4BE3EF25  bl 0x82466e20
	ctx.lr = 0x82627F00;
	sub_82466E20(ctx, base);
	// 82627F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627F10 size=100
    let mut pc: u32 = 0x82627F10;
    'dispatch: loop {
        match pc {
            0x82627F10 => {
    //   block [0x82627F10..0x82627F74)
	// 82627F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627F1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627F24: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 82627F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627F30: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82627F34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627F44: 386A30AC  addi r3, r10, 0x30ac
	ctx.r[3].s64 = ctx.r[10].s64 + 12460;
	// 82627F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627F50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627F58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627F60: 4BE3EEC1  bl 0x82466e20
	ctx.lr = 0x82627F64;
	sub_82466E20(ctx, base);
	// 82627F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627F78 size=96
    let mut pc: u32 = 0x82627F78;
    'dispatch: loop {
        match pc {
            0x82627F78 => {
    //   block [0x82627F78..0x82627FD8)
	// 82627F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627F84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627F8C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82627F90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627F98: 386A30DC  addi r3, r10, 0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + 12508;
	// 82627F9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627FA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627FB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627FC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627FC4: 4BE3EE5D  bl 0x82466e20
	ctx.lr = 0x82627FC8;
	sub_82466E20(ctx, base);
	// 82627FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627FD8 size=112
    let mut pc: u32 = 0x82627FD8;
    'dispatch: loop {
        match pc {
            0x82627FD8 => {
    //   block [0x82627FD8..0x82628048)
	// 82627FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627FE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627FE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627FEC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627FF4: 390B6F7C  addi r8, r11, 0x6f7c
	ctx.r[8].s64 = ctx.r[11].s64 + 28540;
	// 82627FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82627FFC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82628000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262800C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628010: 386A310C  addi r3, r10, 0x310c
	ctx.r[3].s64 = ctx.r[10].s64 + 12556;
	// 82628014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262801C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262802C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628034: 4BE3EDED  bl 0x82466e20
	ctx.lr = 0x82628038;
	sub_82466E20(ctx, base);
	// 82628038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262803C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628048 size=96
    let mut pc: u32 = 0x82628048;
    'dispatch: loop {
        match pc {
            0x82628048 => {
    //   block [0x82628048..0x826280A8)
	// 82628048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262804C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262805C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82628060: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628068: 386A313C  addi r3, r10, 0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + 12604;
	// 8262806C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628074: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82628078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262807C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262808C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628094: 4BE3ED8D  bl 0x82466e20
	ctx.lr = 0x82628098;
	sub_82466E20(ctx, base);
	// 82628098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262809C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826280A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826280A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826280A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826280A8 size=112
    let mut pc: u32 = 0x826280A8;
    'dispatch: loop {
        match pc {
            0x826280A8 => {
    //   block [0x826280A8..0x82628118)
	// 826280A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826280AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826280B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826280B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826280B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826280BC: 38AA313C  addi r5, r10, 0x313c
	ctx.r[5].s64 = ctx.r[10].s64 + 12604;
	// 826280C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826280C4: 390B6F94  addi r8, r11, 0x6f94
	ctx.r[8].s64 = ctx.r[11].s64 + 28564;
	// 826280C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826280CC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826280D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826280D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826280D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826280DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826280E0: 386A316C  addi r3, r10, 0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + 12652;
	// 826280E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826280E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826280EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826280F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826280F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826280F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826280FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628104: 4BE3ED1D  bl 0x82466e20
	ctx.lr = 0x82628108;
	sub_82466E20(ctx, base);
	// 82628108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262810C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628118 size=108
    let mut pc: u32 = 0x82628118;
    'dispatch: loop {
        match pc {
            0x82628118 => {
    //   block [0x82628118..0x82628184)
	// 82628118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262811C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628124: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262812C: 38EB6FB0  addi r7, r11, 0x6fb0
	ctx.r[7].s64 = ctx.r[11].s64 + 28592;
	// 82628130: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82628134: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82628138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262813C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628148: 386A319C  addi r3, r10, 0x319c
	ctx.r[3].s64 = ctx.r[10].s64 + 12700;
	// 8262814C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262815C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262816C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628170: 4BE3ECB1  bl 0x82466e20
	ctx.lr = 0x82628174;
	sub_82466E20(ctx, base);
	// 82628174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262817C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628188 size=112
    let mut pc: u32 = 0x82628188;
    'dispatch: loop {
        match pc {
            0x82628188 => {
    //   block [0x82628188..0x826281F8)
	// 82628188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262818C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628198: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262819C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 826281A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826281A4: 390B7010  addi r8, r11, 0x7010
	ctx.r[8].s64 = ctx.r[11].s64 + 28688;
	// 826281A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826281AC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826281B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826281B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826281B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826281BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826281C0: 386A31CC  addi r3, r10, 0x31cc
	ctx.r[3].s64 = ctx.r[10].s64 + 12748;
	// 826281C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826281C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826281CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826281D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826281D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826281D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826281DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826281E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826281E4: 4BE3EC3D  bl 0x82466e20
	ctx.lr = 0x826281E8;
	sub_82466E20(ctx, base);
	// 826281E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826281EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826281F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826281F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826281F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826281F8 size=112
    let mut pc: u32 = 0x826281F8;
    'dispatch: loop {
        match pc {
            0x826281F8 => {
    //   block [0x826281F8..0x82628268)
	// 826281F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826281FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628204: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628208: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262820C: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628214: 390B7028  addi r8, r11, 0x7028
	ctx.r[8].s64 = ctx.r[11].s64 + 28712;
	// 82628218: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262821C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82628220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262822C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628230: 386A31FC  addi r3, r10, 0x31fc
	ctx.r[3].s64 = ctx.r[10].s64 + 12796;
	// 82628234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262823C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262824C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628254: 4BE3EBCD  bl 0x82466e20
	ctx.lr = 0x82628258;
	sub_82466E20(ctx, base);
	// 82628258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262825C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628268 size=112
    let mut pc: u32 = 0x82628268;
    'dispatch: loop {
        match pc {
            0x82628268 => {
    //   block [0x82628268..0x826282D8)
	// 82628268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262826C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628278: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262827C: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628284: 390B7058  addi r8, r11, 0x7058
	ctx.r[8].s64 = ctx.r[11].s64 + 28760;
	// 82628288: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262828C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82628290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262829C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826282A0: 386A322C  addi r3, r10, 0x322c
	ctx.r[3].s64 = ctx.r[10].s64 + 12844;
	// 826282A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826282A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826282AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826282B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826282B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826282B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826282BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826282C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826282C4: 4BE3EB5D  bl 0x82466e20
	ctx.lr = 0x826282C8;
	sub_82466E20(ctx, base);
	// 826282C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826282CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826282D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826282D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826282D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826282D8 size=116
    let mut pc: u32 = 0x826282D8;
    'dispatch: loop {
        match pc {
            0x826282D8 => {
    //   block [0x826282D8..0x8262834C)
	// 826282D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826282DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826282E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826282E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826282E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826282EC: 390B7070  addi r8, r11, 0x7070
	ctx.r[8].s64 = ctx.r[11].s64 + 28784;
	// 826282F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826282F4: 392A342C  addi r9, r10, 0x342c
	ctx.r[9].s64 = ctx.r[10].s64 + 13356;
	// 826282F8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826282FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82628300: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628304: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262830C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262831C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82628320: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82628324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82628328: 386B325C  addi r3, r11, 0x325c
	ctx.r[3].s64 = ctx.r[11].s64 + 12892;
	// 8262832C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82628330: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628338: 4BE3EAE9  bl 0x82466e20
	ctx.lr = 0x8262833C;
	sub_82466E20(ctx, base);
	// 8262833C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628350 size=112
    let mut pc: u32 = 0x82628350;
    'dispatch: loop {
        match pc {
            0x82628350 => {
    //   block [0x82628350..0x826283C0)
	// 82628350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262835C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628360: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628364: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262836C: 390B70A0  addi r8, r11, 0x70a0
	ctx.r[8].s64 = ctx.r[11].s64 + 28832;
	// 82628370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82628374: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82628378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262837C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628388: 386A328C  addi r3, r10, 0x328c
	ctx.r[3].s64 = ctx.r[10].s64 + 12940;
	// 8262838C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262839C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826283A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826283A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826283A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826283AC: 4BE3EA75  bl 0x82466e20
	ctx.lr = 0x826283B0;
	sub_82466E20(ctx, base);
	// 826283B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826283B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826283B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826283BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826283C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826283C0 size=112
    let mut pc: u32 = 0x826283C0;
    'dispatch: loop {
        match pc {
            0x826283C0 => {
    //   block [0x826283C0..0x82628430)
	// 826283C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826283C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826283C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826283CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826283D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826283D4: 38AA382C  addi r5, r10, 0x382c
	ctx.r[5].s64 = ctx.r[10].s64 + 14380;
	// 826283D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826283DC: 390B70B8  addi r8, r11, 0x70b8
	ctx.r[8].s64 = ctx.r[11].s64 + 28856;
	// 826283E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826283E4: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826283E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826283EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826283F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826283F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826283F8: 386A32BC  addi r3, r10, 0x32bc
	ctx.r[3].s64 = ctx.r[10].s64 + 12988;
	// 826283FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262840C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262841C: 4BE3EA05  bl 0x82466e20
	ctx.lr = 0x82628420;
	sub_82466E20(ctx, base);
	// 82628420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628430 size=116
    let mut pc: u32 = 0x82628430;
    'dispatch: loop {
        match pc {
            0x82628430 => {
    //   block [0x82628430..0x826284A4)
	// 82628430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262843C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82628440: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82628444: 390A70D0  addi r8, r10, 0x70d0
	ctx.r[8].s64 = ctx.r[10].s64 + 28880;
	// 82628448: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262844C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82628450: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628454: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628458: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262845C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628464: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82628468: 396B3440  addi r11, r11, 0x3440
	ctx.r[11].s64 = ctx.r[11].s64 + 13376;
	// 8262846C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628470: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82628474: 386A32EC  addi r3, r10, 0x32ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13036;
	// 82628478: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262847C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628480: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82628484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262848C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628490: 4BE3E991  bl 0x82466e20
	ctx.lr = 0x82628494;
	sub_82466E20(ctx, base);
	// 82628494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262849C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826284A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826284A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826284A8 size=112
    let mut pc: u32 = 0x826284A8;
    'dispatch: loop {
        match pc {
            0x826284A8 => {
    //   block [0x826284A8..0x82628518)
	// 826284A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826284AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826284B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826284B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826284B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826284BC: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 826284C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826284C4: 390B7148  addi r8, r11, 0x7148
	ctx.r[8].s64 = ctx.r[11].s64 + 29000;
	// 826284C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826284CC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826284D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826284D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826284D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826284DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826284E0: 386A331C  addi r3, r10, 0x331c
	ctx.r[3].s64 = ctx.r[10].s64 + 13084;
	// 826284E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826284E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826284EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826284F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826284F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826284F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826284FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628504: 4BE3E91D  bl 0x82466e20
	ctx.lr = 0x82628508;
	sub_82466E20(ctx, base);
	// 82628508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262850C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628518 size=112
    let mut pc: u32 = 0x82628518;
    'dispatch: loop {
        match pc {
            0x82628518 => {
    //   block [0x82628518..0x82628588)
	// 82628518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262851C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628528: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262852C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628534: 390B7190  addi r8, r11, 0x7190
	ctx.r[8].s64 = ctx.r[11].s64 + 29072;
	// 82628538: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262853C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82628540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262854C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628550: 386A334C  addi r3, r10, 0x334c
	ctx.r[3].s64 = ctx.r[10].s64 + 13132;
	// 82628554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262855C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262856C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628574: 4BE3E8AD  bl 0x82466e20
	ctx.lr = 0x82628578;
	sub_82466E20(ctx, base);
	// 82628578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262857C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628588 size=112
    let mut pc: u32 = 0x82628588;
    'dispatch: loop {
        match pc {
            0x82628588 => {
    //   block [0x82628588..0x826285F8)
	// 82628588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262858C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628594: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628598: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262859C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 826285A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826285A4: 390B71C0  addi r8, r11, 0x71c0
	ctx.r[8].s64 = ctx.r[11].s64 + 29120;
	// 826285A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826285AC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826285B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826285B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826285B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826285BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826285C0: 386A337C  addi r3, r10, 0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + 13180;
	// 826285C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826285C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826285CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826285D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826285D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826285D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826285DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826285E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826285E4: 4BE3E83D  bl 0x82466e20
	ctx.lr = 0x826285E8;
	sub_82466E20(ctx, base);
	// 826285E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826285EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826285F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826285F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826285F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826285F8 size=108
    let mut pc: u32 = 0x826285F8;
    'dispatch: loop {
        match pc {
            0x826285F8 => {
    //   block [0x826285F8..0x82628664)
	// 826285F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826285FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628604: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262860C: 38EB71F0  addi r7, r11, 0x71f0
	ctx.r[7].s64 = ctx.r[11].s64 + 29168;
	// 82628610: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82628614: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82628618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262861C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628628: 386A33AC  addi r3, r10, 0x33ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13228;
	// 8262862C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262863C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262864C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628650: 4BE3E7D1  bl 0x82466e20
	ctx.lr = 0x82628654;
	sub_82466E20(ctx, base);
	// 82628654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262865C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628668 size=112
    let mut pc: u32 = 0x82628668;
    'dispatch: loop {
        match pc {
            0x82628668 => {
    //   block [0x82628668..0x826286D8)
	// 82628668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262866C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628678: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262867C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628684: 390B7238  addi r8, r11, 0x7238
	ctx.r[8].s64 = ctx.r[11].s64 + 29240;
	// 82628688: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262868C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82628690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262869C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826286A0: 386A33DC  addi r3, r10, 0x33dc
	ctx.r[3].s64 = ctx.r[10].s64 + 13276;
	// 826286A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826286A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826286AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826286B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826286B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826286B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826286BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826286C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826286C4: 4BE3E75D  bl 0x82466e20
	ctx.lr = 0x826286C8;
	sub_82466E20(ctx, base);
	// 826286C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826286CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826286D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826286D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826286D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826286D8 size=116
    let mut pc: u32 = 0x826286D8;
    'dispatch: loop {
        match pc {
            0x826286D8 => {
    //   block [0x826286D8..0x8262874C)
	// 826286D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826286DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826286E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826286E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826286E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826286EC: 392B3480  addi r9, r11, 0x3480
	ctx.r[9].s64 = ctx.r[11].s64 + 13440;
	// 826286F0: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 826286F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826286F8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826286FC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82628700: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628704: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82628708: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262870C: 396B72B8  addi r11, r11, 0x72b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29368;
	// 82628710: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82628714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628718: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262871C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628720: 386A340C  addi r3, r10, 0x340c
	ctx.r[3].s64 = ctx.r[10].s64 + 13324;
	// 82628724: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82628728: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262872C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628730: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82628734: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628738: 4BE3E6E9  bl 0x82466e20
	ctx.lr = 0x8262873C;
	sub_82466E20(ctx, base);
	// 8262873C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82628750 size=36
    let mut pc: u32 = 0x82628750;
    'dispatch: loop {
        match pc {
            0x82628750 => {
    //   block [0x82628750..0x82628774)
	// 82628750: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628754: 814B734C  lwz r10, 0x734c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29516 as u32) ) } as u64;
	// 82628758: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262875C: 396BB170  addi r11, r11, -0x4e90
	ctx.r[11].s64 = ctx.r[11].s64 + -20112;
	// 82628760: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82628764: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82628768: 814A72B4  lwz r10, 0x72b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29364 as u32) ) } as u64;
	// 8262876C: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82628770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628778 size=108
    let mut pc: u32 = 0x82628778;
    'dispatch: loop {
        match pc {
            0x82628778 => {
    //   block [0x82628778..0x826287E4)
	// 82628778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262877C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628784: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82628788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262878C: 38EBB170  addi r7, r11, -0x4e90
	ctx.r[7].s64 = ctx.r[11].s64 + -20112;
	// 82628790: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82628794: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82628798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262879C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826287A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826287A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826287A8: 386A343C  addi r3, r10, 0x343c
	ctx.r[3].s64 = ctx.r[10].s64 + 13372;
	// 826287AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826287B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826287B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826287B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826287BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826287C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826287C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826287C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826287CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826287D0: 4BE3E651  bl 0x82466e20
	ctx.lr = 0x826287D4;
	sub_82466E20(ctx, base);
	// 826287D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826287D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826287DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826287E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826287E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826287E8 size=24
    let mut pc: u32 = 0x826287E8;
    'dispatch: loop {
        match pc {
            0x826287E8 => {
    //   block [0x826287E8..0x82628800)
	// 826287E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826287EC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826287F0: 394AB230  addi r10, r10, -0x4dd0
	ctx.r[10].s64 = ctx.r[10].s64 + -19920;
	// 826287F4: 816B72B4  lwz r11, 0x72b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29364 as u32) ) } as u64;
	// 826287F8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826287FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628800 size=112
    let mut pc: u32 = 0x82628800;
    'dispatch: loop {
        match pc {
            0x82628800 => {
    //   block [0x82628800..0x82628870)
	// 82628800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262880C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628810: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82628814: 38AA343C  addi r5, r10, 0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + 13372;
	// 82628818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262881C: 390BB230  addi r8, r11, -0x4dd0
	ctx.r[8].s64 = ctx.r[11].s64 + -19920;
	// 82628820: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82628824: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 82628828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262882C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628838: 386A346C  addi r3, r10, 0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + 13420;
	// 8262883C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262885C: 4BE3E5C5  bl 0x82466e20
	ctx.lr = 0x82628860;
	sub_82466E20(ctx, base);
	// 82628860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262886C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628870 size=112
    let mut pc: u32 = 0x82628870;
    'dispatch: loop {
        match pc {
            0x82628870 => {
    //   block [0x82628870..0x826288E0)
	// 82628870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262887C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628880: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628884: 38AA343C  addi r5, r10, 0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + 13372;
	// 82628888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262888C: 390B7350  addi r8, r11, 0x7350
	ctx.r[8].s64 = ctx.r[11].s64 + 29520;
	// 82628890: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82628894: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82628898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262889C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826288A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826288A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826288A8: 386A349C  addi r3, r10, 0x349c
	ctx.r[3].s64 = ctx.r[10].s64 + 13468;
	// 826288AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826288B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826288B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826288B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826288BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826288C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826288C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826288C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826288CC: 4BE3E555  bl 0x82466e20
	ctx.lr = 0x826288D0;
	sub_82466E20(ctx, base);
	// 826288D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826288D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826288D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826288DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826288E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826288E0 size=116
    let mut pc: u32 = 0x826288E0;
    'dispatch: loop {
        match pc {
            0x826288E0 => {
    //   block [0x826288E0..0x82628954)
	// 826288E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826288E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826288E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826288EC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826288F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826288F4: 390B73B0  addi r8, r11, 0x73b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29616;
	// 826288F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826288FC: 392A3500  addi r9, r10, 0x3500
	ctx.r[9].s64 = ctx.r[10].s64 + 13568;
	// 82628900: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628904: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82628908: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 8262890C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628914: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262891C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628924: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82628928: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8262892C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82628930: 386B34CC  addi r3, r11, 0x34cc
	ctx.r[3].s64 = ctx.r[11].s64 + 13516;
	// 82628934: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82628938: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628940: 4BE3E4E1  bl 0x82466e20
	ctx.lr = 0x82628944;
	sub_82466E20(ctx, base);
	// 82628944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262894C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628958 size=100
    let mut pc: u32 = 0x82628958;
    'dispatch: loop {
        match pc {
            0x82628958 => {
    //   block [0x82628958..0x826289BC)
	// 82628958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262895C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628964: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262896C: 38AA361C  addi r5, r10, 0x361c
	ctx.r[5].s64 = ctx.r[10].s64 + 13852;
	// 82628970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628978: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8262897C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262898C: 386A34FC  addi r3, r10, 0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13564;
	// 82628990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628994: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628998: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826289A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826289A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826289A8: 4BE3E479  bl 0x82466e20
	ctx.lr = 0x826289AC;
	sub_82466E20(ctx, base);
	// 826289AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826289B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826289B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826289B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826289C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826289C0 size=100
    let mut pc: u32 = 0x826289C0;
    'dispatch: loop {
        match pc {
            0x826289C0 => {
    //   block [0x826289C0..0x82628A24)
	// 826289C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826289C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826289C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826289CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826289D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826289D4: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 826289D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826289DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826289E0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826289E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826289E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826289EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826289F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826289F4: 386A352C  addi r3, r10, 0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + 13612;
	// 826289F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826289FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628A00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82628A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628A08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628A10: 4BE3E411  bl 0x82466e20
	ctx.lr = 0x82628A14;
	sub_82466E20(ctx, base);
	// 82628A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628A28 size=108
    let mut pc: u32 = 0x82628A28;
    'dispatch: loop {
        match pc {
            0x82628A28 => {
    //   block [0x82628A28..0x82628A94)
	// 82628A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628A34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628A3C: 38EB7440  addi r7, r11, 0x7440
	ctx.r[7].s64 = ctx.r[11].s64 + 29760;
	// 82628A40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82628A44: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82628A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628A58: 386A355C  addi r3, r10, 0x355c
	ctx.r[3].s64 = ctx.r[10].s64 + 13660;
	// 82628A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628A80: 4BE3E3A1  bl 0x82466e20
	ctx.lr = 0x82628A84;
	sub_82466E20(ctx, base);
	// 82628A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628A98 size=112
    let mut pc: u32 = 0x82628A98;
    'dispatch: loop {
        match pc {
            0x82628A98 => {
    //   block [0x82628A98..0x82628B08)
	// 82628A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628AA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628AAC: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 82628AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628AB4: 390B7470  addi r8, r11, 0x7470
	ctx.r[8].s64 = ctx.r[11].s64 + 29808;
	// 82628AB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82628ABC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82628AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628AD0: 386A358C  addi r3, r10, 0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + 13708;
	// 82628AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628AF4: 4BE3E32D  bl 0x82466e20
	ctx.lr = 0x82628AF8;
	sub_82466E20(ctx, base);
	// 82628AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628B08 size=108
    let mut pc: u32 = 0x82628B08;
    'dispatch: loop {
        match pc {
            0x82628B08 => {
    //   block [0x82628B08..0x82628B74)
	// 82628B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628B14: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628B1C: 38EB7488  addi r7, r11, 0x7488
	ctx.r[7].s64 = ctx.r[11].s64 + 29832;
	// 82628B20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82628B24: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82628B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628B30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628B38: 386A35BC  addi r3, r10, 0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + 13756;
	// 82628B3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628B60: 4BE3E2C1  bl 0x82466e20
	ctx.lr = 0x82628B64;
	sub_82466E20(ctx, base);
	// 82628B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82628B78 size=28
    let mut pc: u32 = 0x82628B78;
    'dispatch: loop {
        match pc {
            0x82628B78 => {
    //   block [0x82628B78..0x82628B94)
	// 82628B78: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628B7C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82628B80: 394AB2C0  addi r10, r10, -0x4d40
	ctx.r[10].s64 = ctx.r[10].s64 + -19776;
	// 82628B84: 816B74A0  lwz r11, 0x74a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29856 as u32) ) } as u64;
	// 82628B88: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82628B8C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82628B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628B98 size=108
    let mut pc: u32 = 0x82628B98;
    'dispatch: loop {
        match pc {
            0x82628B98 => {
    //   block [0x82628B98..0x82628C04)
	// 82628B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628BA4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82628BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628BAC: 38EBB2C0  addi r7, r11, -0x4d40
	ctx.r[7].s64 = ctx.r[11].s64 + -19776;
	// 82628BB0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82628BB4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82628BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628BBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628BC8: 386A35EC  addi r3, r10, 0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13804;
	// 82628BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628BF0: 4BE3E231  bl 0x82466e20
	ctx.lr = 0x82628BF4;
	sub_82466E20(ctx, base);
	// 82628BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628C08 size=116
    let mut pc: u32 = 0x82628C08;
    'dispatch: loop {
        match pc {
            0x82628C08 => {
    //   block [0x82628C08..0x82628C7C)
	// 82628C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628C14: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628C18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82628C1C: 390B74A8  addi r8, r11, 0x74a8
	ctx.r[8].s64 = ctx.r[11].s64 + 29864;
	// 82628C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628C24: 392A357C  addi r9, r10, 0x357c
	ctx.r[9].s64 = ctx.r[10].s64 + 13692;
	// 82628C28: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628C2C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82628C30: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 82628C34: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628C3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628C4C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82628C50: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82628C54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82628C58: 386B361C  addi r3, r11, 0x361c
	ctx.r[3].s64 = ctx.r[11].s64 + 13852;
	// 82628C5C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82628C60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628C68: 4BE3E1B9  bl 0x82466e20
	ctx.lr = 0x82628C6C;
	sub_82466E20(ctx, base);
	// 82628C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628C80 size=112
    let mut pc: u32 = 0x82628C80;
    'dispatch: loop {
        match pc {
            0x82628C80 => {
    //   block [0x82628C80..0x82628CF0)
	// 82628C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628C8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628C90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628C94: 38AA322C  addi r5, r10, 0x322c
	ctx.r[5].s64 = ctx.r[10].s64 + 12844;
	// 82628C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628C9C: 390B7520  addi r8, r11, 0x7520
	ctx.r[8].s64 = ctx.r[11].s64 + 29984;
	// 82628CA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82628CA4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82628CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628CAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628CB8: 386A364C  addi r3, r10, 0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + 13900;
	// 82628CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628CDC: 4BE3E145  bl 0x82466e20
	ctx.lr = 0x82628CE0;
	sub_82466E20(ctx, base);
	// 82628CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628CF0 size=108
    let mut pc: u32 = 0x82628CF0;
    'dispatch: loop {
        match pc {
            0x82628CF0 => {
    //   block [0x82628CF0..0x82628D5C)
	// 82628CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628CFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628D04: 38EB7538  addi r7, r11, 0x7538
	ctx.r[7].s64 = ctx.r[11].s64 + 30008;
	// 82628D08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82628D0C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82628D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628D14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628D20: 386A367C  addi r3, r10, 0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + 13948;
	// 82628D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628D48: 4BE3E0D9  bl 0x82466e20
	ctx.lr = 0x82628D4C;
	sub_82466E20(ctx, base);
	// 82628D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628D60 size=112
    let mut pc: u32 = 0x82628D60;
    'dispatch: loop {
        match pc {
            0x82628D60 => {
    //   block [0x82628D60..0x82628DD0)
	// 82628D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628D6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628D70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628D74: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628D7C: 390B7568  addi r8, r11, 0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + 30056;
	// 82628D80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82628D84: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82628D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628D98: 386A36AC  addi r3, r10, 0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13996;
	// 82628D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628DBC: 4BE3E065  bl 0x82466e20
	ctx.lr = 0x82628DC0;
	sub_82466E20(ctx, base);
	// 82628DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628DD0 size=112
    let mut pc: u32 = 0x82628DD0;
    'dispatch: loop {
        match pc {
            0x82628DD0 => {
    //   block [0x82628DD0..0x82628E40)
	// 82628DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628DDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628DE0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628DE4: 38AA382C  addi r5, r10, 0x382c
	ctx.r[5].s64 = ctx.r[10].s64 + 14380;
	// 82628DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628DEC: 390B7598  addi r8, r11, 0x7598
	ctx.r[8].s64 = ctx.r[11].s64 + 30104;
	// 82628DF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82628DF4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82628DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628DFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628E08: 386A36DC  addi r3, r10, 0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14044;
	// 82628E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628E2C: 4BE3DFF5  bl 0x82466e20
	ctx.lr = 0x82628E30;
	sub_82466E20(ctx, base);
	// 82628E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628E40 size=100
    let mut pc: u32 = 0x82628E40;
    'dispatch: loop {
        match pc {
            0x82628E40 => {
    //   block [0x82628E40..0x82628EA4)
	// 82628E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628E4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628E54: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628E60: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 82628E64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628E74: 386A370C  addi r3, r10, 0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + 14092;
	// 82628E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628E80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82628E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628E90: 4BE3DF91  bl 0x82466e20
	ctx.lr = 0x82628E94;
	sub_82466E20(ctx, base);
	// 82628E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628EA8 size=112
    let mut pc: u32 = 0x82628EA8;
    'dispatch: loop {
        match pc {
            0x82628EA8 => {
    //   block [0x82628EA8..0x82628F18)
	// 82628EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628EB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628EB8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628EBC: 38AA352C  addi r5, r10, 0x352c
	ctx.r[5].s64 = ctx.r[10].s64 + 13612;
	// 82628EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628EC4: 390B75C8  addi r8, r11, 0x75c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30152;
	// 82628EC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82628ECC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82628ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628ED4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628EE0: 386A373C  addi r3, r10, 0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + 14140;
	// 82628EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628F04: 4BE3DF1D  bl 0x82466e20
	ctx.lr = 0x82628F08;
	sub_82466E20(ctx, base);
	// 82628F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628F18 size=112
    let mut pc: u32 = 0x82628F18;
    'dispatch: loop {
        match pc {
            0x82628F18 => {
    //   block [0x82628F18..0x82628F88)
	// 82628F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628F24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628F28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628F2C: 38AA352C  addi r5, r10, 0x352c
	ctx.r[5].s64 = ctx.r[10].s64 + 13612;
	// 82628F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628F34: 390B7610  addi r8, r11, 0x7610
	ctx.r[8].s64 = ctx.r[11].s64 + 30224;
	// 82628F38: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82628F3C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82628F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628F44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628F50: 386A376C  addi r3, r10, 0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + 14188;
	// 82628F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628F74: 4BE3DEAD  bl 0x82466e20
	ctx.lr = 0x82628F78;
	sub_82466E20(ctx, base);
	// 82628F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628F88 size=108
    let mut pc: u32 = 0x82628F88;
    'dispatch: loop {
        match pc {
            0x82628F88 => {
    //   block [0x82628F88..0x82628FF4)
	// 82628F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628F94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628F9C: 38EB76B8  addi r7, r11, 0x76b8
	ctx.r[7].s64 = ctx.r[11].s64 + 30392;
	// 82628FA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82628FA4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82628FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628FAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628FB8: 386A379C  addi r3, r10, 0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + 14236;
	// 82628FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628FE0: 4BE3DE41  bl 0x82466e20
	ctx.lr = 0x82628FE4;
	sub_82466E20(ctx, base);
	// 82628FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628FF8 size=112
    let mut pc: u32 = 0x82628FF8;
    'dispatch: loop {
        match pc {
            0x82628FF8 => {
    //   block [0x82628FF8..0x82629068)
	// 82628FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629008: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262900C: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 82629010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629014: 390B7700  addi r8, r11, 0x7700
	ctx.r[8].s64 = ctx.r[11].s64 + 30464;
	// 82629018: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262901C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82629020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629024: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262902C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629030: 386A37CC  addi r3, r10, 0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14284;
	// 82629034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262903C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262904C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629054: 4BE3DDCD  bl 0x82466e20
	ctx.lr = 0x82629058;
	sub_82466E20(ctx, base);
	// 82629058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262905C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629068 size=100
    let mut pc: u32 = 0x82629068;
    'dispatch: loop {
        match pc {
            0x82629068 => {
    //   block [0x82629068..0x826290CC)
	// 82629068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262907C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82629080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629088: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8262908C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262909C: 386A37FC  addi r3, r10, 0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14332;
	// 826290A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826290A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826290A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826290AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826290B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826290B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826290B8: 4BE3DD69  bl 0x82466e20
	ctx.lr = 0x826290BC;
	sub_82466E20(ctx, base);
	// 826290BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826290C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826290C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826290C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826290D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826290D0 size=100
    let mut pc: u32 = 0x826290D0;
    'dispatch: loop {
        match pc {
            0x826290D0 => {
    //   block [0x826290D0..0x82629134)
	// 826290D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826290D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826290D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826290DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826290E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826290E4: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 826290E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826290EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826290F0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826290F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826290F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826290FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629104: 386A382C  addi r3, r10, 0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + 14380;
	// 82629108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262910C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629110: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82629114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629118: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262911C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629120: 4BE3DD01  bl 0x82466e20
	ctx.lr = 0x82629124;
	sub_82466E20(ctx, base);
	// 82629124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262912C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629138 size=112
    let mut pc: u32 = 0x82629138;
    'dispatch: loop {
        match pc {
            0x82629138 => {
    //   block [0x82629138..0x826291A8)
	// 82629138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262913C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629148: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262914C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82629150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629154: 390B7760  addi r8, r11, 0x7760
	ctx.r[8].s64 = ctx.r[11].s64 + 30560;
	// 82629158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262915C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82629160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262916C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629170: 386A385C  addi r3, r10, 0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + 14428;
	// 82629174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262917C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262918C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629194: 4BE3DC8D  bl 0x82466e20
	ctx.lr = 0x82629198;
	sub_82466E20(ctx, base);
	// 82629198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262919C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826291A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826291A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826291A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826291A8 size=112
    let mut pc: u32 = 0x826291A8;
    'dispatch: loop {
        match pc {
            0x826291A8 => {
    //   block [0x826291A8..0x82629218)
	// 826291A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826291AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826291B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826291B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826291B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826291BC: 38AA361C  addi r5, r10, 0x361c
	ctx.r[5].s64 = ctx.r[10].s64 + 13852;
	// 826291C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826291C4: 390B77F0  addi r8, r11, 0x77f0
	ctx.r[8].s64 = ctx.r[11].s64 + 30704;
	// 826291C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826291CC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826291D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826291D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826291D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826291DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826291E0: 386A388C  addi r3, r10, 0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + 14476;
	// 826291E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826291E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826291EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826291F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826291F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826291F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826291FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629204: 4BE3DC1D  bl 0x82466e20
	ctx.lr = 0x82629208;
	sub_82466E20(ctx, base);
	// 82629208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262920C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629218 size=112
    let mut pc: u32 = 0x82629218;
    'dispatch: loop {
        match pc {
            0x82629218 => {
    //   block [0x82629218..0x82629288)
	// 82629218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262921C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629228: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262922C: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 82629230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629234: 390B7808  addi r8, r11, 0x7808
	ctx.r[8].s64 = ctx.r[11].s64 + 30728;
	// 82629238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262923C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82629240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262924C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629250: 386A38BC  addi r3, r10, 0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14524;
	// 82629254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262925C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262926C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629274: 4BE3DBAD  bl 0x82466e20
	ctx.lr = 0x82629278;
	sub_82466E20(ctx, base);
	// 82629278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262927C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629288 size=112
    let mut pc: u32 = 0x82629288;
    'dispatch: loop {
        match pc {
            0x82629288 => {
    //   block [0x82629288..0x826292F8)
	// 82629288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262928C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629298: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262929C: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 826292A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826292A4: 390B7838  addi r8, r11, 0x7838
	ctx.r[8].s64 = ctx.r[11].s64 + 30776;
	// 826292A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826292AC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826292B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826292B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826292B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826292BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826292C0: 386A38EC  addi r3, r10, 0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14572;
	// 826292C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826292C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826292CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826292D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826292D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826292D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826292DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826292E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826292E4: 4BE3DB3D  bl 0x82466e20
	ctx.lr = 0x826292E8;
	sub_82466E20(ctx, base);
	// 826292E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826292EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826292F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826292F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826292F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826292F8 size=112
    let mut pc: u32 = 0x826292F8;
    'dispatch: loop {
        match pc {
            0x826292F8 => {
    //   block [0x826292F8..0x82629368)
	// 826292F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826292FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629308: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262930C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82629310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629314: 390B7880  addi r8, r11, 0x7880
	ctx.r[8].s64 = ctx.r[11].s64 + 30848;
	// 82629318: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262931C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82629320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262932C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629330: 386A391C  addi r3, r10, 0x391c
	ctx.r[3].s64 = ctx.r[10].s64 + 14620;
	// 82629334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262933C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262934C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629354: 4BE3DACD  bl 0x82466e20
	ctx.lr = 0x82629358;
	sub_82466E20(ctx, base);
	// 82629358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262935C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629368 size=112
    let mut pc: u32 = 0x82629368;
    'dispatch: loop {
        match pc {
            0x82629368 => {
    //   block [0x82629368..0x826293D8)
	// 82629368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262936C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629378: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262937C: 38AA322C  addi r5, r10, 0x322c
	ctx.r[5].s64 = ctx.r[10].s64 + 12844;
	// 82629380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629384: 390B78C8  addi r8, r11, 0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30920;
	// 82629388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262938C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82629390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262939C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826293A0: 386A394C  addi r3, r10, 0x394c
	ctx.r[3].s64 = ctx.r[10].s64 + 14668;
	// 826293A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826293A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826293AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826293B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826293B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826293B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826293BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826293C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826293C4: 4BE3DA5D  bl 0x82466e20
	ctx.lr = 0x826293C8;
	sub_82466E20(ctx, base);
	// 826293C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826293CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826293D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826293D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826293D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826293D8 size=112
    let mut pc: u32 = 0x826293D8;
    'dispatch: loop {
        match pc {
            0x826293D8 => {
    //   block [0x826293D8..0x82629448)
	// 826293D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826293DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826293E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826293E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826293E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826293EC: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 826293F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826293F4: 390B78E0  addi r8, r11, 0x78e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30944;
	// 826293F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826293FC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 82629400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262940C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629410: 386A397C  addi r3, r10, 0x397c
	ctx.r[3].s64 = ctx.r[10].s64 + 14716;
	// 82629414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262941C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262942C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629434: 4BE3D9ED  bl 0x82466e20
	ctx.lr = 0x82629438;
	sub_82466E20(ctx, base);
	// 82629438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262943C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82629448 size=24
    let mut pc: u32 = 0x82629448;
    'dispatch: loop {
        match pc {
            0x82629448 => {
    //   block [0x82629448..0x82629460)
	// 82629448: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262944C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82629450: 394AB3F8  addi r10, r10, -0x4c08
	ctx.r[10].s64 = ctx.r[10].s64 + -19464;
	// 82629454: 816B7910  lwz r11, 0x7910(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30992 as u32) ) } as u64;
	// 82629458: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262945C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629460 size=112
    let mut pc: u32 = 0x82629460;
    'dispatch: loop {
        match pc {
            0x82629460 => {
    //   block [0x82629460..0x826294D0)
	// 82629460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262946C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82629470: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82629474: 392A36A0  addi r9, r10, 0x36a0
	ctx.r[9].s64 = ctx.r[10].s64 + 13984;
	// 82629478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262947C: 390BB3F8  addi r8, r11, -0x4c08
	ctx.r[8].s64 = ctx.r[11].s64 + -19464;
	// 82629480: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82629484: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 82629488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262948C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629498: 386A39AC  addi r3, r10, 0x39ac
	ctx.r[3].s64 = ctx.r[10].s64 + 14764;
	// 8262949C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826294A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826294A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826294A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826294AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826294B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826294B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826294B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826294BC: 4BE3D965  bl 0x82466e20
	ctx.lr = 0x826294C0;
	sub_82466E20(ctx, base);
	// 826294C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826294C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826294C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826294CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826294D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826294D0 size=112
    let mut pc: u32 = 0x826294D0;
    'dispatch: loop {
        match pc {
            0x826294D0 => {
    //   block [0x826294D0..0x82629540)
	// 826294D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826294D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826294D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826294DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826294E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826294E4: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826294E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826294EC: 390B7918  addi r8, r11, 0x7918
	ctx.r[8].s64 = ctx.r[11].s64 + 31000;
	// 826294F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826294F4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826294F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826294FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629508: 386A39DC  addi r3, r10, 0x39dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14812;
	// 8262950C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262951C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262952C: 4BE3D8F5  bl 0x82466e20
	ctx.lr = 0x82629530;
	sub_82466E20(ctx, base);
	// 82629530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262953C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629540 size=108
    let mut pc: u32 = 0x82629540;
    'dispatch: loop {
        match pc {
            0x82629540 => {
    //   block [0x82629540..0x826295AC)
	// 82629540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262954C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629554: 38EB7948  addi r7, r11, 0x7948
	ctx.r[7].s64 = ctx.r[11].s64 + 31048;
	// 82629558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262955C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 82629560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262956C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629570: 386A3A0C  addi r3, r10, 0x3a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 14860;
	// 82629574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82629578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262957C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262958C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82629598: 4BE3D889  bl 0x82466e20
	ctx.lr = 0x8262959C;
	sub_82466E20(ctx, base);
	// 8262959C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826295A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826295A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826295A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826295B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826295B0 size=100
    let mut pc: u32 = 0x826295B0;
    'dispatch: loop {
        match pc {
            0x826295B0 => {
    //   block [0x826295B0..0x82629614)
	// 826295B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826295B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826295B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826295BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826295C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826295C4: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826295C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826295CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826295D0: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826295D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826295D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826295DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826295E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826295E4: 386A3A3C  addi r3, r10, 0x3a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 14908;
	// 826295E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826295EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826295F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826295F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826295F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826295FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629600: 4BE3D821  bl 0x82466e20
	ctx.lr = 0x82629604;
	sub_82466E20(ctx, base);
	// 82629604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262960C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629618 size=112
    let mut pc: u32 = 0x82629618;
    'dispatch: loop {
        match pc {
            0x82629618 => {
    //   block [0x82629618..0x82629688)
	// 82629618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629624: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629628: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262962C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629634: 390B7960  addi r8, r11, 0x7960
	ctx.r[8].s64 = ctx.r[11].s64 + 31072;
	// 82629638: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262963C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 82629640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262964C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629650: 386A3A6C  addi r3, r10, 0x3a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 14956;
	// 82629654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262965C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262966C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629674: 4BE3D7AD  bl 0x82466e20
	ctx.lr = 0x82629678;
	sub_82466E20(ctx, base);
	// 82629678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629688 size=112
    let mut pc: u32 = 0x82629688;
    'dispatch: loop {
        match pc {
            0x82629688 => {
    //   block [0x82629688..0x826296F8)
	// 82629688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629698: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262969C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826296A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826296A4: 390B7978  addi r8, r11, 0x7978
	ctx.r[8].s64 = ctx.r[11].s64 + 31096;
	// 826296A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826296AC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826296B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826296B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826296B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826296BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826296C0: 386A3A9C  addi r3, r10, 0x3a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15004;
	// 826296C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826296C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826296CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826296D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826296D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826296D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826296DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826296E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826296E4: 4BE3D73D  bl 0x82466e20
	ctx.lr = 0x826296E8;
	sub_82466E20(ctx, base);
	// 826296E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826296EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826296F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826296F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826296F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826296F8 size=112
    let mut pc: u32 = 0x826296F8;
    'dispatch: loop {
        match pc {
            0x826296F8 => {
    //   block [0x826296F8..0x82629768)
	// 826296F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826296FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629704: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629708: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262970C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629714: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 82629718: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262971C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82629720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262972C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629730: 386A3ACC  addi r3, r10, 0x3acc
	ctx.r[3].s64 = ctx.r[10].s64 + 15052;
	// 82629734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262973C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262974C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629754: 4BE3D6CD  bl 0x82466e20
	ctx.lr = 0x82629758;
	sub_82466E20(ctx, base);
	// 82629758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262975C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629768 size=112
    let mut pc: u32 = 0x82629768;
    'dispatch: loop {
        match pc {
            0x82629768 => {
    //   block [0x82629768..0x826297D8)
	// 82629768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262976C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629774: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629778: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262977C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629784: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82629788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262978C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 82629790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262979C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826297A0: 386A3AFC  addi r3, r10, 0x3afc
	ctx.r[3].s64 = ctx.r[10].s64 + 15100;
	// 826297A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826297A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826297AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826297B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826297B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826297B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826297BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826297C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826297C4: 4BE3D65D  bl 0x82466e20
	ctx.lr = 0x826297C8;
	sub_82466E20(ctx, base);
	// 826297C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826297CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826297D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826297D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826297D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826297D8 size=112
    let mut pc: u32 = 0x826297D8;
    'dispatch: loop {
        match pc {
            0x826297D8 => {
    //   block [0x826297D8..0x82629848)
	// 826297D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826297DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826297E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826297E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826297E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826297EC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826297F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826297F4: 390B7A08  addi r8, r11, 0x7a08
	ctx.r[8].s64 = ctx.r[11].s64 + 31240;
	// 826297F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826297FC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82629800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629804: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262980C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629810: 386A3B2C  addi r3, r10, 0x3b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15148;
	// 82629814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262981C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262982C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629834: 4BE3D5ED  bl 0x82466e20
	ctx.lr = 0x82629838;
	sub_82466E20(ctx, base);
	// 82629838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262983C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629848 size=112
    let mut pc: u32 = 0x82629848;
    'dispatch: loop {
        match pc {
            0x82629848 => {
    //   block [0x82629848..0x826298B8)
	// 82629848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262984C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629858: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262985C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629864: 390B7A20  addi r8, r11, 0x7a20
	ctx.r[8].s64 = ctx.r[11].s64 + 31264;
	// 82629868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262986C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82629870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629874: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262987C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629880: 386A3B5C  addi r3, r10, 0x3b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15196;
	// 82629884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262988C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262989C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826298A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826298A4: 4BE3D57D  bl 0x82466e20
	ctx.lr = 0x826298A8;
	sub_82466E20(ctx, base);
	// 826298A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826298AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826298B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826298B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826298B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826298B8 size=112
    let mut pc: u32 = 0x826298B8;
    'dispatch: loop {
        match pc {
            0x826298B8 => {
    //   block [0x826298B8..0x82629928)
	// 826298B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826298BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826298C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826298C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826298C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826298CC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826298D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826298D4: 390B7A38  addi r8, r11, 0x7a38
	ctx.r[8].s64 = ctx.r[11].s64 + 31288;
	// 826298D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826298DC: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826298E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826298E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826298E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826298EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826298F0: 386A3B8C  addi r3, r10, 0x3b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15244;
	// 826298F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826298F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826298FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262990C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629914: 4BE3D50D  bl 0x82466e20
	ctx.lr = 0x82629918;
	sub_82466E20(ctx, base);
	// 82629918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262991C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629928 size=112
    let mut pc: u32 = 0x82629928;
    'dispatch: loop {
        match pc {
            0x82629928 => {
    //   block [0x82629928..0x82629998)
	// 82629928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262992C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629938: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262993C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629944: 390B7A80  addi r8, r11, 0x7a80
	ctx.r[8].s64 = ctx.r[11].s64 + 31360;
	// 82629948: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262994C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82629950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262995C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629960: 386A3BBC  addi r3, r10, 0x3bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15292;
	// 82629964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262996C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262997C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629984: 4BE3D49D  bl 0x82466e20
	ctx.lr = 0x82629988;
	sub_82466E20(ctx, base);
	// 82629988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262998C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629998 size=112
    let mut pc: u32 = 0x82629998;
    'dispatch: loop {
        match pc {
            0x82629998 => {
    //   block [0x82629998..0x82629A08)
	// 82629998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826299A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826299A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826299A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826299AC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826299B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826299B4: 390B7AC8  addi r8, r11, 0x7ac8
	ctx.r[8].s64 = ctx.r[11].s64 + 31432;
	// 826299B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826299BC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826299C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826299C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826299C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826299CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826299D0: 386A3BEC  addi r3, r10, 0x3bec
	ctx.r[3].s64 = ctx.r[10].s64 + 15340;
	// 826299D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826299D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826299DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826299E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826299E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826299E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826299EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826299F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826299F4: 4BE3D42D  bl 0x82466e20
	ctx.lr = 0x826299F8;
	sub_82466E20(ctx, base);
	// 826299F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826299FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629A08 size=112
    let mut pc: u32 = 0x82629A08;
    'dispatch: loop {
        match pc {
            0x82629A08 => {
    //   block [0x82629A08..0x82629A78)
	// 82629A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629A14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629A18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629A1C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629A24: 390B7AE0  addi r8, r11, 0x7ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 31456;
	// 82629A28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82629A2C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82629A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629A34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629A40: 386A3C1C  addi r3, r10, 0x3c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15388;
	// 82629A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629A64: 4BE3D3BD  bl 0x82466e20
	ctx.lr = 0x82629A68;
	sub_82466E20(ctx, base);
	// 82629A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629A78 size=116
    let mut pc: u32 = 0x82629A78;
    'dispatch: loop {
        match pc {
            0x82629A78 => {
    //   block [0x82629A78..0x82629AEC)
	// 82629A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629A84: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82629A88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82629A8C: 390A7B10  addi r8, r10, 0x7b10
	ctx.r[8].s64 = ctx.r[10].s64 + 31504;
	// 82629A90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629A94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82629A98: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629A9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629AA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82629AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629AAC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 82629AB0: 396B36C8  addi r11, r11, 0x36c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14024;
	// 82629AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629AB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629ABC: 386A3C4C  addi r3, r10, 0x3c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 15436;
	// 82629AC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82629AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629AC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82629ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629AD8: 4BE3D349  bl 0x82466e20
	ctx.lr = 0x82629ADC;
	sub_82466E20(ctx, base);
	// 82629ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629AF0 size=116
    let mut pc: u32 = 0x82629AF0;
    'dispatch: loop {
        match pc {
            0x82629AF0 => {
    //   block [0x82629AF0..0x82629B64)
	// 82629AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629AFC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82629B00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82629B04: 390A7B88  addi r8, r10, 0x7b88
	ctx.r[8].s64 = ctx.r[10].s64 + 31624;
	// 82629B08: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629B0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82629B10: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629B14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629B18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82629B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629B24: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 82629B28: 396B36E0  addi r11, r11, 0x36e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14048;
	// 82629B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629B30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629B34: 386A3C7C  addi r3, r10, 0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15484;
	// 82629B38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82629B3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629B40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82629B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629B50: 4BE3D2D1  bl 0x82466e20
	ctx.lr = 0x82629B54;
	sub_82466E20(ctx, base);
	// 82629B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82629B68 size=24
    let mut pc: u32 = 0x82629B68;
    'dispatch: loop {
        match pc {
            0x82629B68 => {
    //   block [0x82629B68..0x82629B80)
	// 82629B68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629B6C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82629B70: 394AB410  addi r10, r10, -0x4bf0
	ctx.r[10].s64 = ctx.r[10].s64 + -19440;
	// 82629B74: 816B7C18  lwz r11, 0x7c18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31768 as u32) ) } as u64;
	// 82629B78: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82629B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629B80 size=116
    let mut pc: u32 = 0x82629B80;
    'dispatch: loop {
        match pc {
            0x82629B80 => {
    //   block [0x82629B80..0x82629BF4)
	// 82629B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629B8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82629B90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629B94: 392B370C  addi r9, r11, 0x370c
	ctx.r[9].s64 = ctx.r[11].s64 + 14092;
	// 82629B98: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629B9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629BA0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82629BA4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82629BA8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82629BAC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 82629BB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629BB4: 396BB410  addi r11, r11, -0x4bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -19440;
	// 82629BB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82629BBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629BC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82629BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629BC8: 386A3CAC  addi r3, r10, 0x3cac
	ctx.r[3].s64 = ctx.r[10].s64 + 15532;
	// 82629BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82629BD0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82629BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629BD8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82629BDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82629BE0: 4BE3D241  bl 0x82466e20
	ctx.lr = 0x82629BE4;
	sub_82466E20(ctx, base);
	// 82629BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629BF8 size=112
    let mut pc: u32 = 0x82629BF8;
    'dispatch: loop {
        match pc {
            0x82629BF8 => {
    //   block [0x82629BF8..0x82629C68)
	// 82629BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629C04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629C0C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629C14: 390B7C20  addi r8, r11, 0x7c20
	ctx.r[8].s64 = ctx.r[11].s64 + 31776;
	// 82629C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82629C1C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 82629C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629C30: 386A3CDC  addi r3, r10, 0x3cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 15580;
	// 82629C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629C54: 4BE3D1CD  bl 0x82466e20
	ctx.lr = 0x82629C58;
	sub_82466E20(ctx, base);
	// 82629C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629C68 size=112
    let mut pc: u32 = 0x82629C68;
    'dispatch: loop {
        match pc {
            0x82629C68 => {
    //   block [0x82629C68..0x82629CD8)
	// 82629C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629C74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C78: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629C7C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629C84: 390B7C80  addi r8, r11, 0x7c80
	ctx.r[8].s64 = ctx.r[11].s64 + 31872;
	// 82629C88: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82629C8C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 82629C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629C94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629CA0: 386A3D0C  addi r3, r10, 0x3d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15628;
	// 82629CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629CC4: 4BE3D15D  bl 0x82466e20
	ctx.lr = 0x82629CC8;
	sub_82466E20(ctx, base);
	// 82629CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629CD8 size=112
    let mut pc: u32 = 0x82629CD8;
    'dispatch: loop {
        match pc {
            0x82629CD8 => {
    //   block [0x82629CD8..0x82629D48)
	// 82629CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629CE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629CE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629CEC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629CF4: 390B7D28  addi r8, r11, 0x7d28
	ctx.r[8].s64 = ctx.r[11].s64 + 32040;
	// 82629CF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82629CFC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 82629D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629D04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629D10: 386A3D3C  addi r3, r10, 0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15676;
	// 82629D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629D34: 4BE3D0ED  bl 0x82466e20
	ctx.lr = 0x82629D38;
	sub_82466E20(ctx, base);
	// 82629D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629D48 size=112
    let mut pc: u32 = 0x82629D48;
    'dispatch: loop {
        match pc {
            0x82629D48 => {
    //   block [0x82629D48..0x82629DB8)
	// 82629D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629D54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629D58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629D5C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629D64: 390B7DA0  addi r8, r11, 0x7da0
	ctx.r[8].s64 = ctx.r[11].s64 + 32160;
	// 82629D68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82629D6C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82629D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629D80: 386A3D6C  addi r3, r10, 0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15724;
	// 82629D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629DA4: 4BE3D07D  bl 0x82466e20
	ctx.lr = 0x82629DA8;
	sub_82466E20(ctx, base);
	// 82629DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629DB8 size=112
    let mut pc: u32 = 0x82629DB8;
    'dispatch: loop {
        match pc {
            0x82629DB8 => {
    //   block [0x82629DB8..0x82629E28)
	// 82629DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629DC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629DC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629DCC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629DD4: 390B7DE8  addi r8, r11, 0x7de8
	ctx.r[8].s64 = ctx.r[11].s64 + 32232;
	// 82629DD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82629DDC: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 82629DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629DF0: 386A3D9C  addi r3, r10, 0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15772;
	// 82629DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629E14: 4BE3D00D  bl 0x82466e20
	ctx.lr = 0x82629E18;
	sub_82466E20(ctx, base);
	// 82629E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629E28 size=112
    let mut pc: u32 = 0x82629E28;
    'dispatch: loop {
        match pc {
            0x82629E28 => {
    //   block [0x82629E28..0x82629E98)
	// 82629E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629E34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629E38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629E3C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629E44: 390B7E78  addi r8, r11, 0x7e78
	ctx.r[8].s64 = ctx.r[11].s64 + 32376;
	// 82629E48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82629E4C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82629E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629E60: 386A3DCC  addi r3, r10, 0x3dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15820;
	// 82629E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629E84: 4BE3CF9D  bl 0x82466e20
	ctx.lr = 0x82629E88;
	sub_82466E20(ctx, base);
	// 82629E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629E98 size=112
    let mut pc: u32 = 0x82629E98;
    'dispatch: loop {
        match pc {
            0x82629E98 => {
    //   block [0x82629E98..0x82629F08)
	// 82629E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629EA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629EAC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629EB4: 390B7ED8  addi r8, r11, 0x7ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 32472;
	// 82629EB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82629EBC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 82629EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629EC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629ED0: 386A3DFC  addi r3, r10, 0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15868;
	// 82629ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629EF4: 4BE3CF2D  bl 0x82466e20
	ctx.lr = 0x82629EF8;
	sub_82466E20(ctx, base);
	// 82629EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629F08 size=112
    let mut pc: u32 = 0x82629F08;
    'dispatch: loop {
        match pc {
            0x82629F08 => {
    //   block [0x82629F08..0x82629F78)
	// 82629F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629F18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629F1C: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 82629F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629F24: 390B7F20  addi r8, r11, 0x7f20
	ctx.r[8].s64 = ctx.r[11].s64 + 32544;
	// 82629F28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82629F2C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 82629F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629F34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629F40: 386A3E2C  addi r3, r10, 0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15916;
	// 82629F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629F64: 4BE3CEBD  bl 0x82466e20
	ctx.lr = 0x82629F68;
	sub_82466E20(ctx, base);
	// 82629F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629F78 size=112
    let mut pc: u32 = 0x82629F78;
    'dispatch: loop {
        match pc {
            0x82629F78 => {
    //   block [0x82629F78..0x82629FE8)
	// 82629F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629F88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629F8C: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 82629F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629F94: 390B7F50  addi r8, r11, 0x7f50
	ctx.r[8].s64 = ctx.r[11].s64 + 32592;
	// 82629F98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82629F9C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 82629FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629FB0: 386A3E5C  addi r3, r10, 0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15964;
	// 82629FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629FD4: 4BE3CE4D  bl 0x82466e20
	ctx.lr = 0x82629FD8;
	sub_82466E20(ctx, base);
	// 82629FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629FE8 size=100
    let mut pc: u32 = 0x82629FE8;
    'dispatch: loop {
        match pc {
            0x82629FE8 => {
    //   block [0x82629FE8..0x8262A04C)
	// 82629FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629FFC: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 8262A000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A008: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8262A00C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A01C: 386A3E8C  addi r3, r10, 0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16012;
	// 8262A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262A02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262A034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A038: 4BE3CDE9  bl 0x82466e20
	ctx.lr = 0x8262A03C;
	sub_82466E20(ctx, base);
	// 8262A03C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A050 size=112
    let mut pc: u32 = 0x8262A050;
    'dispatch: loop {
        match pc {
            0x8262A050 => {
    //   block [0x8262A050..0x8262A0C0)
	// 8262A050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A05C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A060: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A064: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 8262A068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A06C: 390B7F80  addi r8, r11, 0x7f80
	ctx.r[8].s64 = ctx.r[11].s64 + 32640;
	// 8262A070: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A074: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8262A078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A07C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A088: 386A3EBC  addi r3, r10, 0x3ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 16060;
	// 8262A08C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A0AC: 4BE3CD75  bl 0x82466e20
	ctx.lr = 0x8262A0B0;
	sub_82466E20(ctx, base);
	// 8262A0B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A0C0 size=112
    let mut pc: u32 = 0x8262A0C0;
    'dispatch: loop {
        match pc {
            0x8262A0C0 => {
    //   block [0x8262A0C0..0x8262A130)
	// 8262A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A0C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A0CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A0D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A0D4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A0D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A0DC: 390B7F98  addi r8, r11, 0x7f98
	ctx.r[8].s64 = ctx.r[11].s64 + 32664;
	// 8262A0E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262A0E4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8262A0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A0EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A0F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A0F8: 386A3EEC  addi r3, r10, 0x3eec
	ctx.r[3].s64 = ctx.r[10].s64 + 16108;
	// 8262A0FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A11C: 4BE3CD05  bl 0x82466e20
	ctx.lr = 0x8262A120;
	sub_82466E20(ctx, base);
	// 8262A120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A130 size=112
    let mut pc: u32 = 0x8262A130;
    'dispatch: loop {
        match pc {
            0x8262A130 => {
    //   block [0x8262A130..0x8262A1A0)
	// 8262A130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A13C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A140: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A144: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262A148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A14C: 390B7FF8  addi r8, r11, 0x7ff8
	ctx.r[8].s64 = ctx.r[11].s64 + 32760;
	// 8262A150: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A154: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8262A158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A15C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A168: 386A3F1C  addi r3, r10, 0x3f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 16156;
	// 8262A16C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A18C: 4BE3CC95  bl 0x82466e20
	ctx.lr = 0x8262A190;
	sub_82466E20(ctx, base);
	// 8262A190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A1A0 size=112
    let mut pc: u32 = 0x8262A1A0;
    'dispatch: loop {
        match pc {
            0x8262A1A0 => {
    //   block [0x8262A1A0..0x8262A210)
	// 8262A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A1AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A1B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A1B4: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262A1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A1BC: 390B8010  addi r8, r11, -0x7ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -32752;
	// 8262A1C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262A1C4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8262A1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A1CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A1D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A1D8: 386A3F4C  addi r3, r10, 0x3f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 16204;
	// 8262A1DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A1FC: 4BE3CC25  bl 0x82466e20
	ctx.lr = 0x8262A200;
	sub_82466E20(ctx, base);
	// 8262A200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A210 size=112
    let mut pc: u32 = 0x8262A210;
    'dispatch: loop {
        match pc {
            0x8262A210 => {
    //   block [0x8262A210..0x8262A280)
	// 8262A210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A21C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A220: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A224: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262A228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A22C: 390B8040  addi r8, r11, -0x7fc0
	ctx.r[8].s64 = ctx.r[11].s64 + -32704;
	// 8262A230: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A234: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8262A238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A248: 386A3F7C  addi r3, r10, 0x3f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 16252;
	// 8262A24C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A26C: 4BE3CBB5  bl 0x82466e20
	ctx.lr = 0x8262A270;
	sub_82466E20(ctx, base);
	// 8262A270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262A280 size=24
    let mut pc: u32 = 0x8262A280;
    'dispatch: loop {
        match pc {
            0x8262A280 => {
    //   block [0x8262A280..0x8262A298)
	// 8262A280: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A284: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262A288: 394AB4B8  addi r10, r10, -0x4b48
	ctx.r[10].s64 = ctx.r[10].s64 + -19272;
	// 8262A28C: 816B7C1C  lwz r11, 0x7c1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31772 as u32) ) } as u64;
	// 8262A290: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262A294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A298 size=112
    let mut pc: u32 = 0x8262A298;
    'dispatch: loop {
        match pc {
            0x8262A298 => {
    //   block [0x8262A298..0x8262A308)
	// 8262A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A2A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A2A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A2AC: 392A3768  addi r9, r10, 0x3768
	ctx.r[9].s64 = ctx.r[10].s64 + 14184;
	// 8262A2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A2B4: 390BB4B8  addi r8, r11, -0x4b48
	ctx.r[8].s64 = ctx.r[11].s64 + -19272;
	// 8262A2B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262A2BC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8262A2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A2C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A2D0: 386A3FAC  addi r3, r10, 0x3fac
	ctx.r[3].s64 = ctx.r[10].s64 + 16300;
	// 8262A2D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A2D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A2F4: 4BE3CB2D  bl 0x82466e20
	ctx.lr = 0x8262A2F8;
	sub_82466E20(ctx, base);
	// 8262A2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A308 size=108
    let mut pc: u32 = 0x8262A308;
    'dispatch: loop {
        match pc {
            0x8262A308 => {
    //   block [0x8262A308..0x8262A374)
	// 8262A308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A314: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A31C: 38EB8058  addi r7, r11, -0x7fa8
	ctx.r[7].s64 = ctx.r[11].s64 + -32680;
	// 8262A320: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262A324: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8262A328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A32C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A338: 386A3FDC  addi r3, r10, 0x3fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 16348;
	// 8262A33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A360: 4BE3CAC1  bl 0x82466e20
	ctx.lr = 0x8262A364;
	sub_82466E20(ctx, base);
	// 8262A364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A378 size=108
    let mut pc: u32 = 0x8262A378;
    'dispatch: loop {
        match pc {
            0x8262A378 => {
    //   block [0x8262A378..0x8262A3E4)
	// 8262A378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A384: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A38C: 38EB8070  addi r7, r11, -0x7f90
	ctx.r[7].s64 = ctx.r[11].s64 + -32656;
	// 8262A390: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262A394: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8262A398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A39C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A3A8: 386A400C  addi r3, r10, 0x400c
	ctx.r[3].s64 = ctx.r[10].s64 + 16396;
	// 8262A3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A3D0: 4BE3CA51  bl 0x82466e20
	ctx.lr = 0x8262A3D4;
	sub_82466E20(ctx, base);
	// 8262A3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A3E8 size=116
    let mut pc: u32 = 0x8262A3E8;
    'dispatch: loop {
        match pc {
            0x8262A3E8 => {
    //   block [0x8262A3E8..0x8262A45C)
	// 8262A3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A3F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A3F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A3FC: 390B80BC  addi r8, r11, -0x7f44
	ctx.r[8].s64 = ctx.r[11].s64 + -32580;
	// 8262A400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A404: 392A3820  addi r9, r10, 0x3820
	ctx.r[9].s64 = ctx.r[10].s64 + 14368;
	// 8262A408: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A40C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262A410: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A414: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A41C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A42C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262A430: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8262A434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A438: 386B403C  addi r3, r11, 0x403c
	ctx.r[3].s64 = ctx.r[11].s64 + 16444;
	// 8262A43C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A440: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A448: 4BE3C9D9  bl 0x82466e20
	ctx.lr = 0x8262A44C;
	sub_82466E20(ctx, base);
	// 8262A44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A460 size=108
    let mut pc: u32 = 0x8262A460;
    'dispatch: loop {
        match pc {
            0x8262A460 => {
    //   block [0x8262A460..0x8262A4CC)
	// 8262A460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A46C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A474: 38EB80D8  addi r7, r11, -0x7f28
	ctx.r[7].s64 = ctx.r[11].s64 + -32552;
	// 8262A478: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262A47C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 8262A480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A490: 386A406C  addi r3, r10, 0x406c
	ctx.r[3].s64 = ctx.r[10].s64 + 16492;
	// 8262A494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A4B8: 4BE3C969  bl 0x82466e20
	ctx.lr = 0x8262A4BC;
	sub_82466E20(ctx, base);
	// 8262A4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262A4D0 size=24
    let mut pc: u32 = 0x8262A4D0;
    'dispatch: loop {
        match pc {
            0x8262A4D0 => {
    //   block [0x8262A4D0..0x8262A4E8)
	// 8262A4D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A4D4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262A4D8: 394AB500  addi r10, r10, -0x4b00
	ctx.r[10].s64 = ctx.r[10].s64 + -19200;
	// 8262A4DC: 816B80D4  lwz r11, -0x7f2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32556 as u32) ) } as u64;
	// 8262A4E0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8262A4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A4E8 size=116
    let mut pc: u32 = 0x8262A4E8;
    'dispatch: loop {
        match pc {
            0x8262A4E8 => {
    //   block [0x8262A4E8..0x8262A55C)
	// 8262A4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A4F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A4FC: 390BB500  addi r8, r11, -0x4b00
	ctx.r[8].s64 = ctx.r[11].s64 + -19200;
	// 8262A500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A504: 392A387C  addi r9, r10, 0x387c
	ctx.r[9].s64 = ctx.r[10].s64 + 14460;
	// 8262A508: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A50C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8262A510: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A514: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A51C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A52C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262A530: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8262A534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A538: 386B409C  addi r3, r11, 0x409c
	ctx.r[3].s64 = ctx.r[11].s64 + 16540;
	// 8262A53C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8262A540: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A548: 4BE3C8D9  bl 0x82466e20
	ctx.lr = 0x8262A54C;
	sub_82466E20(ctx, base);
	// 8262A54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A560 size=108
    let mut pc: u32 = 0x8262A560;
    'dispatch: loop {
        match pc {
            0x8262A560 => {
    //   block [0x8262A560..0x8262A5CC)
	// 8262A560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A56C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A574: 38EB8140  addi r7, r11, -0x7ec0
	ctx.r[7].s64 = ctx.r[11].s64 + -32448;
	// 8262A578: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262A57C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8262A580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A590: 386A40CC  addi r3, r10, 0x40cc
	ctx.r[3].s64 = ctx.r[10].s64 + 16588;
	// 8262A594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A5B8: 4BE3C869  bl 0x82466e20
	ctx.lr = 0x8262A5BC;
	sub_82466E20(ctx, base);
	// 8262A5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A5D0 size=112
    let mut pc: u32 = 0x8262A5D0;
    'dispatch: loop {
        match pc {
            0x8262A5D0 => {
    //   block [0x8262A5D0..0x8262A640)
	// 8262A5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A5DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A5E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A5E4: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A5E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A5EC: 390B8170  addi r8, r11, -0x7e90
	ctx.r[8].s64 = ctx.r[11].s64 + -32400;
	// 8262A5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A5F4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8262A5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A5FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A608: 386A40FC  addi r3, r10, 0x40fc
	ctx.r[3].s64 = ctx.r[10].s64 + 16636;
	// 8262A60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A62C: 4BE3C7F5  bl 0x82466e20
	ctx.lr = 0x8262A630;
	sub_82466E20(ctx, base);
	// 8262A630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A640 size=112
    let mut pc: u32 = 0x8262A640;
    'dispatch: loop {
        match pc {
            0x8262A640 => {
    //   block [0x8262A640..0x8262A6B0)
	// 8262A640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A64C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A650: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A654: 392A38C0  addi r9, r10, 0x38c0
	ctx.r[9].s64 = ctx.r[10].s64 + 14528;
	// 8262A658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A65C: 390B8190  addi r8, r11, -0x7e70
	ctx.r[8].s64 = ctx.r[11].s64 + -32368;
	// 8262A660: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262A664: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8262A668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A66C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A678: 386A412C  addi r3, r10, 0x412c
	ctx.r[3].s64 = ctx.r[10].s64 + 16684;
	// 8262A67C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A680: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A69C: 4BE3C785  bl 0x82466e20
	ctx.lr = 0x8262A6A0;
	sub_82466E20(ctx, base);
	// 8262A6A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A6B0 size=112
    let mut pc: u32 = 0x8262A6B0;
    'dispatch: loop {
        match pc {
            0x8262A6B0 => {
    //   block [0x8262A6B0..0x8262A720)
	// 8262A6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A6BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A6C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A6C4: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A6CC: 390B81D8  addi r8, r11, -0x7e28
	ctx.r[8].s64 = ctx.r[11].s64 + -32296;
	// 8262A6D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A6D4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8262A6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A6DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A6E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A6E8: 386A415C  addi r3, r10, 0x415c
	ctx.r[3].s64 = ctx.r[10].s64 + 16732;
	// 8262A6EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A70C: 4BE3C715  bl 0x82466e20
	ctx.lr = 0x8262A710;
	sub_82466E20(ctx, base);
	// 8262A710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A720 size=112
    let mut pc: u32 = 0x8262A720;
    'dispatch: loop {
        match pc {
            0x8262A720 => {
    //   block [0x8262A720..0x8262A790)
	// 8262A720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A72C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A730: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A734: 392A38EC  addi r9, r10, 0x38ec
	ctx.r[9].s64 = ctx.r[10].s64 + 14572;
	// 8262A738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A73C: 390B81F0  addi r8, r11, -0x7e10
	ctx.r[8].s64 = ctx.r[11].s64 + -32272;
	// 8262A740: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262A744: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8262A748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A74C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A758: 386A418C  addi r3, r10, 0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + 16780;
	// 8262A75C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A760: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A77C: 4BE3C6A5  bl 0x82466e20
	ctx.lr = 0x8262A780;
	sub_82466E20(ctx, base);
	// 8262A780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A790 size=112
    let mut pc: u32 = 0x8262A790;
    'dispatch: loop {
        match pc {
            0x8262A790 => {
    //   block [0x8262A790..0x8262A800)
	// 8262A790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A7A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A7A4: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A7A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A7AC: 390B8280  addi r8, r11, -0x7d80
	ctx.r[8].s64 = ctx.r[11].s64 + -32128;
	// 8262A7B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A7B4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8262A7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A7BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A7C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A7C8: 386A41BC  addi r3, r10, 0x41bc
	ctx.r[3].s64 = ctx.r[10].s64 + 16828;
	// 8262A7CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A7EC: 4BE3C635  bl 0x82466e20
	ctx.lr = 0x8262A7F0;
	sub_82466E20(ctx, base);
	// 8262A7F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A800 size=112
    let mut pc: u32 = 0x8262A800;
    'dispatch: loop {
        match pc {
            0x8262A800 => {
    //   block [0x8262A800..0x8262A870)
	// 8262A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A810: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A814: 38AA421C  addi r5, r10, 0x421c
	ctx.r[5].s64 = ctx.r[10].s64 + 16924;
	// 8262A818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A81C: 390B8298  addi r8, r11, -0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + -32104;
	// 8262A820: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262A824: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8262A828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A82C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A838: 386A41EC  addi r3, r10, 0x41ec
	ctx.r[3].s64 = ctx.r[10].s64 + 16876;
	// 8262A83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A85C: 4BE3C5C5  bl 0x82466e20
	ctx.lr = 0x8262A860;
	sub_82466E20(ctx, base);
	// 8262A860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A870 size=100
    let mut pc: u32 = 0x8262A870;
    'dispatch: loop {
        match pc {
            0x8262A870 => {
    //   block [0x8262A870..0x8262A8D4)
	// 8262A870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A87C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A884: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A890: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8262A894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A8A4: 386A421C  addi r3, r10, 0x421c
	ctx.r[3].s64 = ctx.r[10].s64 + 16924;
	// 8262A8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A8AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A8B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262A8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A8B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262A8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A8C0: 4BE3C561  bl 0x82466e20
	ctx.lr = 0x8262A8C4;
	sub_82466E20(ctx, base);
	// 8262A8C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262A8D8 size=24
    let mut pc: u32 = 0x8262A8D8;
    'dispatch: loop {
        match pc {
            0x8262A8D8 => {
    //   block [0x8262A8D8..0x8262A8F0)
	// 8262A8D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A8DC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262A8E0: 394AB5D8  addi r10, r10, -0x4a28
	ctx.r[10].s64 = ctx.r[10].s64 + -18984;
	// 8262A8E4: 816B8310  lwz r11, -0x7cf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31984 as u32) ) } as u64;
	// 8262A8E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262A8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A8F0 size=116
    let mut pc: u32 = 0x8262A8F0;
    'dispatch: loop {
        match pc {
            0x8262A8F0 => {
    //   block [0x8262A8F0..0x8262A964)
	// 8262A8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A8FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A900: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A904: 390BB5D8  addi r8, r11, -0x4a28
	ctx.r[8].s64 = ctx.r[11].s64 + -18984;
	// 8262A908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A90C: 392A3928  addi r9, r10, 0x3928
	ctx.r[9].s64 = ctx.r[10].s64 + 14632;
	// 8262A910: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A914: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8262A918: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A91C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A924: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A934: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262A938: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8262A93C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A940: 386B424C  addi r3, r11, 0x424c
	ctx.r[3].s64 = ctx.r[11].s64 + 16972;
	// 8262A944: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A948: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A950: 4BE3C4D1  bl 0x82466e20
	ctx.lr = 0x8262A954;
	sub_82466E20(ctx, base);
	// 8262A954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A968 size=108
    let mut pc: u32 = 0x8262A968;
    'dispatch: loop {
        match pc {
            0x8262A968 => {
    //   block [0x8262A968..0x8262A9D4)
	// 8262A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A974: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A97C: 38EB8314  addi r7, r11, -0x7cec
	ctx.r[7].s64 = ctx.r[11].s64 + -31980;
	// 8262A980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262A984: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8262A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A98C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A998: 386A427C  addi r3, r10, 0x427c
	ctx.r[3].s64 = ctx.r[10].s64 + 17020;
	// 8262A99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A9C0: 4BE3C461  bl 0x82466e20
	ctx.lr = 0x8262A9C4;
	sub_82466E20(ctx, base);
	// 8262A9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A9D8 size=112
    let mut pc: u32 = 0x8262A9D8;
    'dispatch: loop {
        match pc {
            0x8262A9D8 => {
    //   block [0x8262A9D8..0x8262AA48)
	// 8262A9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A9E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A9E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A9EC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A9F4: 390B8344  addi r8, r11, -0x7cbc
	ctx.r[8].s64 = ctx.r[11].s64 + -31932;
	// 8262A9F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A9FC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8262AA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AA04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AA08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AA10: 386A42AC  addi r3, r10, 0x42ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17068;
	// 8262AA14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AA34: 4BE3C3ED  bl 0x82466e20
	ctx.lr = 0x8262AA38;
	sub_82466E20(ctx, base);
	// 8262AA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AA48 size=112
    let mut pc: u32 = 0x8262AA48;
    'dispatch: loop {
        match pc {
            0x8262AA48 => {
    //   block [0x8262AA48..0x8262AAB8)
	// 8262AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AA54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262AA58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AA5C: 392A394C  addi r9, r10, 0x394c
	ctx.r[9].s64 = ctx.r[10].s64 + 14668;
	// 8262AA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AA64: 390B8360  addi r8, r11, -0x7ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -31904;
	// 8262AA68: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262AA6C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8262AA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AA74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AA80: 386A42DC  addi r3, r10, 0x42dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17116;
	// 8262AA84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262AA88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262AA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AA9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AAA4: 4BE3C37D  bl 0x82466e20
	ctx.lr = 0x8262AAA8;
	sub_82466E20(ctx, base);
	// 8262AAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AAB8 size=112
    let mut pc: u32 = 0x8262AAB8;
    'dispatch: loop {
        match pc {
            0x8262AAB8 => {
    //   block [0x8262AAB8..0x8262AB28)
	// 8262AAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AAC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AAC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AACC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AAD4: 390B8408  addi r8, r11, -0x7bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -31736;
	// 8262AAD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262AADC: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8262AAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AAE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AAE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AAF0: 386A430C  addi r3, r10, 0x430c
	ctx.r[3].s64 = ctx.r[10].s64 + 17164;
	// 8262AAF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AB14: 4BE3C30D  bl 0x82466e20
	ctx.lr = 0x8262AB18;
	sub_82466E20(ctx, base);
	// 8262AB18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AB28 size=108
    let mut pc: u32 = 0x8262AB28;
    'dispatch: loop {
        match pc {
            0x8262AB28 => {
    //   block [0x8262AB28..0x8262AB94)
	// 8262AB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AB34: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AB3C: 38EB8420  addi r7, r11, -0x7be0
	ctx.r[7].s64 = ctx.r[11].s64 + -31712;
	// 8262AB40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262AB44: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8262AB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AB4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AB50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262AB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AB58: 386A433C  addi r3, r10, 0x433c
	ctx.r[3].s64 = ctx.r[10].s64 + 17212;
	// 8262AB5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262AB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AB7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AB80: 4BE3C2A1  bl 0x82466e20
	ctx.lr = 0x8262AB84;
	sub_82466E20(ctx, base);
	// 8262AB84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AB98 size=112
    let mut pc: u32 = 0x8262AB98;
    'dispatch: loop {
        match pc {
            0x8262AB98 => {
    //   block [0x8262AB98..0x8262AC08)
	// 8262AB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ABA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ABA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ABA8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ABAC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262ABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ABB4: 390B8450  addi r8, r11, -0x7bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31664;
	// 8262ABB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262ABBC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8262ABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ABC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ABC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ABD0: 386A436C  addi r3, r10, 0x436c
	ctx.r[3].s64 = ctx.r[10].s64 + 17260;
	// 8262ABD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262ABD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262ABDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262ABE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262ABEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262ABF4: 4BE3C22D  bl 0x82466e20
	ctx.lr = 0x8262ABF8;
	sub_82466E20(ctx, base);
	// 8262ABF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ABFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AC08 size=112
    let mut pc: u32 = 0x8262AC08;
    'dispatch: loop {
        match pc {
            0x8262AC08 => {
    //   block [0x8262AC08..0x8262AC78)
	// 8262AC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AC14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262AC18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AC1C: 392A3980  addi r9, r10, 0x3980
	ctx.r[9].s64 = ctx.r[10].s64 + 14720;
	// 8262AC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AC24: 390B8470  addi r8, r11, -0x7b90
	ctx.r[8].s64 = ctx.r[11].s64 + -31632;
	// 8262AC28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262AC2C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8262AC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AC34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AC40: 386A439C  addi r3, r10, 0x439c
	ctx.r[3].s64 = ctx.r[10].s64 + 17308;
	// 8262AC44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262AC48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262AC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AC64: 4BE3C1BD  bl 0x82466e20
	ctx.lr = 0x8262AC68;
	sub_82466E20(ctx, base);
	// 8262AC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AC78 size=112
    let mut pc: u32 = 0x8262AC78;
    'dispatch: loop {
        match pc {
            0x8262AC78 => {
    //   block [0x8262AC78..0x8262ACE8)
	// 8262AC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AC84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AC88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AC8C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AC94: 390B8518  addi r8, r11, -0x7ae8
	ctx.r[8].s64 = ctx.r[11].s64 + -31464;
	// 8262AC98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262AC9C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8262ACA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ACA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ACA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262ACAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ACB0: 386A43CC  addi r3, r10, 0x43cc
	ctx.r[3].s64 = ctx.r[10].s64 + 17356;
	// 8262ACB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262ACB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262ACBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ACC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262ACC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ACC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262ACCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ACD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262ACD4: 4BE3C14D  bl 0x82466e20
	ctx.lr = 0x8262ACD8;
	sub_82466E20(ctx, base);
	// 8262ACD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ACE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ACE8 size=112
    let mut pc: u32 = 0x8262ACE8;
    'dispatch: loop {
        match pc {
            0x8262ACE8 => {
    //   block [0x8262ACE8..0x8262AD58)
	// 8262ACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ACF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ACF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ACF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ACFC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AD04: 390B8560  addi r8, r11, -0x7aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -31392;
	// 8262AD08: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8262AD0C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8262AD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AD14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AD20: 386A43FC  addi r3, r10, 0x43fc
	ctx.r[3].s64 = ctx.r[10].s64 + 17404;
	// 8262AD24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AD44: 4BE3C0DD  bl 0x82466e20
	ctx.lr = 0x8262AD48;
	sub_82466E20(ctx, base);
	// 8262AD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AD58 size=100
    let mut pc: u32 = 0x8262AD58;
    'dispatch: loop {
        match pc {
            0x8262AD58 => {
    //   block [0x8262AD58..0x8262ADBC)
	// 8262AD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AD64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AD6C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AD78: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8262AD7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AD80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AD88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AD8C: 386A442C  addi r3, r10, 0x442c
	ctx.r[3].s64 = ctx.r[10].s64 + 17452;
	// 8262AD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AD94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AD98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262AD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ADA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262ADA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ADA8: 4BE3C079  bl 0x82466e20
	ctx.lr = 0x8262ADAC;
	sub_82466E20(ctx, base);
	// 8262ADAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ADB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ADB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ADB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ADC0 size=112
    let mut pc: u32 = 0x8262ADC0;
    'dispatch: loop {
        match pc {
            0x8262ADC0 => {
    //   block [0x8262ADC0..0x8262AE30)
	// 8262ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ADC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ADCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ADD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ADD4: 38AA409C  addi r5, r10, 0x409c
	ctx.r[5].s64 = ctx.r[10].s64 + 16540;
	// 8262ADD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ADDC: 390B8638  addi r8, r11, -0x79c8
	ctx.r[8].s64 = ctx.r[11].s64 + -31176;
	// 8262ADE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262ADE4: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8262ADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ADEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ADF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262ADF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ADF8: 386A445C  addi r3, r10, 0x445c
	ctx.r[3].s64 = ctx.r[10].s64 + 17500;
	// 8262ADFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AE1C: 4BE3C005  bl 0x82466e20
	ctx.lr = 0x8262AE20;
	sub_82466E20(ctx, base);
	// 8262AE20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AE30 size=112
    let mut pc: u32 = 0x8262AE30;
    'dispatch: loop {
        match pc {
            0x8262AE30 => {
    //   block [0x8262AE30..0x8262AEA0)
	// 8262AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AE3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AE40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AE44: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262AE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AE4C: 390B8668  addi r8, r11, -0x7998
	ctx.r[8].s64 = ctx.r[11].s64 + -31128;
	// 8262AE50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262AE54: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8262AE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AE5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AE60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AE68: 386A448C  addi r3, r10, 0x448c
	ctx.r[3].s64 = ctx.r[10].s64 + 17548;
	// 8262AE6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AE8C: 4BE3BF95  bl 0x82466e20
	ctx.lr = 0x8262AE90;
	sub_82466E20(ctx, base);
	// 8262AE90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AEA0 size=108
    let mut pc: u32 = 0x8262AEA0;
    'dispatch: loop {
        match pc {
            0x8262AEA0 => {
    //   block [0x8262AEA0..0x8262AF0C)
	// 8262AEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AEAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AEB4: 38EB8680  addi r7, r11, -0x7980
	ctx.r[7].s64 = ctx.r[11].s64 + -31104;
	// 8262AEB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262AEBC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8262AEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262AECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AED0: 386A44BC  addi r3, r10, 0x44bc
	ctx.r[3].s64 = ctx.r[10].s64 + 17596;
	// 8262AED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262AED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AEF8: 4BE3BF29  bl 0x82466e20
	ctx.lr = 0x8262AEFC;
	sub_82466E20(ctx, base);
	// 8262AEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AF10 size=112
    let mut pc: u32 = 0x8262AF10;
    'dispatch: loop {
        match pc {
            0x8262AF10 => {
    //   block [0x8262AF10..0x8262AF80)
	// 8262AF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AF1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AF20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AF24: 38AA442C  addi r5, r10, 0x442c
	ctx.r[5].s64 = ctx.r[10].s64 + 17452;
	// 8262AF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AF2C: 390B86B0  addi r8, r11, -0x7950
	ctx.r[8].s64 = ctx.r[11].s64 + -31056;
	// 8262AF30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262AF34: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8262AF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AF3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AF48: 386A44EC  addi r3, r10, 0x44ec
	ctx.r[3].s64 = ctx.r[10].s64 + 17644;
	// 8262AF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AF6C: 4BE3BEB5  bl 0x82466e20
	ctx.lr = 0x8262AF70;
	sub_82466E20(ctx, base);
	// 8262AF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AF80 size=112
    let mut pc: u32 = 0x8262AF80;
    'dispatch: loop {
        match pc {
            0x8262AF80 => {
    //   block [0x8262AF80..0x8262AFF0)
	// 8262AF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AF8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262AF90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AF94: 392A39AC  addi r9, r10, 0x39ac
	ctx.r[9].s64 = ctx.r[10].s64 + 14764;
	// 8262AF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AF9C: 390B8740  addi r8, r11, -0x78c0
	ctx.r[8].s64 = ctx.r[11].s64 + -30912;
	// 8262AFA0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262AFA4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8262AFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AFAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AFB8: 386A451C  addi r3, r10, 0x451c
	ctx.r[3].s64 = ctx.r[10].s64 + 17692;
	// 8262AFBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262AFC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AFDC: 4BE3BE45  bl 0x82466e20
	ctx.lr = 0x8262AFE0;
	sub_82466E20(ctx, base);
	// 8262AFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AFF0 size=112
    let mut pc: u32 = 0x8262AFF0;
    'dispatch: loop {
        match pc {
            0x8262AFF0 => {
    //   block [0x8262AFF0..0x8262B060)
	// 8262AFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AFFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B000: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B004: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B00C: 390B8788  addi r8, r11, -0x7878
	ctx.r[8].s64 = ctx.r[11].s64 + -30840;
	// 8262B010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262B014: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8262B018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B01C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B028: 386A454C  addi r3, r10, 0x454c
	ctx.r[3].s64 = ctx.r[10].s64 + 17740;
	// 8262B02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B04C: 4BE3BDD5  bl 0x82466e20
	ctx.lr = 0x8262B050;
	sub_82466E20(ctx, base);
	// 8262B050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B060 size=108
    let mut pc: u32 = 0x8262B060;
    'dispatch: loop {
        match pc {
            0x8262B060 => {
    //   block [0x8262B060..0x8262B0CC)
	// 8262B060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B06C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B074: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 8262B078: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262B07C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8262B080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B090: 386A457C  addi r3, r10, 0x457c
	ctx.r[3].s64 = ctx.r[10].s64 + 17788;
	// 8262B094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B0B8: 4BE3BD69  bl 0x82466e20
	ctx.lr = 0x8262B0BC;
	sub_82466E20(ctx, base);
	// 8262B0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B0D0 size=116
    let mut pc: u32 = 0x8262B0D0;
    'dispatch: loop {
        match pc {
            0x8262B0D0 => {
    //   block [0x8262B0D0..0x8262B144)
	// 8262B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B0DC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B0E0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8262B0E4: 390A8830  addi r8, r10, -0x77d0
	ctx.r[8].s64 = ctx.r[10].s64 + -30672;
	// 8262B0E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B0EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262B0F0: 38AA442C  addi r5, r10, 0x442c
	ctx.r[5].s64 = ctx.r[10].s64 + 17452;
	// 8262B0F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B0F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262B0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B104: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8262B108: 396B39C0  addi r11, r11, 0x39c0
	ctx.r[11].s64 = ctx.r[11].s64 + 14784;
	// 8262B10C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B110: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B114: 386A45AC  addi r3, r10, 0x45ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17836;
	// 8262B118: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262B11C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B120: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262B124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B130: 4BE3BCF1  bl 0x82466e20
	ctx.lr = 0x8262B134;
	sub_82466E20(ctx, base);
	// 8262B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B148 size=108
    let mut pc: u32 = 0x8262B148;
    'dispatch: loop {
        match pc {
            0x8262B148 => {
    //   block [0x8262B148..0x8262B1B4)
	// 8262B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B154: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B15C: 38EB8908  addi r7, r11, -0x76f8
	ctx.r[7].s64 = ctx.r[11].s64 + -30456;
	// 8262B160: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262B164: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8262B168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B178: 386A45DC  addi r3, r10, 0x45dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17884;
	// 8262B17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B1A0: 4BE3BC81  bl 0x82466e20
	ctx.lr = 0x8262B1A4;
	sub_82466E20(ctx, base);
	// 8262B1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B1B8 size=112
    let mut pc: u32 = 0x8262B1B8;
    'dispatch: loop {
        match pc {
            0x8262B1B8 => {
    //   block [0x8262B1B8..0x8262B228)
	// 8262B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B1C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B1C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B1CC: 38AA442C  addi r5, r10, 0x442c
	ctx.r[5].s64 = ctx.r[10].s64 + 17452;
	// 8262B1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B1D4: 390B8950  addi r8, r11, -0x76b0
	ctx.r[8].s64 = ctx.r[11].s64 + -30384;
	// 8262B1D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262B1DC: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8262B1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B1E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B1F0: 386A460C  addi r3, r10, 0x460c
	ctx.r[3].s64 = ctx.r[10].s64 + 17932;
	// 8262B1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B214: 4BE3BC0D  bl 0x82466e20
	ctx.lr = 0x8262B218;
	sub_82466E20(ctx, base);
	// 8262B218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B228 size=112
    let mut pc: u32 = 0x8262B228;
    'dispatch: loop {
        match pc {
            0x8262B228 => {
    //   block [0x8262B228..0x8262B298)
	// 8262B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B238: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B23C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B244: 390B89C8  addi r8, r11, -0x7638
	ctx.r[8].s64 = ctx.r[11].s64 + -30264;
	// 8262B248: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262B24C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8262B250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B254: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B260: 386A463C  addi r3, r10, 0x463c
	ctx.r[3].s64 = ctx.r[10].s64 + 17980;
	// 8262B264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B284: 4BE3BB9D  bl 0x82466e20
	ctx.lr = 0x8262B288;
	sub_82466E20(ctx, base);
	// 8262B288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B298 size=108
    let mut pc: u32 = 0x8262B298;
    'dispatch: loop {
        match pc {
            0x8262B298 => {
    //   block [0x8262B298..0x8262B304)
	// 8262B298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B2A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B2AC: 38EB89F8  addi r7, r11, -0x7608
	ctx.r[7].s64 = ctx.r[11].s64 + -30216;
	// 8262B2B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262B2B4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8262B2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B2BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B2C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B2C8: 386A466C  addi r3, r10, 0x466c
	ctx.r[3].s64 = ctx.r[10].s64 + 18028;
	// 8262B2CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B2F0: 4BE3BB31  bl 0x82466e20
	ctx.lr = 0x8262B2F4;
	sub_82466E20(ctx, base);
	// 8262B2F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B308 size=112
    let mut pc: u32 = 0x8262B308;
    'dispatch: loop {
        match pc {
            0x8262B308 => {
    //   block [0x8262B308..0x8262B378)
	// 8262B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B318: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B31C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B324: 390B8A70  addi r8, r11, -0x7590
	ctx.r[8].s64 = ctx.r[11].s64 + -30096;
	// 8262B328: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262B32C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8262B330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B334: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B340: 386A469C  addi r3, r10, 0x469c
	ctx.r[3].s64 = ctx.r[10].s64 + 18076;
	// 8262B344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B364: 4BE3BABD  bl 0x82466e20
	ctx.lr = 0x8262B368;
	sub_82466E20(ctx, base);
	// 8262B368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262B378 size=24
    let mut pc: u32 = 0x8262B378;
    'dispatch: loop {
        match pc {
            0x8262B378 => {
    //   block [0x8262B378..0x8262B390)
	// 8262B378: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B37C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B380: 394AB650  addi r10, r10, -0x49b0
	ctx.r[10].s64 = ctx.r[10].s64 + -18864;
	// 8262B384: 816B8AB8  lwz r11, -0x7548(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30024 as u32) ) } as u64;
	// 8262B388: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262B38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B390 size=116
    let mut pc: u32 = 0x8262B390;
    'dispatch: loop {
        match pc {
            0x8262B390 => {
    //   block [0x8262B390..0x8262B404)
	// 8262B390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B39C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B3A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262B3A4: 390BB650  addi r8, r11, -0x49b0
	ctx.r[8].s64 = ctx.r[11].s64 + -18864;
	// 8262B3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B3AC: 392A3A1C  addi r9, r10, 0x3a1c
	ctx.r[9].s64 = ctx.r[10].s64 + 14876;
	// 8262B3B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B3B4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262B3B8: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262B3BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B3C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B3D4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262B3D8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8262B3DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262B3E0: 386B46CC  addi r3, r11, 0x46cc
	ctx.r[3].s64 = ctx.r[11].s64 + 18124;
	// 8262B3E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262B3E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B3F0: 4BE3BA31  bl 0x82466e20
	ctx.lr = 0x8262B3F4;
	sub_82466E20(ctx, base);
	// 8262B3F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B408 size=112
    let mut pc: u32 = 0x8262B408;
    'dispatch: loop {
        match pc {
            0x8262B408 => {
    //   block [0x8262B408..0x8262B478)
	// 8262B408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B414: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B418: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B41C: 38AA46CC  addi r5, r10, 0x46cc
	ctx.r[5].s64 = ctx.r[10].s64 + 18124;
	// 8262B420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B424: 390B8ABC  addi r8, r11, -0x7544
	ctx.r[8].s64 = ctx.r[11].s64 + -30020;
	// 8262B428: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262B42C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8262B430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B440: 386A46FC  addi r3, r10, 0x46fc
	ctx.r[3].s64 = ctx.r[10].s64 + 18172;
	// 8262B444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B464: 4BE3B9BD  bl 0x82466e20
	ctx.lr = 0x8262B468;
	sub_82466E20(ctx, base);
	// 8262B468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B478 size=112
    let mut pc: u32 = 0x8262B478;
    'dispatch: loop {
        match pc {
            0x8262B478 => {
    //   block [0x8262B478..0x8262B4E8)
	// 8262B478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B488: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B48C: 38AA46FC  addi r5, r10, 0x46fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18172;
	// 8262B490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B494: 390B8AF0  addi r8, r11, -0x7510
	ctx.r[8].s64 = ctx.r[11].s64 + -29968;
	// 8262B498: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262B49C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8262B4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B4A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B4B0: 386A472C  addi r3, r10, 0x472c
	ctx.r[3].s64 = ctx.r[10].s64 + 18220;
	// 8262B4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B4D4: 4BE3B94D  bl 0x82466e20
	ctx.lr = 0x8262B4D8;
	sub_82466E20(ctx, base);
	// 8262B4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B4E8 size=112
    let mut pc: u32 = 0x8262B4E8;
    'dispatch: loop {
        match pc {
            0x8262B4E8 => {
    //   block [0x8262B4E8..0x8262B558)
	// 8262B4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B4F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B4F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B4FC: 38AA46FC  addi r5, r10, 0x46fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18172;
	// 8262B500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B504: 390B8B50  addi r8, r11, -0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + -29872;
	// 8262B508: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262B50C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 8262B510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B514: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B520: 386A475C  addi r3, r10, 0x475c
	ctx.r[3].s64 = ctx.r[10].s64 + 18268;
	// 8262B524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B544: 4BE3B8DD  bl 0x82466e20
	ctx.lr = 0x8262B548;
	sub_82466E20(ctx, base);
	// 8262B548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B558 size=112
    let mut pc: u32 = 0x8262B558;
    'dispatch: loop {
        match pc {
            0x8262B558 => {
    //   block [0x8262B558..0x8262B5C8)
	// 8262B558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B568: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B56C: 38AA46FC  addi r5, r10, 0x46fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18172;
	// 8262B570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B574: 390B8B80  addi r8, r11, -0x7480
	ctx.r[8].s64 = ctx.r[11].s64 + -29824;
	// 8262B578: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262B57C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 8262B580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B590: 386A478C  addi r3, r10, 0x478c
	ctx.r[3].s64 = ctx.r[10].s64 + 18316;
	// 8262B594: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B59C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B5B4: 4BE3B86D  bl 0x82466e20
	ctx.lr = 0x8262B5B8;
	sub_82466E20(ctx, base);
	// 8262B5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B5C8 size=108
    let mut pc: u32 = 0x8262B5C8;
    'dispatch: loop {
        match pc {
            0x8262B5C8 => {
    //   block [0x8262B5C8..0x8262B634)
	// 8262B5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B5D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B5DC: 38EB8BC8  addi r7, r11, -0x7438
	ctx.r[7].s64 = ctx.r[11].s64 + -29752;
	// 8262B5E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262B5E4: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 8262B5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B5EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B5F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B5F8: 386A47BC  addi r3, r10, 0x47bc
	ctx.r[3].s64 = ctx.r[10].s64 + 18364;
	// 8262B5FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B61C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B620: 4BE3B801  bl 0x82466e20
	ctx.lr = 0x8262B624;
	sub_82466E20(ctx, base);
	// 8262B624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B62C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B638 size=112
    let mut pc: u32 = 0x8262B638;
    'dispatch: loop {
        match pc {
            0x8262B638 => {
    //   block [0x8262B638..0x8262B6A8)
	// 8262B638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B648: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B64C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B654: 390B8BF8  addi r8, r11, -0x7408
	ctx.r[8].s64 = ctx.r[11].s64 + -29704;
	// 8262B658: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262B65C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 8262B660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B664: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B670: 386A47EC  addi r3, r10, 0x47ec
	ctx.r[3].s64 = ctx.r[10].s64 + 18412;
	// 8262B674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B694: 4BE3B78D  bl 0x82466e20
	ctx.lr = 0x8262B698;
	sub_82466E20(ctx, base);
	// 8262B698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B6A8 size=116
    let mut pc: u32 = 0x8262B6A8;
    'dispatch: loop {
        match pc {
            0x8262B6A8 => {
    //   block [0x8262B6A8..0x8262B71C)
	// 8262B6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B6B4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B6B8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8262B6BC: 390A8C10  addi r8, r10, -0x73f0
	ctx.r[8].s64 = ctx.r[10].s64 + -29680;
	// 8262B6C0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B6C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262B6C8: 38AA4C6C  addi r5, r10, 0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + 19564;
	// 8262B6CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B6D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262B6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B6DC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8262B6E0: 396B3A30  addi r11, r11, 0x3a30
	ctx.r[11].s64 = ctx.r[11].s64 + 14896;
	// 8262B6E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B6E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B6EC: 386A481C  addi r3, r10, 0x481c
	ctx.r[3].s64 = ctx.r[10].s64 + 18460;
	// 8262B6F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262B6F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B6F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262B6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B708: 4BE3B719  bl 0x82466e20
	ctx.lr = 0x8262B70C;
	sub_82466E20(ctx, base);
	// 8262B70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B720 size=100
    let mut pc: u32 = 0x8262B720;
    'dispatch: loop {
        match pc {
            0x8262B720 => {
    //   block [0x8262B720..0x8262B784)
	// 8262B720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B72C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B734: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262B738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B740: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8262B744: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B754: 386A484C  addi r3, r10, 0x484c
	ctx.r[3].s64 = ctx.r[10].s64 + 18508;
	// 8262B758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B75C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B760: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B768: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B770: 4BE3B6B1  bl 0x82466e20
	ctx.lr = 0x8262B774;
	sub_82466E20(ctx, base);
	// 8262B774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B788 size=100
    let mut pc: u32 = 0x8262B788;
    'dispatch: loop {
        match pc {
            0x8262B788 => {
    //   block [0x8262B788..0x8262B7EC)
	// 8262B788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B79C: 38AA48DC  addi r5, r10, 0x48dc
	ctx.r[5].s64 = ctx.r[10].s64 + 18652;
	// 8262B7A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B7A8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8262B7AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B7BC: 386A487C  addi r3, r10, 0x487c
	ctx.r[3].s64 = ctx.r[10].s64 + 18556;
	// 8262B7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B7C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B7C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B7D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B7D8: 4BE3B649  bl 0x82466e20
	ctx.lr = 0x8262B7DC;
	sub_82466E20(ctx, base);
	// 8262B7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B7F0 size=100
    let mut pc: u32 = 0x8262B7F0;
    'dispatch: loop {
        match pc {
            0x8262B7F0 => {
    //   block [0x8262B7F0..0x8262B854)
	// 8262B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B7FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B804: 38AA481C  addi r5, r10, 0x481c
	ctx.r[5].s64 = ctx.r[10].s64 + 18460;
	// 8262B808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B810: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8262B814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B824: 386A48AC  addi r3, r10, 0x48ac
	ctx.r[3].s64 = ctx.r[10].s64 + 18604;
	// 8262B828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B82C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B830: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B838: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B840: 4BE3B5E1  bl 0x82466e20
	ctx.lr = 0x8262B844;
	sub_82466E20(ctx, base);
	// 8262B844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B858 size=104
    let mut pc: u32 = 0x8262B858;
    'dispatch: loop {
        match pc {
            0x8262B858 => {
    //   block [0x8262B858..0x8262B8C0)
	// 8262B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B864: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B86C: 392A3A94  addi r9, r10, 0x3a94
	ctx.r[9].s64 = ctx.r[10].s64 + 14996;
	// 8262B870: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B878: 38AA484C  addi r5, r10, 0x484c
	ctx.r[5].s64 = ctx.r[10].s64 + 18508;
	// 8262B87C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B88C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8262B890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B8A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B8A4: 386A48DC  addi r3, r10, 0x48dc
	ctx.r[3].s64 = ctx.r[10].s64 + 18652;
	// 8262B8A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262B8AC: 4BE3B575  bl 0x82466e20
	ctx.lr = 0x8262B8B0;
	sub_82466E20(ctx, base);
	// 8262B8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B8C0 size=108
    let mut pc: u32 = 0x8262B8C0;
    'dispatch: loop {
        match pc {
            0x8262B8C0 => {
    //   block [0x8262B8C0..0x8262B92C)
	// 8262B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B8CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B8D4: 38EB8D90  addi r7, r11, -0x7270
	ctx.r[7].s64 = ctx.r[11].s64 + -29296;
	// 8262B8D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262B8DC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8262B8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B8E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B8F0: 386A490C  addi r3, r10, 0x490c
	ctx.r[3].s64 = ctx.r[10].s64 + 18700;
	// 8262B8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B918: 4BE3B509  bl 0x82466e20
	ctx.lr = 0x8262B91C;
	sub_82466E20(ctx, base);
	// 8262B91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B930 size=112
    let mut pc: u32 = 0x8262B930;
    'dispatch: loop {
        match pc {
            0x8262B930 => {
    //   block [0x8262B930..0x8262B9A0)
	// 8262B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B93C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B940: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B944: 38AA48DC  addi r5, r10, 0x48dc
	ctx.r[5].s64 = ctx.r[10].s64 + 18652;
	// 8262B948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B94C: 390B8DC0  addi r8, r11, -0x7240
	ctx.r[8].s64 = ctx.r[11].s64 + -29248;
	// 8262B950: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262B954: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8262B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B95C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B968: 386A493C  addi r3, r10, 0x493c
	ctx.r[3].s64 = ctx.r[10].s64 + 18748;
	// 8262B96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B98C: 4BE3B495  bl 0x82466e20
	ctx.lr = 0x8262B990;
	sub_82466E20(ctx, base);
	// 8262B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262B9A0 size=24
    let mut pc: u32 = 0x8262B9A0;
    'dispatch: loop {
        match pc {
            0x8262B9A0 => {
    //   block [0x8262B9A0..0x8262B9B8)
	// 8262B9A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B9A4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B9A8: 394AB668  addi r10, r10, -0x4998
	ctx.r[10].s64 = ctx.r[10].s64 + -18840;
	// 8262B9AC: 816B8E68  lwz r11, -0x7198(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29080 as u32) ) } as u64;
	// 8262B9B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B9B8 size=116
    let mut pc: u32 = 0x8262B9B8;
    'dispatch: loop {
        match pc {
            0x8262B9B8 => {
    //   block [0x8262B9B8..0x8262BA2C)
	// 8262B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B9C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262B9CC: 390BB668  addi r8, r11, -0x4998
	ctx.r[8].s64 = ctx.r[11].s64 + -18840;
	// 8262B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B9D4: 392A3AF8  addi r9, r10, 0x3af8
	ctx.r[9].s64 = ctx.r[10].s64 + 15096;
	// 8262B9D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B9DC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262B9E0: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262B9E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B9EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B9FC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262BA00: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8262BA04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262BA08: 386B496C  addi r3, r11, 0x496c
	ctx.r[3].s64 = ctx.r[11].s64 + 18796;
	// 8262BA0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262BA10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BA18: 4BE3B409  bl 0x82466e20
	ctx.lr = 0x8262BA1C;
	sub_82466E20(ctx, base);
	// 8262BA1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BA30 size=100
    let mut pc: u32 = 0x8262BA30;
    'dispatch: loop {
        match pc {
            0x8262BA30 => {
    //   block [0x8262BA30..0x8262BA94)
	// 8262BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BA3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BA44: 38AA496C  addi r5, r10, 0x496c
	ctx.r[5].s64 = ctx.r[10].s64 + 18796;
	// 8262BA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BA50: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8262BA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BA64: 386A499C  addi r3, r10, 0x499c
	ctx.r[3].s64 = ctx.r[10].s64 + 18844;
	// 8262BA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BA6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BA70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BA78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BA80: 4BE3B3A1  bl 0x82466e20
	ctx.lr = 0x8262BA84;
	sub_82466E20(ctx, base);
	// 8262BA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BA98 size=100
    let mut pc: u32 = 0x8262BA98;
    'dispatch: loop {
        match pc {
            0x8262BA98 => {
    //   block [0x8262BA98..0x8262BAFC)
	// 8262BA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BAA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BAAC: 38AA49FC  addi r5, r10, 0x49fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18940;
	// 8262BAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BAB8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8262BABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BACC: 386A49CC  addi r3, r10, 0x49cc
	ctx.r[3].s64 = ctx.r[10].s64 + 18892;
	// 8262BAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BAE8: 4BE3B339  bl 0x82466e20
	ctx.lr = 0x8262BAEC;
	sub_82466E20(ctx, base);
	// 8262BAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BB00 size=112
    let mut pc: u32 = 0x8262BB00;
    'dispatch: loop {
        match pc {
            0x8262BB00 => {
    //   block [0x8262BB00..0x8262BB70)
	// 8262BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BB0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BB14: 38AA496C  addi r5, r10, 0x496c
	ctx.r[5].s64 = ctx.r[10].s64 + 18796;
	// 8262BB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BB1C: 390B8E6C  addi r8, r11, -0x7194
	ctx.r[8].s64 = ctx.r[11].s64 + -29076;
	// 8262BB20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262BB24: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8262BB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BB2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BB38: 386A49FC  addi r3, r10, 0x49fc
	ctx.r[3].s64 = ctx.r[10].s64 + 18940;
	// 8262BB3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BB5C: 4BE3B2C5  bl 0x82466e20
	ctx.lr = 0x8262BB60;
	sub_82466E20(ctx, base);
	// 8262BB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BB70 size=100
    let mut pc: u32 = 0x8262BB70;
    'dispatch: loop {
        match pc {
            0x8262BB70 => {
    //   block [0x8262BB70..0x8262BBD4)
	// 8262BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BB7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BB84: 38AA49FC  addi r5, r10, 0x49fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18940;
	// 8262BB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BB8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BB90: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8262BB94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BBA4: 386A4A2C  addi r3, r10, 0x4a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 18988;
	// 8262BBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BBAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BBB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BBB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BBC0: 4BE3B261  bl 0x82466e20
	ctx.lr = 0x8262BBC4;
	sub_82466E20(ctx, base);
	// 8262BBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BBD8 size=100
    let mut pc: u32 = 0x8262BBD8;
    'dispatch: loop {
        match pc {
            0x8262BBD8 => {
    //   block [0x8262BBD8..0x8262BC3C)
	// 8262BBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BBE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BBEC: 38AA496C  addi r5, r10, 0x496c
	ctx.r[5].s64 = ctx.r[10].s64 + 18796;
	// 8262BBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BBF8: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8262BBFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BC0C: 386A4A5C  addi r3, r10, 0x4a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 19036;
	// 8262BC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BC14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BC18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BC20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BC28: 4BE3B1F9  bl 0x82466e20
	ctx.lr = 0x8262BC2C;
	sub_82466E20(ctx, base);
	// 8262BC2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BC40 size=100
    let mut pc: u32 = 0x8262BC40;
    'dispatch: loop {
        match pc {
            0x8262BC40 => {
    //   block [0x8262BC40..0x8262BCA4)
	// 8262BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BC4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BC54: 38AA499C  addi r5, r10, 0x499c
	ctx.r[5].s64 = ctx.r[10].s64 + 18844;
	// 8262BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BC60: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8262BC64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BC74: 386A4A8C  addi r3, r10, 0x4a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 19084;
	// 8262BC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BC7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BC80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BC88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BC90: 4BE3B191  bl 0x82466e20
	ctx.lr = 0x8262BC94;
	sub_82466E20(ctx, base);
	// 8262BC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BCA8 size=100
    let mut pc: u32 = 0x8262BCA8;
    'dispatch: loop {
        match pc {
            0x8262BCA8 => {
    //   block [0x8262BCA8..0x8262BD0C)
	// 8262BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BCB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BCBC: 38AA4A5C  addi r5, r10, 0x4a5c
	ctx.r[5].s64 = ctx.r[10].s64 + 19036;
	// 8262BCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BCC8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8262BCCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BCD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BCD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BCD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BCDC: 386A4ABC  addi r3, r10, 0x4abc
	ctx.r[3].s64 = ctx.r[10].s64 + 19132;
	// 8262BCE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BCE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BCE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BCF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BCF8: 4BE3B129  bl 0x82466e20
	ctx.lr = 0x8262BCFC;
	sub_82466E20(ctx, base);
	// 8262BCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BD10 size=100
    let mut pc: u32 = 0x8262BD10;
    'dispatch: loop {
        match pc {
            0x8262BD10 => {
    //   block [0x8262BD10..0x8262BD74)
	// 8262BD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BD1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BD24: 38AA499C  addi r5, r10, 0x499c
	ctx.r[5].s64 = ctx.r[10].s64 + 18844;
	// 8262BD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BD30: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8262BD34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BD44: 386A4AEC  addi r3, r10, 0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + 19180;
	// 8262BD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BD50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BD58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BD60: 4BE3B0C1  bl 0x82466e20
	ctx.lr = 0x8262BD64;
	sub_82466E20(ctx, base);
	// 8262BD64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BD78 size=112
    let mut pc: u32 = 0x8262BD78;
    'dispatch: loop {
        match pc {
            0x8262BD78 => {
    //   block [0x8262BD78..0x8262BDE8)
	// 8262BD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BD84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BD88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BD8C: 38AA4B7C  addi r5, r10, 0x4b7c
	ctx.r[5].s64 = ctx.r[10].s64 + 19324;
	// 8262BD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BD94: 390B8E9C  addi r8, r11, -0x7164
	ctx.r[8].s64 = ctx.r[11].s64 + -29028;
	// 8262BD98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262BD9C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8262BDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BDA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BDB0: 386A4B1C  addi r3, r10, 0x4b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 19228;
	// 8262BDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BDD4: 4BE3B04D  bl 0x82466e20
	ctx.lr = 0x8262BDD8;
	sub_82466E20(ctx, base);
	// 8262BDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BDE8 size=112
    let mut pc: u32 = 0x8262BDE8;
    'dispatch: loop {
        match pc {
            0x8262BDE8 => {
    //   block [0x8262BDE8..0x8262BE58)
	// 8262BDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BDF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BDF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BDFC: 38AA4BAC  addi r5, r10, 0x4bac
	ctx.r[5].s64 = ctx.r[10].s64 + 19372;
	// 8262BE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BE04: 390B8ECC  addi r8, r11, -0x7134
	ctx.r[8].s64 = ctx.r[11].s64 + -28980;
	// 8262BE08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262BE0C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8262BE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BE14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BE20: 386A4B4C  addi r3, r10, 0x4b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 19276;
	// 8262BE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BE44: 4BE3AFDD  bl 0x82466e20
	ctx.lr = 0x8262BE48;
	sub_82466E20(ctx, base);
	// 8262BE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BE58 size=112
    let mut pc: u32 = 0x8262BE58;
    'dispatch: loop {
        match pc {
            0x8262BE58 => {
    //   block [0x8262BE58..0x8262BEC8)
	// 8262BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BE64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BE68: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BE6C: 38AA4C6C  addi r5, r10, 0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + 19564;
	// 8262BE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BE74: 390B8EE4  addi r8, r11, -0x711c
	ctx.r[8].s64 = ctx.r[11].s64 + -28956;
	// 8262BE78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262BE7C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8262BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BE84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BE90: 386A4B7C  addi r3, r10, 0x4b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 19324;
	// 8262BE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BEB4: 4BE3AF6D  bl 0x82466e20
	ctx.lr = 0x8262BEB8;
	sub_82466E20(ctx, base);
	// 8262BEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


