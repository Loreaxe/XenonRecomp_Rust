pub fn sub_826EA860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA860 size=112
    let mut pc: u32 = 0x826EA860;
    'dispatch: loop {
        match pc {
            0x826EA860 => {
    //   block [0x826EA860..0x826EA8D0)
	// 826EA860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA870: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA874: 38AA502C  addi r5, r10, 0x502c
	ctx.r[5].s64 = ctx.r[10].s64 + 20524;
	// 826EA878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA87C: 390B339C  addi r8, r11, 0x339c
	ctx.r[8].s64 = ctx.r[11].s64 + 13212;
	// 826EA880: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EA884: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 826EA888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA88C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA898: 386A505C  addi r3, r10, 0x505c
	ctx.r[3].s64 = ctx.r[10].s64 + 20572;
	// 826EA89C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA8BC: 4BD7C565  bl 0x82466e20
	ctx.lr = 0x826EA8C0;
	sub_82466E20(ctx, base);
	// 826EA8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EA8D0 size=24
    let mut pc: u32 = 0x826EA8D0;
    'dispatch: loop {
        match pc {
            0x826EA8D0 => {
    //   block [0x826EA8D0..0x826EA8E8)
	// 826EA8D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA8D4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EA8D8: 394A80E8  addi r10, r10, -0x7f18
	ctx.r[10].s64 = ctx.r[10].s64 + -32536;
	// 826EA8DC: 816B33CC  lwz r11, 0x33cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13260 as u32) ) } as u64;
	// 826EA8E0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826EA8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA8E8 size=116
    let mut pc: u32 = 0x826EA8E8;
    'dispatch: loop {
        match pc {
            0x826EA8E8 => {
    //   block [0x826EA8E8..0x826EA95C)
	// 826EA8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA8F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EA8F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EA8FC: 390B80E8  addi r8, r11, -0x7f18
	ctx.r[8].s64 = ctx.r[11].s64 + -32536;
	// 826EA900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA904: 392A85FC  addi r9, r10, -0x7a04
	ctx.r[9].s64 = ctx.r[10].s64 + -31236;
	// 826EA908: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA90C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826EA910: 38AA505C  addi r5, r10, 0x505c
	ctx.r[5].s64 = ctx.r[10].s64 + 20572;
	// 826EA914: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA91C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA92C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EA930: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826EA934: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EA938: 386B508C  addi r3, r11, 0x508c
	ctx.r[3].s64 = ctx.r[11].s64 + 20620;
	// 826EA93C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EA940: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA948: 4BD7C4D9  bl 0x82466e20
	ctx.lr = 0x826EA94C;
	sub_82466E20(ctx, base);
	// 826EA94C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA960 size=112
    let mut pc: u32 = 0x826EA960;
    'dispatch: loop {
        match pc {
            0x826EA960 => {
    //   block [0x826EA960..0x826EA9D0)
	// 826EA960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA96C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA970: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA974: 38AA505C  addi r5, r10, 0x505c
	ctx.r[5].s64 = ctx.r[10].s64 + 20572;
	// 826EA978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA97C: 390B33D0  addi r8, r11, 0x33d0
	ctx.r[8].s64 = ctx.r[11].s64 + 13264;
	// 826EA980: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826EA984: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 826EA988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA98C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA998: 386A50BC  addi r3, r10, 0x50bc
	ctx.r[3].s64 = ctx.r[10].s64 + 20668;
	// 826EA99C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA9BC: 4BD7C465  bl 0x82466e20
	ctx.lr = 0x826EA9C0;
	sub_82466E20(ctx, base);
	// 826EA9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA9D0 size=112
    let mut pc: u32 = 0x826EA9D0;
    'dispatch: loop {
        match pc {
            0x826EA9D0 => {
    //   block [0x826EA9D0..0x826EAA40)
	// 826EA9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA9DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA9E0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA9E4: 38AA505C  addi r5, r10, 0x505c
	ctx.r[5].s64 = ctx.r[10].s64 + 20572;
	// 826EA9E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA9EC: 390B3430  addi r8, r11, 0x3430
	ctx.r[8].s64 = ctx.r[11].s64 + 13360;
	// 826EA9F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EA9F4: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 826EA9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA9FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EAA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAA08: 386A50EC  addi r3, r10, 0x50ec
	ctx.r[3].s64 = ctx.r[10].s64 + 20716;
	// 826EAA0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EAA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAA2C: 4BD7C3F5  bl 0x82466e20
	ctx.lr = 0x826EAA30;
	sub_82466E20(ctx, base);
	// 826EAA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAA40 size=112
    let mut pc: u32 = 0x826EAA40;
    'dispatch: loop {
        match pc {
            0x826EAA40 => {
    //   block [0x826EAA40..0x826EAAB0)
	// 826EAA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAA4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAA50: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAA54: 38AA505C  addi r5, r10, 0x505c
	ctx.r[5].s64 = ctx.r[10].s64 + 20572;
	// 826EAA58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAA5C: 390B3460  addi r8, r11, 0x3460
	ctx.r[8].s64 = ctx.r[11].s64 + 13408;
	// 826EAA60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EAA64: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826EAA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAA6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAA70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EAA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAA78: 386A511C  addi r3, r10, 0x511c
	ctx.r[3].s64 = ctx.r[10].s64 + 20764;
	// 826EAA7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EAA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAA84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAA9C: 4BD7C385  bl 0x82466e20
	ctx.lr = 0x826EAAA0;
	sub_82466E20(ctx, base);
	// 826EAAA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAAB0 size=108
    let mut pc: u32 = 0x826EAAB0;
    'dispatch: loop {
        match pc {
            0x826EAAB0 => {
    //   block [0x826EAAB0..0x826EAB1C)
	// 826EAAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAABC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAAC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAAC4: 38EB34A8  addi r7, r11, 0x34a8
	ctx.r[7].s64 = ctx.r[11].s64 + 13480;
	// 826EAAC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EAACC: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 826EAAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAAD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAAD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EAADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAAE0: 386A514C  addi r3, r10, 0x514c
	ctx.r[3].s64 = ctx.r[10].s64 + 20812;
	// 826EAAE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EAAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EAB08: 4BD7C319  bl 0x82466e20
	ctx.lr = 0x826EAB0C;
	sub_82466E20(ctx, base);
	// 826EAB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAB20 size=112
    let mut pc: u32 = 0x826EAB20;
    'dispatch: loop {
        match pc {
            0x826EAB20 => {
    //   block [0x826EAB20..0x826EAB90)
	// 826EAB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAB2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAB30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAB34: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EAB38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAB3C: 390B34D8  addi r8, r11, 0x34d8
	ctx.r[8].s64 = ctx.r[11].s64 + 13528;
	// 826EAB40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EAB44: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826EAB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAB4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAB50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EAB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAB58: 386A517C  addi r3, r10, 0x517c
	ctx.r[3].s64 = ctx.r[10].s64 + 20860;
	// 826EAB5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EAB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAB7C: 4BD7C2A5  bl 0x82466e20
	ctx.lr = 0x826EAB80;
	sub_82466E20(ctx, base);
	// 826EAB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAB90 size=108
    let mut pc: u32 = 0x826EAB90;
    'dispatch: loop {
        match pc {
            0x826EAB90 => {
    //   block [0x826EAB90..0x826EABFC)
	// 826EAB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAB9C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EABA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EABA4: 38EB34F8  addi r7, r11, 0x34f8
	ctx.r[7].s64 = ctx.r[11].s64 + 13560;
	// 826EABA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826EABAC: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 826EABB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EABB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EABB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EABBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EABC0: 386A51AC  addi r3, r10, 0x51ac
	ctx.r[3].s64 = ctx.r[10].s64 + 20908;
	// 826EABC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EABC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EABCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EABD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EABD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EABD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EABDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EABE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EABE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EABE8: 4BD7C239  bl 0x82466e20
	ctx.lr = 0x826EABEC;
	sub_82466E20(ctx, base);
	// 826EABEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EABF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EABF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EABF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAC00 size=108
    let mut pc: u32 = 0x826EAC00;
    'dispatch: loop {
        match pc {
            0x826EAC00 => {
    //   block [0x826EAC00..0x826EAC6C)
	// 826EAC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAC0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAC14: 38EB3540  addi r7, r11, 0x3540
	ctx.r[7].s64 = ctx.r[11].s64 + 13632;
	// 826EAC18: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EAC1C: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 826EAC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAC24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EAC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAC30: 386A51DC  addi r3, r10, 0x51dc
	ctx.r[3].s64 = ctx.r[10].s64 + 20956;
	// 826EAC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EAC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EAC58: 4BD7C1C9  bl 0x82466e20
	ctx.lr = 0x826EAC5C;
	sub_82466E20(ctx, base);
	// 826EAC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAC70 size=108
    let mut pc: u32 = 0x826EAC70;
    'dispatch: loop {
        match pc {
            0x826EAC70 => {
    //   block [0x826EAC70..0x826EACDC)
	// 826EAC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAC7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAC80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAC84: 38EB35A0  addi r7, r11, 0x35a0
	ctx.r[7].s64 = ctx.r[11].s64 + 13728;
	// 826EAC88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EAC8C: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 826EAC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAC94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EAC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EACA0: 386A520C  addi r3, r10, 0x520c
	ctx.r[3].s64 = ctx.r[10].s64 + 21004;
	// 826EACA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EACA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EACAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EACB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EACB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EACB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EACBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EACC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EACC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EACC8: 4BD7C159  bl 0x82466e20
	ctx.lr = 0x826EACCC;
	sub_82466E20(ctx, base);
	// 826EACCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EACD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EACD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EACD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EACE0 size=116
    let mut pc: u32 = 0x826EACE0;
    'dispatch: loop {
        match pc {
            0x826EACE0 => {
    //   block [0x826EACE0..0x826EAD54)
	// 826EACE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EACE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EACE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EACEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EACF0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EACF4: 392B8638  addi r9, r11, -0x79c8
	ctx.r[9].s64 = ctx.r[11].s64 + -31176;
	// 826EACF8: 38AA571C  addi r5, r10, 0x571c
	ctx.r[5].s64 = ctx.r[10].s64 + 22300;
	// 826EACFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAD00: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826EAD04: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826EAD08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAD0C: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 826EAD10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAD14: 396B35D0  addi r11, r11, 0x35d0
	ctx.r[11].s64 = ctx.r[11].s64 + 13776;
	// 826EAD18: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EAD1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAD20: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EAD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAD28: 386A523C  addi r3, r10, 0x523c
	ctx.r[3].s64 = ctx.r[10].s64 + 21052;
	// 826EAD2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EAD30: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EAD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAD38: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EAD3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EAD40: 4BD7C0E1  bl 0x82466e20
	ctx.lr = 0x826EAD44;
	sub_82466E20(ctx, base);
	// 826EAD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAD58 size=100
    let mut pc: u32 = 0x826EAD58;
    'dispatch: loop {
        match pc {
            0x826EAD58 => {
    //   block [0x826EAD58..0x826EADBC)
	// 826EAD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAD64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAD6C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EAD70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAD78: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826EAD7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAD80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAD88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAD8C: 386A526C  addi r3, r10, 0x526c
	ctx.r[3].s64 = ctx.r[10].s64 + 21100;
	// 826EAD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAD94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAD98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EAD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EADA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EADA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EADA8: 4BD7C079  bl 0x82466e20
	ctx.lr = 0x826EADAC;
	sub_82466E20(ctx, base);
	// 826EADAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EADB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EADB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EADB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EADC0 size=100
    let mut pc: u32 = 0x826EADC0;
    'dispatch: loop {
        match pc {
            0x826EADC0 => {
    //   block [0x826EADC0..0x826EAE24)
	// 826EADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EADC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EADCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EADD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EADD4: 38AA52FC  addi r5, r10, 0x52fc
	ctx.r[5].s64 = ctx.r[10].s64 + 21244;
	// 826EADD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EADDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EADE0: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 826EADE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EADEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EADF4: 386A529C  addi r3, r10, 0x529c
	ctx.r[3].s64 = ctx.r[10].s64 + 21148;
	// 826EADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EADFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAE00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EAE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAE08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EAE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAE10: 4BD7C011  bl 0x82466e20
	ctx.lr = 0x826EAE14;
	sub_82466E20(ctx, base);
	// 826EAE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAE28 size=100
    let mut pc: u32 = 0x826EAE28;
    'dispatch: loop {
        match pc {
            0x826EAE28 => {
    //   block [0x826EAE28..0x826EAE8C)
	// 826EAE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAE3C: 38AA523C  addi r5, r10, 0x523c
	ctx.r[5].s64 = ctx.r[10].s64 + 21052;
	// 826EAE40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAE48: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826EAE4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAE5C: 386A52CC  addi r3, r10, 0x52cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21196;
	// 826EAE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAE64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAE68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EAE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAE70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EAE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAE78: 4BD7BFA9  bl 0x82466e20
	ctx.lr = 0x826EAE7C;
	sub_82466E20(ctx, base);
	// 826EAE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAE90 size=104
    let mut pc: u32 = 0x826EAE90;
    'dispatch: loop {
        match pc {
            0x826EAE90 => {
    //   block [0x826EAE90..0x826EAEF8)
	// 826EAE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAE9C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EAEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAEA4: 392A86B8  addi r9, r10, -0x7948
	ctx.r[9].s64 = ctx.r[10].s64 + -31048;
	// 826EAEA8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAEB0: 38AA526C  addi r5, r10, 0x526c
	ctx.r[5].s64 = ctx.r[10].s64 + 21100;
	// 826EAEB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAEC4: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 826EAEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAED0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EAED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAED8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EAEDC: 386A52FC  addi r3, r10, 0x52fc
	ctx.r[3].s64 = ctx.r[10].s64 + 21244;
	// 826EAEE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EAEE4: 4BD7BF3D  bl 0x82466e20
	ctx.lr = 0x826EAEE8;
	sub_82466E20(ctx, base);
	// 826EAEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAEF8 size=108
    let mut pc: u32 = 0x826EAEF8;
    'dispatch: loop {
        match pc {
            0x826EAEF8 => {
    //   block [0x826EAEF8..0x826EAF64)
	// 826EAEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAF04: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAF08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAF0C: 38EB3768  addi r7, r11, 0x3768
	ctx.r[7].s64 = ctx.r[11].s64 + 14184;
	// 826EAF10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EAF14: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 826EAF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAF1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EAF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAF28: 386A532C  addi r3, r10, 0x532c
	ctx.r[3].s64 = ctx.r[10].s64 + 21292;
	// 826EAF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EAF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EAF50: 4BD7BED1  bl 0x82466e20
	ctx.lr = 0x826EAF54;
	sub_82466E20(ctx, base);
	// 826EAF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAF68 size=112
    let mut pc: u32 = 0x826EAF68;
    'dispatch: loop {
        match pc {
            0x826EAF68 => {
    //   block [0x826EAF68..0x826EAFD8)
	// 826EAF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAF74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAF78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAF7C: 38AA52FC  addi r5, r10, 0x52fc
	ctx.r[5].s64 = ctx.r[10].s64 + 21244;
	// 826EAF80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EAF84: 390B3798  addi r8, r11, 0x3798
	ctx.r[8].s64 = ctx.r[11].s64 + 14232;
	// 826EAF88: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826EAF8C: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 826EAF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EAF94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EAF98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EAF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EAFA0: 386A535C  addi r3, r10, 0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + 21340;
	// 826EAFA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EAFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EAFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EAFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EAFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EAFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EAFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EAFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EAFC4: 4BD7BE5D  bl 0x82466e20
	ctx.lr = 0x826EAFC8;
	sub_82466E20(ctx, base);
	// 826EAFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EAFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EAFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EAFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EAFD8 size=24
    let mut pc: u32 = 0x826EAFD8;
    'dispatch: loop {
        match pc {
            0x826EAFD8 => {
    //   block [0x826EAFD8..0x826EAFF0)
	// 826EAFD8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EAFDC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EAFE0: 394A8160  addi r10, r10, -0x7ea0
	ctx.r[10].s64 = ctx.r[10].s64 + -32416;
	// 826EAFE4: 816B3840  lwz r11, 0x3840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14400 as u32) ) } as u64;
	// 826EAFE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826EAFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EAFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EAFF0 size=116
    let mut pc: u32 = 0x826EAFF0;
    'dispatch: loop {
        match pc {
            0x826EAFF0 => {
    //   block [0x826EAFF0..0x826EB064)
	// 826EAFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EAFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EAFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EAFFC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EB000: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EB004: 390B8160  addi r8, r11, -0x7ea0
	ctx.r[8].s64 = ctx.r[11].s64 + -32416;
	// 826EB008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB00C: 392A8728  addi r9, r10, -0x78d8
	ctx.r[9].s64 = ctx.r[10].s64 + -30936;
	// 826EB010: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB014: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826EB018: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB01C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB024: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB034: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EB038: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826EB03C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EB040: 386B538C  addi r3, r11, 0x538c
	ctx.r[3].s64 = ctx.r[11].s64 + 21388;
	// 826EB044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EB048: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB050: 4BD7BDD1  bl 0x82466e20
	ctx.lr = 0x826EB054;
	sub_82466E20(ctx, base);
	// 826EB054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB068 size=100
    let mut pc: u32 = 0x826EB068;
    'dispatch: loop {
        match pc {
            0x826EB068 => {
    //   block [0x826EB068..0x826EB0CC)
	// 826EB068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB07C: 38AA538C  addi r5, r10, 0x538c
	ctx.r[5].s64 = ctx.r[10].s64 + 21388;
	// 826EB080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB088: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 826EB08C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB09C: 386A53BC  addi r3, r10, 0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + 21436;
	// 826EB0A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB0A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB0A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB0B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB0B8: 4BD7BD69  bl 0x82466e20
	ctx.lr = 0x826EB0BC;
	sub_82466E20(ctx, base);
	// 826EB0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB0D0 size=100
    let mut pc: u32 = 0x826EB0D0;
    'dispatch: loop {
        match pc {
            0x826EB0D0 => {
    //   block [0x826EB0D0..0x826EB134)
	// 826EB0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB0DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB0E4: 38AA538C  addi r5, r10, 0x538c
	ctx.r[5].s64 = ctx.r[10].s64 + 21388;
	// 826EB0E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB0F0: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 826EB0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB0FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB104: 386A53EC  addi r3, r10, 0x53ec
	ctx.r[3].s64 = ctx.r[10].s64 + 21484;
	// 826EB108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB10C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB110: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB118: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB120: 4BD7BD01  bl 0x82466e20
	ctx.lr = 0x826EB124;
	sub_82466E20(ctx, base);
	// 826EB124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB12C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB138 size=100
    let mut pc: u32 = 0x826EB138;
    'dispatch: loop {
        match pc {
            0x826EB138 => {
    //   block [0x826EB138..0x826EB19C)
	// 826EB138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB14C: 38AA544C  addi r5, r10, 0x544c
	ctx.r[5].s64 = ctx.r[10].s64 + 21580;
	// 826EB150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB158: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826EB15C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB16C: 386A541C  addi r3, r10, 0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + 21532;
	// 826EB170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB174: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB178: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB180: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB188: 4BD7BC99  bl 0x82466e20
	ctx.lr = 0x826EB18C;
	sub_82466E20(ctx, base);
	// 826EB18C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB1A0 size=100
    let mut pc: u32 = 0x826EB1A0;
    'dispatch: loop {
        match pc {
            0x826EB1A0 => {
    //   block [0x826EB1A0..0x826EB204)
	// 826EB1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB1AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB1B4: 38AA538C  addi r5, r10, 0x538c
	ctx.r[5].s64 = ctx.r[10].s64 + 21388;
	// 826EB1B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB1C0: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 826EB1C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB1D4: 386A544C  addi r3, r10, 0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + 21580;
	// 826EB1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB1DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB1E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB1E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB1F0: 4BD7BC31  bl 0x82466e20
	ctx.lr = 0x826EB1F4;
	sub_82466E20(ctx, base);
	// 826EB1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB208 size=100
    let mut pc: u32 = 0x826EB208;
    'dispatch: loop {
        match pc {
            0x826EB208 => {
    //   block [0x826EB208..0x826EB26C)
	// 826EB208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB214: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB21C: 38AA544C  addi r5, r10, 0x544c
	ctx.r[5].s64 = ctx.r[10].s64 + 21580;
	// 826EB220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB228: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 826EB22C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB23C: 386A547C  addi r3, r10, 0x547c
	ctx.r[3].s64 = ctx.r[10].s64 + 21628;
	// 826EB240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB244: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB248: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB258: 4BD7BBC9  bl 0x82466e20
	ctx.lr = 0x826EB25C;
	sub_82466E20(ctx, base);
	// 826EB25C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB270 size=100
    let mut pc: u32 = 0x826EB270;
    'dispatch: loop {
        match pc {
            0x826EB270 => {
    //   block [0x826EB270..0x826EB2D4)
	// 826EB270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB27C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB284: 38AA538C  addi r5, r10, 0x538c
	ctx.r[5].s64 = ctx.r[10].s64 + 21388;
	// 826EB288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB290: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 826EB294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB2A4: 386A54AC  addi r3, r10, 0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + 21676;
	// 826EB2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB2AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB2B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB2B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB2C0: 4BD7BB61  bl 0x82466e20
	ctx.lr = 0x826EB2C4;
	sub_82466E20(ctx, base);
	// 826EB2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB2D8 size=100
    let mut pc: u32 = 0x826EB2D8;
    'dispatch: loop {
        match pc {
            0x826EB2D8 => {
    //   block [0x826EB2D8..0x826EB33C)
	// 826EB2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB2E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB2EC: 38AA53BC  addi r5, r10, 0x53bc
	ctx.r[5].s64 = ctx.r[10].s64 + 21436;
	// 826EB2F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB2F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB2F8: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 826EB2FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB30C: 386A54DC  addi r3, r10, 0x54dc
	ctx.r[3].s64 = ctx.r[10].s64 + 21724;
	// 826EB310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB320: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB328: 4BD7BAF9  bl 0x82466e20
	ctx.lr = 0x826EB32C;
	sub_82466E20(ctx, base);
	// 826EB32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB340 size=100
    let mut pc: u32 = 0x826EB340;
    'dispatch: loop {
        match pc {
            0x826EB340 => {
    //   block [0x826EB340..0x826EB3A4)
	// 826EB340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB34C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB354: 38AA54AC  addi r5, r10, 0x54ac
	ctx.r[5].s64 = ctx.r[10].s64 + 21676;
	// 826EB358: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB360: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 826EB364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB374: 386A550C  addi r3, r10, 0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + 21772;
	// 826EB378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB37C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB380: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB388: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB390: 4BD7BA91  bl 0x82466e20
	ctx.lr = 0x826EB394;
	sub_82466E20(ctx, base);
	// 826EB394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB3A8 size=100
    let mut pc: u32 = 0x826EB3A8;
    'dispatch: loop {
        match pc {
            0x826EB3A8 => {
    //   block [0x826EB3A8..0x826EB40C)
	// 826EB3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB3B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB3BC: 38AA53BC  addi r5, r10, 0x53bc
	ctx.r[5].s64 = ctx.r[10].s64 + 21436;
	// 826EB3C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB3C8: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 826EB3CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB3DC: 386A553C  addi r3, r10, 0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + 21820;
	// 826EB3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB3E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB3E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EB3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB3F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB3F8: 4BD7BA29  bl 0x82466e20
	ctx.lr = 0x826EB3FC;
	sub_82466E20(ctx, base);
	// 826EB3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB410 size=112
    let mut pc: u32 = 0x826EB410;
    'dispatch: loop {
        match pc {
            0x826EB410 => {
    //   block [0x826EB410..0x826EB480)
	// 826EB410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB41C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB420: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB424: 38AA55CC  addi r5, r10, 0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + 21964;
	// 826EB428: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB42C: 390B3844  addi r8, r11, 0x3844
	ctx.r[8].s64 = ctx.r[11].s64 + 14404;
	// 826EB430: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EB434: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 826EB438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB43C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB448: 386A556C  addi r3, r10, 0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + 21868;
	// 826EB44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EB450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB46C: 4BD7B9B5  bl 0x82466e20
	ctx.lr = 0x826EB470;
	sub_82466E20(ctx, base);
	// 826EB470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB480 size=112
    let mut pc: u32 = 0x826EB480;
    'dispatch: loop {
        match pc {
            0x826EB480 => {
    //   block [0x826EB480..0x826EB4F0)
	// 826EB480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB48C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB490: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB494: 38AA55FC  addi r5, r10, 0x55fc
	ctx.r[5].s64 = ctx.r[10].s64 + 22012;
	// 826EB498: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB49C: 390B3874  addi r8, r11, 0x3874
	ctx.r[8].s64 = ctx.r[11].s64 + 14452;
	// 826EB4A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EB4A4: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 826EB4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB4AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB4B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB4B8: 386A559C  addi r3, r10, 0x559c
	ctx.r[3].s64 = ctx.r[10].s64 + 21916;
	// 826EB4BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EB4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB4DC: 4BD7B945  bl 0x82466e20
	ctx.lr = 0x826EB4E0;
	sub_82466E20(ctx, base);
	// 826EB4E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB4F0 size=112
    let mut pc: u32 = 0x826EB4F0;
    'dispatch: loop {
        match pc {
            0x826EB4F0 => {
    //   block [0x826EB4F0..0x826EB560)
	// 826EB4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB4FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB500: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB504: 38AA571C  addi r5, r10, 0x571c
	ctx.r[5].s64 = ctx.r[10].s64 + 22300;
	// 826EB508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB50C: 390B388C  addi r8, r11, 0x388c
	ctx.r[8].s64 = ctx.r[11].s64 + 14476;
	// 826EB510: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EB514: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 826EB518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB51C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB528: 386A55CC  addi r3, r10, 0x55cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21964;
	// 826EB52C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EB530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB54C: 4BD7B8D5  bl 0x82466e20
	ctx.lr = 0x826EB550;
	sub_82466E20(ctx, base);
	// 826EB550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB560 size=112
    let mut pc: u32 = 0x826EB560;
    'dispatch: loop {
        match pc {
            0x826EB560 => {
    //   block [0x826EB560..0x826EB5D0)
	// 826EB560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB56C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB570: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB574: 38AA55CC  addi r5, r10, 0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + 21964;
	// 826EB578: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB57C: 390B38BC  addi r8, r11, 0x38bc
	ctx.r[8].s64 = ctx.r[11].s64 + 14524;
	// 826EB580: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EB584: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 826EB588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB58C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB598: 386A55FC  addi r3, r10, 0x55fc
	ctx.r[3].s64 = ctx.r[10].s64 + 22012;
	// 826EB59C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EB5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB5A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB5A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB5B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB5B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB5B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB5BC: 4BD7B865  bl 0x82466e20
	ctx.lr = 0x826EB5C0;
	sub_82466E20(ctx, base);
	// 826EB5C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB5D0 size=108
    let mut pc: u32 = 0x826EB5D0;
    'dispatch: loop {
        match pc {
            0x826EB5D0 => {
    //   block [0x826EB5D0..0x826EB63C)
	// 826EB5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB5DC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB5E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB5E4: 38EB38D4  addi r7, r11, 0x38d4
	ctx.r[7].s64 = ctx.r[11].s64 + 14548;
	// 826EB5E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EB5EC: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 826EB5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB5F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB5F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EB5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB600: 386A562C  addi r3, r10, 0x562c
	ctx.r[3].s64 = ctx.r[10].s64 + 22060;
	// 826EB604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EB608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB61C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EB628: 4BD7B7F9  bl 0x82466e20
	ctx.lr = 0x826EB62C;
	sub_82466E20(ctx, base);
	// 826EB62C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB640 size=112
    let mut pc: u32 = 0x826EB640;
    'dispatch: loop {
        match pc {
            0x826EB640 => {
    //   block [0x826EB640..0x826EB6B0)
	// 826EB640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB64C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB650: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB654: 38AA55FC  addi r5, r10, 0x55fc
	ctx.r[5].s64 = ctx.r[10].s64 + 22012;
	// 826EB658: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB65C: 390B38EC  addi r8, r11, 0x38ec
	ctx.r[8].s64 = ctx.r[11].s64 + 14572;
	// 826EB660: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EB664: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 826EB668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB66C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB678: 386A565C  addi r3, r10, 0x565c
	ctx.r[3].s64 = ctx.r[10].s64 + 22108;
	// 826EB67C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EB680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB69C: 4BD7B785  bl 0x82466e20
	ctx.lr = 0x826EB6A0;
	sub_82466E20(ctx, base);
	// 826EB6A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB6B0 size=116
    let mut pc: u32 = 0x826EB6B0;
    'dispatch: loop {
        match pc {
            0x826EB6B0 => {
    //   block [0x826EB6B0..0x826EB724)
	// 826EB6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB6BC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EB6C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826EB6C4: 390A3908  addi r8, r10, 0x3908
	ctx.r[8].s64 = ctx.r[10].s64 + 14600;
	// 826EB6C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB6CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EB6D0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB6D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB6D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EB6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB6E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB6E4: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 826EB6E8: 396B873C  addi r11, r11, -0x78c4
	ctx.r[11].s64 = ctx.r[11].s64 + -30916;
	// 826EB6EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB6F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB6F4: 386A568C  addi r3, r10, 0x568c
	ctx.r[3].s64 = ctx.r[10].s64 + 22156;
	// 826EB6F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EB6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB700: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EB704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB710: 4BD7B711  bl 0x82466e20
	ctx.lr = 0x826EB714;
	sub_82466E20(ctx, base);
	// 826EB714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB728 size=116
    let mut pc: u32 = 0x826EB728;
    'dispatch: loop {
        match pc {
            0x826EB728 => {
    //   block [0x826EB728..0x826EB79C)
	// 826EB728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB734: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EB738: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826EB73C: 392A8864  addi r9, r10, -0x779c
	ctx.r[9].s64 = ctx.r[10].s64 + -30620;
	// 826EB740: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB744: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 826EB748: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB74C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB750: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826EB754: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB758: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 826EB75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB760: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EB764: 396B39C8  addi r11, r11, 0x39c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14792;
	// 826EB768: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB770: 386A56BC  addi r3, r10, 0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + 22204;
	// 826EB774: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826EB778: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EB77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB780: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 826EB784: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB788: 4BD7B699  bl 0x82466e20
	ctx.lr = 0x826EB78C;
	sub_82466E20(ctx, base);
	// 826EB78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EB7A0 size=48
    let mut pc: u32 = 0x826EB7A0;
    'dispatch: loop {
        match pc {
            0x826EB7A0 => {
    //   block [0x826EB7A0..0x826EB7D0)
	// 826EB7A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB7A4: 814B401C  lwz r10, 0x401c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16412 as u32) ) } as u64;
	// 826EB7A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EB7AC: 396B8268  addi r11, r11, -0x7d98
	ctx.r[11].s64 = ctx.r[11].s64 + -32152;
	// 826EB7B0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826EB7B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EB7B8: 814A4018  lwz r10, 0x4018(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16408 as u32) ) } as u64;
	// 826EB7BC: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826EB7C0: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EB7C4: 814A4014  lwz r10, 0x4014(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16404 as u32) ) } as u64;
	// 826EB7C8: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 826EB7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB7D0 size=116
    let mut pc: u32 = 0x826EB7D0;
    'dispatch: loop {
        match pc {
            0x826EB7D0 => {
    //   block [0x826EB7D0..0x826EB844)
	// 826EB7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB7DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EB7E0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB7E4: 392B8940  addi r9, r11, -0x76c0
	ctx.r[9].s64 = ctx.r[11].s64 + -30400;
	// 826EB7E8: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB7EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB7F0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826EB7F4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 826EB7F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EB7FC: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 826EB800: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB804: 396B8268  addi r11, r11, -0x7d98
	ctx.r[11].s64 = ctx.r[11].s64 + -32152;
	// 826EB808: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EB80C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB810: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EB814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB818: 386A56EC  addi r3, r10, 0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + 22252;
	// 826EB81C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826EB820: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EB824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB828: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EB82C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB830: 4BD7B5F1  bl 0x82466e20
	ctx.lr = 0x826EB834;
	sub_82466E20(ctx, base);
	// 826EB834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB848 size=116
    let mut pc: u32 = 0x826EB848;
    'dispatch: loop {
        match pc {
            0x826EB848 => {
    //   block [0x826EB848..0x826EB8BC)
	// 826EB848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB854: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB858: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EB85C: 390B4028  addi r8, r11, 0x4028
	ctx.r[8].s64 = ctx.r[11].s64 + 16424;
	// 826EB860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB864: 392A8ADC  addi r9, r10, -0x7524
	ctx.r[9].s64 = ctx.r[10].s64 + -29988;
	// 826EB868: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB86C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826EB870: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB874: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB87C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB88C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EB890: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 826EB894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EB898: 386B571C  addi r3, r11, 0x571c
	ctx.r[3].s64 = ctx.r[11].s64 + 22300;
	// 826EB89C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EB8A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB8A8: 4BD7B579  bl 0x82466e20
	ctx.lr = 0x826EB8AC;
	sub_82466E20(ctx, base);
	// 826EB8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB8C0 size=112
    let mut pc: u32 = 0x826EB8C0;
    'dispatch: loop {
        match pc {
            0x826EB8C0 => {
    //   block [0x826EB8C0..0x826EB930)
	// 826EB8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB8CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB8D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB8D4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB8D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB8DC: 390B40B8  addi r8, r11, 0x40b8
	ctx.r[8].s64 = ctx.r[11].s64 + 16568;
	// 826EB8E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EB8E4: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 826EB8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB8EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB8F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EB8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB8F8: 386A574C  addi r3, r10, 0x574c
	ctx.r[3].s64 = ctx.r[10].s64 + 22348;
	// 826EB8FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EB900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EB904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EB908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EB90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EB914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EB91C: 4BD7B505  bl 0x82466e20
	ctx.lr = 0x826EB920;
	sub_82466E20(ctx, base);
	// 826EB920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EB930 size=36
    let mut pc: u32 = 0x826EB930;
    'dispatch: loop {
        match pc {
            0x826EB930 => {
    //   block [0x826EB930..0x826EB954)
	// 826EB930: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB934: 814B40D8  lwz r10, 0x40d8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16600 as u32) ) } as u64;
	// 826EB938: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EB93C: 396B8610  addi r11, r11, -0x79f0
	ctx.r[11].s64 = ctx.r[11].s64 + -31216;
	// 826EB940: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826EB944: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EB948: 814A40D0  lwz r10, 0x40d0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16592 as u32) ) } as u64;
	// 826EB94C: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 826EB950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB958 size=116
    let mut pc: u32 = 0x826EB958;
    'dispatch: loop {
        match pc {
            0x826EB958 => {
    //   block [0x826EB958..0x826EB9CC)
	// 826EB958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB964: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EB968: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826EB96C: 392A8B44  addi r9, r10, -0x74bc
	ctx.r[9].s64 = ctx.r[10].s64 + -29884;
	// 826EB970: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB974: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826EB978: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EB97C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB980: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826EB984: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EB988: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 826EB98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EB990: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EB994: 396B8610  addi r11, r11, -0x79f0
	ctx.r[11].s64 = ctx.r[11].s64 + -31216;
	// 826EB998: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EB9A0: 386A577C  addi r3, r10, 0x577c
	ctx.r[3].s64 = ctx.r[10].s64 + 22396;
	// 826EB9A4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826EB9A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EB9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EB9B0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 826EB9B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EB9B8: 4BD7B469  bl 0x82466e20
	ctx.lr = 0x826EB9BC;
	sub_82466E20(ctx, base);
	// 826EB9BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EB9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EB9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EB9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EB9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EB9D0 size=108
    let mut pc: u32 = 0x826EB9D0;
    'dispatch: loop {
        match pc {
            0x826EB9D0 => {
    //   block [0x826EB9D0..0x826EBA3C)
	// 826EB9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EB9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EB9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EB9DC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EB9E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EB9E4: 38EB40E0  addi r7, r11, 0x40e0
	ctx.r[7].s64 = ctx.r[11].s64 + 16608;
	// 826EB9E8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826EB9EC: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 826EB9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EB9F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EB9F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EB9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBA00: 386A57AC  addi r3, r10, 0x57ac
	ctx.r[3].s64 = ctx.r[10].s64 + 22444;
	// 826EBA04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EBA08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBA24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EBA28: 4BD7B3F9  bl 0x82466e20
	ctx.lr = 0x826EBA2C;
	sub_82466E20(ctx, base);
	// 826EBA2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBA40 size=112
    let mut pc: u32 = 0x826EBA40;
    'dispatch: loop {
        match pc {
            0x826EBA40 => {
    //   block [0x826EBA40..0x826EBAB0)
	// 826EBA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBA4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBA50: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBA54: 38AA34CC  addi r5, r10, 0x34cc
	ctx.r[5].s64 = ctx.r[10].s64 + 13516;
	// 826EBA58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBA5C: 390B4158  addi r8, r11, 0x4158
	ctx.r[8].s64 = ctx.r[11].s64 + 16728;
	// 826EBA60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EBA64: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 826EBA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBA6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBA70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBA78: 386A57DC  addi r3, r10, 0x57dc
	ctx.r[3].s64 = ctx.r[10].s64 + 22492;
	// 826EBA7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBA84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBA9C: 4BD7B385  bl 0x82466e20
	ctx.lr = 0x826EBAA0;
	sub_82466E20(ctx, base);
	// 826EBAA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBAB0 size=108
    let mut pc: u32 = 0x826EBAB0;
    'dispatch: loop {
        match pc {
            0x826EBAB0 => {
    //   block [0x826EBAB0..0x826EBB1C)
	// 826EBAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBABC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBAC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBAC4: 38EB4170  addi r7, r11, 0x4170
	ctx.r[7].s64 = ctx.r[11].s64 + 16752;
	// 826EBAC8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EBACC: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 826EBAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBAD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBAD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EBADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBAE0: 386A580C  addi r3, r10, 0x580c
	ctx.r[3].s64 = ctx.r[10].s64 + 22540;
	// 826EBAE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EBAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EBB08: 4BD7B319  bl 0x82466e20
	ctx.lr = 0x826EBB0C;
	sub_82466E20(ctx, base);
	// 826EBB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBB20 size=112
    let mut pc: u32 = 0x826EBB20;
    'dispatch: loop {
        match pc {
            0x826EBB20 => {
    //   block [0x826EBB20..0x826EBB90)
	// 826EBB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBB2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBB30: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBB34: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EBB38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBB3C: 390B4188  addi r8, r11, 0x4188
	ctx.r[8].s64 = ctx.r[11].s64 + 16776;
	// 826EBB40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EBB44: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 826EBB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBB4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBB50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBB58: 386A583C  addi r3, r10, 0x583c
	ctx.r[3].s64 = ctx.r[10].s64 + 22588;
	// 826EBB5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBB7C: 4BD7B2A5  bl 0x82466e20
	ctx.lr = 0x826EBB80;
	sub_82466E20(ctx, base);
	// 826EBB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBB90 size=108
    let mut pc: u32 = 0x826EBB90;
    'dispatch: loop {
        match pc {
            0x826EBB90 => {
    //   block [0x826EBB90..0x826EBBFC)
	// 826EBB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBB9C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBBA4: 38EB41D0  addi r7, r11, 0x41d0
	ctx.r[7].s64 = ctx.r[11].s64 + 16848;
	// 826EBBA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EBBAC: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 826EBBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBBB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EBBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBBC0: 386A586C  addi r3, r10, 0x586c
	ctx.r[3].s64 = ctx.r[10].s64 + 22636;
	// 826EBBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EBBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EBBE8: 4BD7B239  bl 0x82466e20
	ctx.lr = 0x826EBBEC;
	sub_82466E20(ctx, base);
	// 826EBBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBC00 size=108
    let mut pc: u32 = 0x826EBC00;
    'dispatch: loop {
        match pc {
            0x826EBC00 => {
    //   block [0x826EBC00..0x826EBC6C)
	// 826EBC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBC0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBC14: 38EB4200  addi r7, r11, 0x4200
	ctx.r[7].s64 = ctx.r[11].s64 + 16896;
	// 826EBC18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EBC1C: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 826EBC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBC24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EBC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBC30: 386A589C  addi r3, r10, 0x589c
	ctx.r[3].s64 = ctx.r[10].s64 + 22684;
	// 826EBC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EBC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EBC58: 4BD7B1C9  bl 0x82466e20
	ctx.lr = 0x826EBC5C;
	sub_82466E20(ctx, base);
	// 826EBC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBC70 size=112
    let mut pc: u32 = 0x826EBC70;
    'dispatch: loop {
        match pc {
            0x826EBC70 => {
    //   block [0x826EBC70..0x826EBCE0)
	// 826EBC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBC7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBC80: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBC84: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EBC88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBC8C: 390B4218  addi r8, r11, 0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + 16920;
	// 826EBC90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EBC94: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 826EBC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBCA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBCA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBCA8: 386A58CC  addi r3, r10, 0x58cc
	ctx.r[3].s64 = ctx.r[10].s64 + 22732;
	// 826EBCAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBCB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBCCC: 4BD7B155  bl 0x82466e20
	ctx.lr = 0x826EBCD0;
	sub_82466E20(ctx, base);
	// 826EBCD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBCE0 size=112
    let mut pc: u32 = 0x826EBCE0;
    'dispatch: loop {
        match pc {
            0x826EBCE0 => {
    //   block [0x826EBCE0..0x826EBD50)
	// 826EBCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBCEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBCF0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBCF4: 38AA478C  addi r5, r10, 0x478c
	ctx.r[5].s64 = ctx.r[10].s64 + 18316;
	// 826EBCF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBCFC: 390B4248  addi r8, r11, 0x4248
	ctx.r[8].s64 = ctx.r[11].s64 + 16968;
	// 826EBD00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EBD04: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 826EBD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBD0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBD18: 386A58FC  addi r3, r10, 0x58fc
	ctx.r[3].s64 = ctx.r[10].s64 + 22780;
	// 826EBD1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBD3C: 4BD7B0E5  bl 0x82466e20
	ctx.lr = 0x826EBD40;
	sub_82466E20(ctx, base);
	// 826EBD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBD50 size=112
    let mut pc: u32 = 0x826EBD50;
    'dispatch: loop {
        match pc {
            0x826EBD50 => {
    //   block [0x826EBD50..0x826EBDC0)
	// 826EBD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBD5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBD60: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBD64: 38AA478C  addi r5, r10, 0x478c
	ctx.r[5].s64 = ctx.r[10].s64 + 18316;
	// 826EBD68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBD6C: 390B4290  addi r8, r11, 0x4290
	ctx.r[8].s64 = ctx.r[11].s64 + 17040;
	// 826EBD70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826EBD74: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 826EBD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBD7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBD88: 386A592C  addi r3, r10, 0x592c
	ctx.r[3].s64 = ctx.r[10].s64 + 22828;
	// 826EBD8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBDAC: 4BD7B075  bl 0x82466e20
	ctx.lr = 0x826EBDB0;
	sub_82466E20(ctx, base);
	// 826EBDB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBDC0 size=112
    let mut pc: u32 = 0x826EBDC0;
    'dispatch: loop {
        match pc {
            0x826EBDC0 => {
    //   block [0x826EBDC0..0x826EBE30)
	// 826EBDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBDCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBDD0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBDD4: 38AA47BC  addi r5, r10, 0x47bc
	ctx.r[5].s64 = ctx.r[10].s64 + 18364;
	// 826EBDD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBDDC: 390B42F0  addi r8, r11, 0x42f0
	ctx.r[8].s64 = ctx.r[11].s64 + 17136;
	// 826EBDE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826EBDE4: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 826EBDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBDEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBDF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBDF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBDF8: 386A595C  addi r3, r10, 0x595c
	ctx.r[3].s64 = ctx.r[10].s64 + 22876;
	// 826EBDFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBE1C: 4BD7B005  bl 0x82466e20
	ctx.lr = 0x826EBE20;
	sub_82466E20(ctx, base);
	// 826EBE20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBE30 size=112
    let mut pc: u32 = 0x826EBE30;
    'dispatch: loop {
        match pc {
            0x826EBE30 => {
    //   block [0x826EBE30..0x826EBEA0)
	// 826EBE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBE3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBE40: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBE44: 38AA47BC  addi r5, r10, 0x47bc
	ctx.r[5].s64 = ctx.r[10].s64 + 18364;
	// 826EBE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBE4C: 390B4350  addi r8, r11, 0x4350
	ctx.r[8].s64 = ctx.r[11].s64 + 17232;
	// 826EBE50: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826EBE54: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 826EBE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBE5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBE60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBE68: 386A598C  addi r3, r10, 0x598c
	ctx.r[3].s64 = ctx.r[10].s64 + 22924;
	// 826EBE6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBE8C: 4BD7AF95  bl 0x82466e20
	ctx.lr = 0x826EBE90;
	sub_82466E20(ctx, base);
	// 826EBE90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBEA0 size=112
    let mut pc: u32 = 0x826EBEA0;
    'dispatch: loop {
        match pc {
            0x826EBEA0 => {
    //   block [0x826EBEA0..0x826EBF10)
	// 826EBEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBEAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBEB0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBEB4: 38AA47BC  addi r5, r10, 0x47bc
	ctx.r[5].s64 = ctx.r[10].s64 + 18364;
	// 826EBEB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBEBC: 390B4410  addi r8, r11, 0x4410
	ctx.r[8].s64 = ctx.r[11].s64 + 17424;
	// 826EBEC0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826EBEC4: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 826EBEC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBED8: 386A59BC  addi r3, r10, 0x59bc
	ctx.r[3].s64 = ctx.r[10].s64 + 22972;
	// 826EBEDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBEE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBEE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBEF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBEF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBEFC: 4BD7AF25  bl 0x82466e20
	ctx.lr = 0x826EBF00;
	sub_82466E20(ctx, base);
	// 826EBF00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBF10 size=112
    let mut pc: u32 = 0x826EBF10;
    'dispatch: loop {
        match pc {
            0x826EBF10 => {
    //   block [0x826EBF10..0x826EBF80)
	// 826EBF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBF1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBF20: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EBF24: 38AA478C  addi r5, r10, 0x478c
	ctx.r[5].s64 = ctx.r[10].s64 + 18316;
	// 826EBF28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBF2C: 390B4470  addi r8, r11, 0x4470
	ctx.r[8].s64 = ctx.r[11].s64 + 17520;
	// 826EBF30: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826EBF34: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 826EBF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EBF3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EBF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBF48: 386A59EC  addi r3, r10, 0x59ec
	ctx.r[3].s64 = ctx.r[10].s64 + 23020;
	// 826EBF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EBF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EBF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EBF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBF6C: 4BD7AEB5  bl 0x82466e20
	ctx.lr = 0x826EBF70;
	sub_82466E20(ctx, base);
	// 826EBF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBF80 size=112
    let mut pc: u32 = 0x826EBF80;
    'dispatch: loop {
        match pc {
            0x826EBF80 => {
    //   block [0x826EBF80..0x826EBFF0)
	// 826EBF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBF8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EBF90: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826EBF94: 38EA4530  addi r7, r10, 0x4530
	ctx.r[7].s64 = ctx.r[10].s64 + 17712;
	// 826EBF98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EBF9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EBFA0: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 826EBFA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EBFA8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EBFAC: 396B8B70  addi r11, r11, -0x7490
	ctx.r[11].s64 = ctx.r[11].s64 + -29840;
	// 826EBFB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EBFB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EBFB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EBFBC: 386A5A1C  addi r3, r10, 0x5a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 23068;
	// 826EBFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EBFC4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EBFC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EBFCC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EBFD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EBFD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EBFD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EBFDC: 4BD7AE45  bl 0x82466e20
	ctx.lr = 0x826EBFE0;
	sub_82466E20(ctx, base);
	// 826EBFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EBFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EBFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EBFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EBFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EBFF0 size=112
    let mut pc: u32 = 0x826EBFF0;
    'dispatch: loop {
        match pc {
            0x826EBFF0 => {
    //   block [0x826EBFF0..0x826EC060)
	// 826EBFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EBFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EBFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EBFFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC000: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC004: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826EC008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC00C: 390B46F8  addi r8, r11, 0x46f8
	ctx.r[8].s64 = ctx.r[11].s64 + 18168;
	// 826EC010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC014: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 826EC018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC01C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC028: 386A5A4C  addi r3, r10, 0x5a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 23116;
	// 826EC02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC03C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EC040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC04C: 4BD7ADD5  bl 0x82466e20
	ctx.lr = 0x826EC050;
	sub_82466E20(ctx, base);
	// 826EC050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC060 size=108
    let mut pc: u32 = 0x826EC060;
    'dispatch: loop {
        match pc {
            0x826EC060 => {
    //   block [0x826EC060..0x826EC0CC)
	// 826EC060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC06C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC074: 38EB4710  addi r7, r11, 0x4710
	ctx.r[7].s64 = ctx.r[11].s64 + 18192;
	// 826EC078: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EC07C: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826EC080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC090: 386A5A7C  addi r3, r10, 0x5a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 23164;
	// 826EC094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC0B8: 4BD7AD69  bl 0x82466e20
	ctx.lr = 0x826EC0BC;
	sub_82466E20(ctx, base);
	// 826EC0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC0D0 size=112
    let mut pc: u32 = 0x826EC0D0;
    'dispatch: loop {
        match pc {
            0x826EC0D0 => {
    //   block [0x826EC0D0..0x826EC140)
	// 826EC0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC0DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC0E0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC0E4: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826EC0E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC0EC: 390B4740  addi r8, r11, 0x4740
	ctx.r[8].s64 = ctx.r[11].s64 + 18240;
	// 826EC0F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC0F4: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 826EC0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC0FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC108: 386A5AAC  addi r3, r10, 0x5aac
	ctx.r[3].s64 = ctx.r[10].s64 + 23212;
	// 826EC10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC11C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EC120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC12C: 4BD7ACF5  bl 0x82466e20
	ctx.lr = 0x826EC130;
	sub_82466E20(ctx, base);
	// 826EC130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC140 size=108
    let mut pc: u32 = 0x826EC140;
    'dispatch: loop {
        match pc {
            0x826EC140 => {
    //   block [0x826EC140..0x826EC1AC)
	// 826EC140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC14C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC154: 38EB4758  addi r7, r11, 0x4758
	ctx.r[7].s64 = ctx.r[11].s64 + 18264;
	// 826EC158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EC15C: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826EC160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC170: 386A5ADC  addi r3, r10, 0x5adc
	ctx.r[3].s64 = ctx.r[10].s64 + 23260;
	// 826EC174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC198: 4BD7AC89  bl 0x82466e20
	ctx.lr = 0x826EC19C;
	sub_82466E20(ctx, base);
	// 826EC19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC1B0 size=108
    let mut pc: u32 = 0x826EC1B0;
    'dispatch: loop {
        match pc {
            0x826EC1B0 => {
    //   block [0x826EC1B0..0x826EC21C)
	// 826EC1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC1BC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC1C4: 38EB4788  addi r7, r11, 0x4788
	ctx.r[7].s64 = ctx.r[11].s64 + 18312;
	// 826EC1C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826EC1CC: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 826EC1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC1D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC1E0: 386A5B0C  addi r3, r10, 0x5b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 23308;
	// 826EC1E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC208: 4BD7AC19  bl 0x82466e20
	ctx.lr = 0x826EC20C;
	sub_82466E20(ctx, base);
	// 826EC20C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC220 size=112
    let mut pc: u32 = 0x826EC220;
    'dispatch: loop {
        match pc {
            0x826EC220 => {
    //   block [0x826EC220..0x826EC290)
	// 826EC220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC22C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC230: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC234: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC238: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC23C: 390B47D0  addi r8, r11, 0x47d0
	ctx.r[8].s64 = ctx.r[11].s64 + 18384;
	// 826EC240: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EC244: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 826EC248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC24C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC258: 386A5B3C  addi r3, r10, 0x5b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 23356;
	// 826EC25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC27C: 4BD7ABA5  bl 0x82466e20
	ctx.lr = 0x826EC280;
	sub_82466E20(ctx, base);
	// 826EC280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC290 size=112
    let mut pc: u32 = 0x826EC290;
    'dispatch: loop {
        match pc {
            0x826EC290 => {
    //   block [0x826EC290..0x826EC300)
	// 826EC290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC29C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC2A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC2A4: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826EC2A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC2AC: 390B4818  addi r8, r11, 0x4818
	ctx.r[8].s64 = ctx.r[11].s64 + 18456;
	// 826EC2B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC2B4: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 826EC2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC2BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC2C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC2C8: 386A5B6C  addi r3, r10, 0x5b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 23404;
	// 826EC2CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC2DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EC2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC2EC: 4BD7AB35  bl 0x82466e20
	ctx.lr = 0x826EC2F0;
	sub_82466E20(ctx, base);
	// 826EC2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC300 size=112
    let mut pc: u32 = 0x826EC300;
    'dispatch: loop {
        match pc {
            0x826EC300 => {
    //   block [0x826EC300..0x826EC370)
	// 826EC300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC30C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC310: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC314: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826EC318: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC31C: 390B4830  addi r8, r11, 0x4830
	ctx.r[8].s64 = ctx.r[11].s64 + 18480;
	// 826EC320: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EC324: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 826EC328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC32C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC338: 386A5B9C  addi r3, r10, 0x5b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 23452;
	// 826EC33C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC35C: 4BD7AAC5  bl 0x82466e20
	ctx.lr = 0x826EC360;
	sub_82466E20(ctx, base);
	// 826EC360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC370 size=112
    let mut pc: u32 = 0x826EC370;
    'dispatch: loop {
        match pc {
            0x826EC370 => {
    //   block [0x826EC370..0x826EC3E0)
	// 826EC370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC37C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC380: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC384: 38AA568C  addi r5, r10, 0x568c
	ctx.r[5].s64 = ctx.r[10].s64 + 22156;
	// 826EC388: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC38C: 390B4860  addi r8, r11, 0x4860
	ctx.r[8].s64 = ctx.r[11].s64 + 18528;
	// 826EC390: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC394: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 826EC398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC39C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC3A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC3A8: 386A5BCC  addi r3, r10, 0x5bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 23500;
	// 826EC3AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC3CC: 4BD7AA55  bl 0x82466e20
	ctx.lr = 0x826EC3D0;
	sub_82466E20(ctx, base);
	// 826EC3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC3E0 size=108
    let mut pc: u32 = 0x826EC3E0;
    'dispatch: loop {
        match pc {
            0x826EC3E0 => {
    //   block [0x826EC3E0..0x826EC44C)
	// 826EC3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC3EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC3F4: 38EB4878  addi r7, r11, 0x4878
	ctx.r[7].s64 = ctx.r[11].s64 + 18552;
	// 826EC3F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EC3FC: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 826EC400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC410: 386A5BFC  addi r3, r10, 0x5bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 23548;
	// 826EC414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC438: 4BD7A9E9  bl 0x82466e20
	ctx.lr = 0x826EC43C;
	sub_82466E20(ctx, base);
	// 826EC43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC450 size=112
    let mut pc: u32 = 0x826EC450;
    'dispatch: loop {
        match pc {
            0x826EC450 => {
    //   block [0x826EC450..0x826EC4C0)
	// 826EC450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC45C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC460: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC464: 38AA5BFC  addi r5, r10, 0x5bfc
	ctx.r[5].s64 = ctx.r[10].s64 + 23548;
	// 826EC468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC46C: 390B48A8  addi r8, r11, 0x48a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18600;
	// 826EC470: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EC474: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 826EC478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC47C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC488: 386A5C2C  addi r3, r10, 0x5c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 23596;
	// 826EC48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC4AC: 4BD7A975  bl 0x82466e20
	ctx.lr = 0x826EC4B0;
	sub_82466E20(ctx, base);
	// 826EC4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EC4C0 size=24
    let mut pc: u32 = 0x826EC4C0;
    'dispatch: loop {
        match pc {
            0x826EC4C0 => {
    //   block [0x826EC4C0..0x826EC4D8)
	// 826EC4C0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC4C4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EC4C8: 394A86D0  addi r10, r10, -0x7930
	ctx.r[10].s64 = ctx.r[10].s64 + -31024;
	// 826EC4CC: 816B40DC  lwz r11, 0x40dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16604 as u32) ) } as u64;
	// 826EC4D0: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826EC4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC4D8 size=116
    let mut pc: u32 = 0x826EC4D8;
    'dispatch: loop {
        match pc {
            0x826EC4D8 => {
    //   block [0x826EC4D8..0x826EC54C)
	// 826EC4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC4E4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EC4E8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EC4EC: 390B86D0  addi r8, r11, -0x7930
	ctx.r[8].s64 = ctx.r[11].s64 + -31024;
	// 826EC4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC4F4: 392A8C10  addi r9, r10, -0x73f0
	ctx.r[9].s64 = ctx.r[10].s64 + -29680;
	// 826EC4F8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC4FC: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826EC500: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC504: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC50C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC51C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EC520: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 826EC524: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EC528: 386B5C5C  addi r3, r11, 0x5c5c
	ctx.r[3].s64 = ctx.r[11].s64 + 23644;
	// 826EC52C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EC530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC538: 4BD7A8E9  bl 0x82466e20
	ctx.lr = 0x826EC53C;
	sub_82466E20(ctx, base);
	// 826EC53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC550 size=108
    let mut pc: u32 = 0x826EC550;
    'dispatch: loop {
        match pc {
            0x826EC550 => {
    //   block [0x826EC550..0x826EC5BC)
	// 826EC550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC55C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC560: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC564: 38EB48D8  addi r7, r11, 0x48d8
	ctx.r[7].s64 = ctx.r[11].s64 + 18648;
	// 826EC568: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826EC56C: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 826EC570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC574: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC580: 386A5C8C  addi r3, r10, 0x5c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 23692;
	// 826EC584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC5A8: 4BD7A879  bl 0x82466e20
	ctx.lr = 0x826EC5AC;
	sub_82466E20(ctx, base);
	// 826EC5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC5C0 size=108
    let mut pc: u32 = 0x826EC5C0;
    'dispatch: loop {
        match pc {
            0x826EC5C0 => {
    //   block [0x826EC5C0..0x826EC62C)
	// 826EC5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC5CC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC5D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC5D4: 38EB4920  addi r7, r11, 0x4920
	ctx.r[7].s64 = ctx.r[11].s64 + 18720;
	// 826EC5D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EC5DC: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 826EC5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC5E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC5F0: 386A5CBC  addi r3, r10, 0x5cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 23740;
	// 826EC5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC618: 4BD7A809  bl 0x82466e20
	ctx.lr = 0x826EC61C;
	sub_82466E20(ctx, base);
	// 826EC61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC630 size=108
    let mut pc: u32 = 0x826EC630;
    'dispatch: loop {
        match pc {
            0x826EC630 => {
    //   block [0x826EC630..0x826EC69C)
	// 826EC630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC63C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC644: 38EB4950  addi r7, r11, 0x4950
	ctx.r[7].s64 = ctx.r[11].s64 + 18768;
	// 826EC648: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EC64C: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 826EC650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC654: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC660: 386A5CEC  addi r3, r10, 0x5cec
	ctx.r[3].s64 = ctx.r[10].s64 + 23788;
	// 826EC664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC688: 4BD7A799  bl 0x82466e20
	ctx.lr = 0x826EC68C;
	sub_82466E20(ctx, base);
	// 826EC68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC6A0 size=112
    let mut pc: u32 = 0x826EC6A0;
    'dispatch: loop {
        match pc {
            0x826EC6A0 => {
    //   block [0x826EC6A0..0x826EC710)
	// 826EC6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC6AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC6B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC6B4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC6BC: 390B4980  addi r8, r11, 0x4980
	ctx.r[8].s64 = ctx.r[11].s64 + 18816;
	// 826EC6C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EC6C4: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 826EC6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC6CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC6D8: 386A5D1C  addi r3, r10, 0x5d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 23836;
	// 826EC6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC6FC: 4BD7A725  bl 0x82466e20
	ctx.lr = 0x826EC700;
	sub_82466E20(ctx, base);
	// 826EC700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC710 size=112
    let mut pc: u32 = 0x826EC710;
    'dispatch: loop {
        match pc {
            0x826EC710 => {
    //   block [0x826EC710..0x826EC780)
	// 826EC710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC71C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC720: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC724: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC72C: 390B49B0  addi r8, r11, 0x49b0
	ctx.r[8].s64 = ctx.r[11].s64 + 18864;
	// 826EC730: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC734: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 826EC738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC73C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC748: 386A5D4C  addi r3, r10, 0x5d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 23884;
	// 826EC74C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC76C: 4BD7A6B5  bl 0x82466e20
	ctx.lr = 0x826EC770;
	sub_82466E20(ctx, base);
	// 826EC770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC780 size=112
    let mut pc: u32 = 0x826EC780;
    'dispatch: loop {
        match pc {
            0x826EC780 => {
    //   block [0x826EC780..0x826EC7F0)
	// 826EC780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC78C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC790: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC794: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC79C: 390B49C8  addi r8, r11, 0x49c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18888;
	// 826EC7A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC7A4: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 826EC7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC7AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC7B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC7B8: 386A5D7C  addi r3, r10, 0x5d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 23932;
	// 826EC7BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC7DC: 4BD7A645  bl 0x82466e20
	ctx.lr = 0x826EC7E0;
	sub_82466E20(ctx, base);
	// 826EC7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC7F0 size=108
    let mut pc: u32 = 0x826EC7F0;
    'dispatch: loop {
        match pc {
            0x826EC7F0 => {
    //   block [0x826EC7F0..0x826EC85C)
	// 826EC7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC7FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC804: 38EB49E0  addi r7, r11, 0x49e0
	ctx.r[7].s64 = ctx.r[11].s64 + 18912;
	// 826EC808: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EC80C: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 826EC810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC820: 386A5DAC  addi r3, r10, 0x5dac
	ctx.r[3].s64 = ctx.r[10].s64 + 23980;
	// 826EC824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC848: 4BD7A5D9  bl 0x82466e20
	ctx.lr = 0x826EC84C;
	sub_82466E20(ctx, base);
	// 826EC84C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC860 size=112
    let mut pc: u32 = 0x826EC860;
    'dispatch: loop {
        match pc {
            0x826EC860 => {
    //   block [0x826EC860..0x826EC8D0)
	// 826EC860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC870: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC874: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC87C: 390B4A10  addi r8, r11, 0x4a10
	ctx.r[8].s64 = ctx.r[11].s64 + 18960;
	// 826EC880: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EC884: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 826EC888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC88C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC898: 386A5DDC  addi r3, r10, 0x5ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 24028;
	// 826EC89C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EC8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC8BC: 4BD7A565  bl 0x82466e20
	ctx.lr = 0x826EC8C0;
	sub_82466E20(ctx, base);
	// 826EC8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC8D0 size=108
    let mut pc: u32 = 0x826EC8D0;
    'dispatch: loop {
        match pc {
            0x826EC8D0 => {
    //   block [0x826EC8D0..0x826EC93C)
	// 826EC8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC8DC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC8E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC8E4: 38EB4A28  addi r7, r11, 0x4a28
	ctx.r[7].s64 = ctx.r[11].s64 + 18984;
	// 826EC8E8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826EC8EC: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 826EC8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC8F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC900: 386A5E0C  addi r3, r10, 0x5e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 24076;
	// 826EC904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EC91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EC928: 4BD7A4F9  bl 0x82466e20
	ctx.lr = 0x826EC92C;
	sub_82466E20(ctx, base);
	// 826EC92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC940 size=116
    let mut pc: u32 = 0x826EC940;
    'dispatch: loop {
        match pc {
            0x826EC940 => {
    //   block [0x826EC940..0x826EC9B4)
	// 826EC940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC94C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EC950: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826EC954: 390A4B18  addi r8, r10, 0x4b18
	ctx.r[8].s64 = ctx.r[10].s64 + 19224;
	// 826EC958: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC95C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EC960: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EC964: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC968: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EC96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EC974: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826EC978: 396B8C28  addi r11, r11, -0x73d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29656;
	// 826EC97C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC980: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC984: 386A5E3C  addi r3, r10, 0x5e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 24124;
	// 826EC988: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EC98C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC990: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EC994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EC998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EC99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EC9A0: 4BD7A481  bl 0x82466e20
	ctx.lr = 0x826EC9A4;
	sub_82466E20(ctx, base);
	// 826EC9A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EC9A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EC9AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EC9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EC9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EC9B8 size=108
    let mut pc: u32 = 0x826EC9B8;
    'dispatch: loop {
        match pc {
            0x826EC9B8 => {
    //   block [0x826EC9B8..0x826ECA24)
	// 826EC9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EC9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EC9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EC9C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EC9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EC9CC: 38EB4CE0  addi r7, r11, 0x4ce0
	ctx.r[7].s64 = ctx.r[11].s64 + 19680;
	// 826EC9D0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826EC9D4: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 826EC9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EC9DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EC9E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EC9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EC9E8: 386A5E6C  addi r3, r10, 0x5e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 24172;
	// 826EC9EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EC9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EC9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EC9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EC9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECA0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ECA10: 4BD7A411  bl 0x82466e20
	ctx.lr = 0x826ECA14;
	sub_82466E20(ctx, base);
	// 826ECA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECA28 size=112
    let mut pc: u32 = 0x826ECA28;
    'dispatch: loop {
        match pc {
            0x826ECA28 => {
    //   block [0x826ECA28..0x826ECA98)
	// 826ECA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECA34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECA38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECA3C: 38AA47BC  addi r5, r10, 0x47bc
	ctx.r[5].s64 = ctx.r[10].s64 + 18364;
	// 826ECA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECA44: 390B4E78  addi r8, r11, 0x4e78
	ctx.r[8].s64 = ctx.r[11].s64 + 20088;
	// 826ECA48: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826ECA4C: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826ECA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECA54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ECA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECA60: 386A5E9C  addi r3, r10, 0x5e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 24220;
	// 826ECA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ECA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECA84: 4BD7A39D  bl 0x82466e20
	ctx.lr = 0x826ECA88;
	sub_82466E20(ctx, base);
	// 826ECA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECA98 size=100
    let mut pc: u32 = 0x826ECA98;
    'dispatch: loop {
        match pc {
            0x826ECA98 => {
    //   block [0x826ECA98..0x826ECAFC)
	// 826ECA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECAA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECAAC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ECAB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECAB8: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 826ECABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECACC: 386A5ECC  addi r3, r10, 0x5ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 24268;
	// 826ECAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ECADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ECAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECAE8: 4BD7A339  bl 0x82466e20
	ctx.lr = 0x826ECAEC;
	sub_82466E20(ctx, base);
	// 826ECAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECB00 size=112
    let mut pc: u32 = 0x826ECB00;
    'dispatch: loop {
        match pc {
            0x826ECB00 => {
    //   block [0x826ECB00..0x826ECB70)
	// 826ECB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECB0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECB10: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECB14: 38AA5ECC  addi r5, r10, 0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + 24268;
	// 826ECB18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECB1C: 390B50D0  addi r8, r11, 0x50d0
	ctx.r[8].s64 = ctx.r[11].s64 + 20688;
	// 826ECB20: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826ECB24: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826ECB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECB2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECB30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ECB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECB38: 386A5EFC  addi r3, r10, 0x5efc
	ctx.r[3].s64 = ctx.r[10].s64 + 24316;
	// 826ECB3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ECB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECB5C: 4BD7A2C5  bl 0x82466e20
	ctx.lr = 0x826ECB60;
	sub_82466E20(ctx, base);
	// 826ECB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECB70 size=100
    let mut pc: u32 = 0x826ECB70;
    'dispatch: loop {
        match pc {
            0x826ECB70 => {
    //   block [0x826ECB70..0x826ECBD4)
	// 826ECB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECB7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECB84: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ECB88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECB8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECB90: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 826ECB94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECBA4: 386A5F2C  addi r3, r10, 0x5f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 24364;
	// 826ECBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECBAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECBB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ECBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECBB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ECBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECBC0: 4BD7A261  bl 0x82466e20
	ctx.lr = 0x826ECBC4;
	sub_82466E20(ctx, base);
	// 826ECBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECBD8 size=108
    let mut pc: u32 = 0x826ECBD8;
    'dispatch: loop {
        match pc {
            0x826ECBD8 => {
    //   block [0x826ECBD8..0x826ECC44)
	// 826ECBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECBE4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECBE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECBEC: 38EB5148  addi r7, r11, 0x5148
	ctx.r[7].s64 = ctx.r[11].s64 + 20808;
	// 826ECBF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ECBF4: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 826ECBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECBFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECC00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ECC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECC08: 386A5F5C  addi r3, r10, 0x5f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 24412;
	// 826ECC0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ECC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECC2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ECC30: 4BD7A1F1  bl 0x82466e20
	ctx.lr = 0x826ECC34;
	sub_82466E20(ctx, base);
	// 826ECC34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECC48 size=112
    let mut pc: u32 = 0x826ECC48;
    'dispatch: loop {
        match pc {
            0x826ECC48 => {
    //   block [0x826ECC48..0x826ECCB8)
	// 826ECC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECC58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECC5C: 38AA5F2C  addi r5, r10, 0x5f2c
	ctx.r[5].s64 = ctx.r[10].s64 + 24364;
	// 826ECC60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECC64: 390B5190  addi r8, r11, 0x5190
	ctx.r[8].s64 = ctx.r[11].s64 + 20880;
	// 826ECC68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ECC6C: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826ECC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECC74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ECC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECC80: 386A5F8C  addi r3, r10, 0x5f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 24460;
	// 826ECC84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ECC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECCA4: 4BD7A17D  bl 0x82466e20
	ctx.lr = 0x826ECCA8;
	sub_82466E20(ctx, base);
	// 826ECCA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECCB8 size=100
    let mut pc: u32 = 0x826ECCB8;
    'dispatch: loop {
        match pc {
            0x826ECCB8 => {
    //   block [0x826ECCB8..0x826ECD1C)
	// 826ECCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECCC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECCCC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ECCD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECCD8: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 826ECCDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECCEC: 386A5FBC  addi r3, r10, 0x5fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 24508;
	// 826ECCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECCF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECCF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ECCFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECD00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ECD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECD08: 4BD7A119  bl 0x82466e20
	ctx.lr = 0x826ECD0C;
	sub_82466E20(ctx, base);
	// 826ECD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECD20 size=100
    let mut pc: u32 = 0x826ECD20;
    'dispatch: loop {
        match pc {
            0x826ECD20 => {
    //   block [0x826ECD20..0x826ECD84)
	// 826ECD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECD2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECD34: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ECD38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECD3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECD40: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826ECD44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECD54: 386A5FEC  addi r3, r10, 0x5fec
	ctx.r[3].s64 = ctx.r[10].s64 + 24556;
	// 826ECD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECD5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECD60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ECD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECD68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ECD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECD70: 4BD7A0B1  bl 0x82466e20
	ctx.lr = 0x826ECD74;
	sub_82466E20(ctx, base);
	// 826ECD74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECD88 size=112
    let mut pc: u32 = 0x826ECD88;
    'dispatch: loop {
        match pc {
            0x826ECD88 => {
    //   block [0x826ECD88..0x826ECDF8)
	// 826ECD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECD94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECD98: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECD9C: 38AA5FBC  addi r5, r10, 0x5fbc
	ctx.r[5].s64 = ctx.r[10].s64 + 24508;
	// 826ECDA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECDA4: 390B51C0  addi r8, r11, 0x51c0
	ctx.r[8].s64 = ctx.r[11].s64 + 20928;
	// 826ECDA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826ECDAC: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 826ECDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECDB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ECDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECDC0: 386A601C  addi r3, r10, 0x601c
	ctx.r[3].s64 = ctx.r[10].s64 + 24604;
	// 826ECDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ECDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECDE4: 4BD7A03D  bl 0x82466e20
	ctx.lr = 0x826ECDE8;
	sub_82466E20(ctx, base);
	// 826ECDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECDF8 size=112
    let mut pc: u32 = 0x826ECDF8;
    'dispatch: loop {
        match pc {
            0x826ECDF8 => {
    //   block [0x826ECDF8..0x826ECE68)
	// 826ECDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECE04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECE08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECE0C: 38AA5FEC  addi r5, r10, 0x5fec
	ctx.r[5].s64 = ctx.r[10].s64 + 24556;
	// 826ECE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECE14: 390B5220  addi r8, r11, 0x5220
	ctx.r[8].s64 = ctx.r[11].s64 + 21024;
	// 826ECE18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826ECE1C: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 826ECE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECE24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ECE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECE30: 386A604C  addi r3, r10, 0x604c
	ctx.r[3].s64 = ctx.r[10].s64 + 24652;
	// 826ECE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ECE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECE54: 4BD79FCD  bl 0x82466e20
	ctx.lr = 0x826ECE58;
	sub_82466E20(ctx, base);
	// 826ECE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECE68 size=100
    let mut pc: u32 = 0x826ECE68;
    'dispatch: loop {
        match pc {
            0x826ECE68 => {
    //   block [0x826ECE68..0x826ECECC)
	// 826ECE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECE74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECE78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECE7C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ECE80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECE88: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 826ECE8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECE9C: 386A607C  addi r3, r10, 0x607c
	ctx.r[3].s64 = ctx.r[10].s64 + 24700;
	// 826ECEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECEA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECEA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ECEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECEB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ECEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECEB8: 4BD79F69  bl 0x82466e20
	ctx.lr = 0x826ECEBC;
	sub_82466E20(ctx, base);
	// 826ECEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECED0 size=112
    let mut pc: u32 = 0x826ECED0;
    'dispatch: loop {
        match pc {
            0x826ECED0 => {
    //   block [0x826ECED0..0x826ECF40)
	// 826ECED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECEDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECEE0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECEE4: 38AA607C  addi r5, r10, 0x607c
	ctx.r[5].s64 = ctx.r[10].s64 + 24700;
	// 826ECEE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECEEC: 390B5280  addi r8, r11, 0x5280
	ctx.r[8].s64 = ctx.r[11].s64 + 21120;
	// 826ECEF0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826ECEF4: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 826ECEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECEFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ECF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECF08: 386A60AC  addi r3, r10, 0x60ac
	ctx.r[3].s64 = ctx.r[10].s64 + 24748;
	// 826ECF0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ECF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECF2C: 4BD79EF5  bl 0x82466e20
	ctx.lr = 0x826ECF30;
	sub_82466E20(ctx, base);
	// 826ECF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECF40 size=108
    let mut pc: u32 = 0x826ECF40;
    'dispatch: loop {
        match pc {
            0x826ECF40 => {
    //   block [0x826ECF40..0x826ECFAC)
	// 826ECF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECF4C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECF50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECF54: 38EB5370  addi r7, r11, 0x5370
	ctx.r[7].s64 = ctx.r[11].s64 + 21360;
	// 826ECF58: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826ECF5C: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826ECF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECF64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ECF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECF70: 386A60DC  addi r3, r10, 0x60dc
	ctx.r[3].s64 = ctx.r[10].s64 + 24796;
	// 826ECF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ECF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ECF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ECF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ECF98: 4BD79E89  bl 0x82466e20
	ctx.lr = 0x826ECF9C;
	sub_82466E20(ctx, base);
	// 826ECF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ECFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ECFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ECFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ECFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ECFB0 size=108
    let mut pc: u32 = 0x826ECFB0;
    'dispatch: loop {
        match pc {
            0x826ECFB0 => {
    //   block [0x826ECFB0..0x826ED01C)
	// 826ECFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ECFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ECFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ECFBC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ECFC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ECFC4: 38EB5460  addi r7, r11, 0x5460
	ctx.r[7].s64 = ctx.r[11].s64 + 21600;
	// 826ECFC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ECFCC: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 826ECFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ECFD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ECFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ECFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ECFE0: 386A610C  addi r3, r10, 0x610c
	ctx.r[3].s64 = ctx.r[10].s64 + 24844;
	// 826ECFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ECFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ECFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ECFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ECFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ECFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ECFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED008: 4BD79E19  bl 0x82466e20
	ctx.lr = 0x826ED00C;
	sub_82466E20(ctx, base);
	// 826ED00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED020 size=108
    let mut pc: u32 = 0x826ED020;
    'dispatch: loop {
        match pc {
            0x826ED020 => {
    //   block [0x826ED020..0x826ED08C)
	// 826ED020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED02C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED030: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED034: 38EB54A8  addi r7, r11, 0x54a8
	ctx.r[7].s64 = ctx.r[11].s64 + 21672;
	// 826ED038: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826ED03C: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826ED040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED044: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED050: 386A613C  addi r3, r10, 0x613c
	ctx.r[3].s64 = ctx.r[10].s64 + 24892;
	// 826ED054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED078: 4BD79DA9  bl 0x82466e20
	ctx.lr = 0x826ED07C;
	sub_82466E20(ctx, base);
	// 826ED07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED090 size=108
    let mut pc: u32 = 0x826ED090;
    'dispatch: loop {
        match pc {
            0x826ED090 => {
    //   block [0x826ED090..0x826ED0FC)
	// 826ED090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED09C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED0A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED0A4: 38EB5580  addi r7, r11, 0x5580
	ctx.r[7].s64 = ctx.r[11].s64 + 21888;
	// 826ED0A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ED0AC: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 826ED0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED0B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED0B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED0BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED0C0: 386A616C  addi r3, r10, 0x616c
	ctx.r[3].s64 = ctx.r[10].s64 + 24940;
	// 826ED0C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED0E8: 4BD79D39  bl 0x82466e20
	ctx.lr = 0x826ED0EC;
	sub_82466E20(ctx, base);
	// 826ED0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED100 size=100
    let mut pc: u32 = 0x826ED100;
    'dispatch: loop {
        match pc {
            0x826ED100 => {
    //   block [0x826ED100..0x826ED164)
	// 826ED100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED10C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED114: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED120: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 826ED124: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED134: 386A619C  addi r3, r10, 0x619c
	ctx.r[3].s64 = ctx.r[10].s64 + 24988;
	// 826ED138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED13C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED140: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ED144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED148: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ED14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED150: 4BD79CD1  bl 0x82466e20
	ctx.lr = 0x826ED154;
	sub_82466E20(ctx, base);
	// 826ED154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED168 size=112
    let mut pc: u32 = 0x826ED168;
    'dispatch: loop {
        match pc {
            0x826ED168 => {
    //   block [0x826ED168..0x826ED1D8)
	// 826ED168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED174: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED178: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED17C: 38AA619C  addi r5, r10, 0x619c
	ctx.r[5].s64 = ctx.r[10].s64 + 24988;
	// 826ED180: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED184: 390B5598  addi r8, r11, 0x5598
	ctx.r[8].s64 = ctx.r[11].s64 + 21912;
	// 826ED188: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED18C: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 826ED190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED194: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED1A0: 386A61CC  addi r3, r10, 0x61cc
	ctx.r[3].s64 = ctx.r[10].s64 + 25036;
	// 826ED1A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED1C4: 4BD79C5D  bl 0x82466e20
	ctx.lr = 0x826ED1C8;
	sub_82466E20(ctx, base);
	// 826ED1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED1D8 size=108
    let mut pc: u32 = 0x826ED1D8;
    'dispatch: loop {
        match pc {
            0x826ED1D8 => {
    //   block [0x826ED1D8..0x826ED244)
	// 826ED1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED1E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED1E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED1EC: 38EB55E0  addi r7, r11, 0x55e0
	ctx.r[7].s64 = ctx.r[11].s64 + 21984;
	// 826ED1F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ED1F4: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826ED1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED1FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED208: 386A61FC  addi r3, r10, 0x61fc
	ctx.r[3].s64 = ctx.r[10].s64 + 25084;
	// 826ED20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED230: 4BD79BF1  bl 0x82466e20
	ctx.lr = 0x826ED234;
	sub_82466E20(ctx, base);
	// 826ED234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED248 size=112
    let mut pc: u32 = 0x826ED248;
    'dispatch: loop {
        match pc {
            0x826ED248 => {
    //   block [0x826ED248..0x826ED2B8)
	// 826ED248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED258: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED25C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED264: 390B5628  addi r8, r11, 0x5628
	ctx.r[8].s64 = ctx.r[11].s64 + 22056;
	// 826ED268: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED26C: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826ED270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED280: 386A622C  addi r3, r10, 0x622c
	ctx.r[3].s64 = ctx.r[10].s64 + 25132;
	// 826ED284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED2A4: 4BD79B7D  bl 0x82466e20
	ctx.lr = 0x826ED2A8;
	sub_82466E20(ctx, base);
	// 826ED2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED2B8 size=108
    let mut pc: u32 = 0x826ED2B8;
    'dispatch: loop {
        match pc {
            0x826ED2B8 => {
    //   block [0x826ED2B8..0x826ED324)
	// 826ED2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED2C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED2C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED2CC: 38EB5640  addi r7, r11, 0x5640
	ctx.r[7].s64 = ctx.r[11].s64 + 22080;
	// 826ED2D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ED2D4: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826ED2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED2DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED2E8: 386A625C  addi r3, r10, 0x625c
	ctx.r[3].s64 = ctx.r[10].s64 + 25180;
	// 826ED2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED310: 4BD79B11  bl 0x82466e20
	ctx.lr = 0x826ED314;
	sub_82466E20(ctx, base);
	// 826ED314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED328 size=112
    let mut pc: u32 = 0x826ED328;
    'dispatch: loop {
        match pc {
            0x826ED328 => {
    //   block [0x826ED328..0x826ED398)
	// 826ED328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED338: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED33C: 38AA622C  addi r5, r10, 0x622c
	ctx.r[5].s64 = ctx.r[10].s64 + 25132;
	// 826ED340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED344: 390B5688  addi r8, r11, 0x5688
	ctx.r[8].s64 = ctx.r[11].s64 + 22152;
	// 826ED348: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED34C: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826ED350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED360: 386A628C  addi r3, r10, 0x628c
	ctx.r[3].s64 = ctx.r[10].s64 + 25228;
	// 826ED364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED384: 4BD79A9D  bl 0x82466e20
	ctx.lr = 0x826ED388;
	sub_82466E20(ctx, base);
	// 826ED388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED398 size=100
    let mut pc: u32 = 0x826ED398;
    'dispatch: loop {
        match pc {
            0x826ED398 => {
    //   block [0x826ED398..0x826ED3FC)
	// 826ED398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED3A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED3AC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED3B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED3B8: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826ED3BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED3CC: 386A62BC  addi r3, r10, 0x62bc
	ctx.r[3].s64 = ctx.r[10].s64 + 25276;
	// 826ED3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED3D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED3D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ED3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ED3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED3E8: 4BD79A39  bl 0x82466e20
	ctx.lr = 0x826ED3EC;
	sub_82466E20(ctx, base);
	// 826ED3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED400 size=112
    let mut pc: u32 = 0x826ED400;
    'dispatch: loop {
        match pc {
            0x826ED400 => {
    //   block [0x826ED400..0x826ED470)
	// 826ED400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED40C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED410: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED414: 38AA62BC  addi r5, r10, 0x62bc
	ctx.r[5].s64 = ctx.r[10].s64 + 25276;
	// 826ED418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED41C: 390B56A0  addi r8, r11, 0x56a0
	ctx.r[8].s64 = ctx.r[11].s64 + 22176;
	// 826ED420: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826ED424: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826ED428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED42C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED438: 386A62EC  addi r3, r10, 0x62ec
	ctx.r[3].s64 = ctx.r[10].s64 + 25324;
	// 826ED43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED45C: 4BD799C5  bl 0x82466e20
	ctx.lr = 0x826ED460;
	sub_82466E20(ctx, base);
	// 826ED460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED470 size=108
    let mut pc: u32 = 0x826ED470;
    'dispatch: loop {
        match pc {
            0x826ED470 => {
    //   block [0x826ED470..0x826ED4DC)
	// 826ED470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED47C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED484: 38EB5748  addi r7, r11, 0x5748
	ctx.r[7].s64 = ctx.r[11].s64 + 22344;
	// 826ED488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ED48C: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826ED490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED4A0: 386A631C  addi r3, r10, 0x631c
	ctx.r[3].s64 = ctx.r[10].s64 + 25372;
	// 826ED4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED4C8: 4BD79959  bl 0x82466e20
	ctx.lr = 0x826ED4CC;
	sub_82466E20(ctx, base);
	// 826ED4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED4E0 size=112
    let mut pc: u32 = 0x826ED4E0;
    'dispatch: loop {
        match pc {
            0x826ED4E0 => {
    //   block [0x826ED4E0..0x826ED550)
	// 826ED4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED4EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED4F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED4F4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED4FC: 390B5778  addi r8, r11, 0x5778
	ctx.r[8].s64 = ctx.r[11].s64 + 22392;
	// 826ED500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED504: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826ED508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED50C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED518: 386A634C  addi r3, r10, 0x634c
	ctx.r[3].s64 = ctx.r[10].s64 + 25420;
	// 826ED51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED53C: 4BD798E5  bl 0x82466e20
	ctx.lr = 0x826ED540;
	sub_82466E20(ctx, base);
	// 826ED540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED550 size=112
    let mut pc: u32 = 0x826ED550;
    'dispatch: loop {
        match pc {
            0x826ED550 => {
    //   block [0x826ED550..0x826ED5C0)
	// 826ED550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED55C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED560: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED564: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED56C: 390B57C0  addi r8, r11, 0x57c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22464;
	// 826ED570: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED574: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826ED578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED57C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED588: 386A637C  addi r3, r10, 0x637c
	ctx.r[3].s64 = ctx.r[10].s64 + 25468;
	// 826ED58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED5AC: 4BD79875  bl 0x82466e20
	ctx.lr = 0x826ED5B0;
	sub_82466E20(ctx, base);
	// 826ED5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED5C0 size=100
    let mut pc: u32 = 0x826ED5C0;
    'dispatch: loop {
        match pc {
            0x826ED5C0 => {
    //   block [0x826ED5C0..0x826ED624)
	// 826ED5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED5CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED5D4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED5D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED5E0: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826ED5E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED5F4: 386A63AC  addi r3, r10, 0x63ac
	ctx.r[3].s64 = ctx.r[10].s64 + 25516;
	// 826ED5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED5FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ED604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ED60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED610: 4BD79811  bl 0x82466e20
	ctx.lr = 0x826ED614;
	sub_82466E20(ctx, base);
	// 826ED614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED628 size=112
    let mut pc: u32 = 0x826ED628;
    'dispatch: loop {
        match pc {
            0x826ED628 => {
    //   block [0x826ED628..0x826ED698)
	// 826ED628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED634: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED638: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED63C: 38AA63AC  addi r5, r10, 0x63ac
	ctx.r[5].s64 = ctx.r[10].s64 + 25516;
	// 826ED640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED644: 390B5808  addi r8, r11, 0x5808
	ctx.r[8].s64 = ctx.r[11].s64 + 22536;
	// 826ED648: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED64C: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826ED650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED654: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED660: 386A63DC  addi r3, r10, 0x63dc
	ctx.r[3].s64 = ctx.r[10].s64 + 25564;
	// 826ED664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED684: 4BD7979D  bl 0x82466e20
	ctx.lr = 0x826ED688;
	sub_82466E20(ctx, base);
	// 826ED688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED698 size=112
    let mut pc: u32 = 0x826ED698;
    'dispatch: loop {
        match pc {
            0x826ED698 => {
    //   block [0x826ED698..0x826ED708)
	// 826ED698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED6A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED6A8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED6AC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED6B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED6B4: 390B5850  addi r8, r11, 0x5850
	ctx.r[8].s64 = ctx.r[11].s64 + 22608;
	// 826ED6B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED6BC: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826ED6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED6C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED6D0: 386A640C  addi r3, r10, 0x640c
	ctx.r[3].s64 = ctx.r[10].s64 + 25612;
	// 826ED6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED6F4: 4BD7972D  bl 0x82466e20
	ctx.lr = 0x826ED6F8;
	sub_82466E20(ctx, base);
	// 826ED6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED708 size=112
    let mut pc: u32 = 0x826ED708;
    'dispatch: loop {
        match pc {
            0x826ED708 => {
    //   block [0x826ED708..0x826ED778)
	// 826ED708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED718: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED71C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED724: 390B5868  addi r8, r11, 0x5868
	ctx.r[8].s64 = ctx.r[11].s64 + 22632;
	// 826ED728: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED72C: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826ED730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED740: 386A643C  addi r3, r10, 0x643c
	ctx.r[3].s64 = ctx.r[10].s64 + 25660;
	// 826ED744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED754: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ED758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED764: 4BD796BD  bl 0x82466e20
	ctx.lr = 0x826ED768;
	sub_82466E20(ctx, base);
	// 826ED768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED778 size=112
    let mut pc: u32 = 0x826ED778;
    'dispatch: loop {
        match pc {
            0x826ED778 => {
    //   block [0x826ED778..0x826ED7E8)
	// 826ED778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED788: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED78C: 38AA640C  addi r5, r10, 0x640c
	ctx.r[5].s64 = ctx.r[10].s64 + 25612;
	// 826ED790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED794: 390B5880  addi r8, r11, 0x5880
	ctx.r[8].s64 = ctx.r[11].s64 + 22656;
	// 826ED798: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED79C: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826ED7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED7A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED7B0: 386A646C  addi r3, r10, 0x646c
	ctx.r[3].s64 = ctx.r[10].s64 + 25708;
	// 826ED7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED7D4: 4BD7964D  bl 0x82466e20
	ctx.lr = 0x826ED7D8;
	sub_82466E20(ctx, base);
	// 826ED7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED7E8 size=72
    let mut pc: u32 = 0x826ED7E8;
    'dispatch: loop {
        match pc {
            0x826ED7E8 => {
    //   block [0x826ED7E8..0x826ED830)
	// 826ED7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED7F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED7F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826ED7F8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826ED7FC: 38CB68C8  addi r6, r11, 0x68c8
	ctx.r[6].s64 = ctx.r[11].s64 + 26824;
	// 826ED800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826ED804: 388B8C78  addi r4, r11, -0x7388
	ctx.r[4].s64 = ctx.r[11].s64 + -29576;
	// 826ED808: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826ED80C: 386B649C  addi r3, r11, 0x649c
	ctx.r[3].s64 = ctx.r[11].s64 + 25756;
	// 826ED810: 4BD8E279  bl 0x8247ba88
	ctx.lr = 0x826ED814;
	sub_8247BA88(ctx, base);
	// 826ED814: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826ED818: 386BCF30  addi r3, r11, -0x30d0
	ctx.r[3].s64 = ctx.r[11].s64 + -12496;
	// 826ED81C: 4BE4531D  bl 0x82532b38
	ctx.lr = 0x826ED820;
	sub_82532B38(ctx, base);
	// 826ED820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826ED824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED830 size=108
    let mut pc: u32 = 0x826ED830;
    'dispatch: loop {
        match pc {
            0x826ED830 => {
    //   block [0x826ED830..0x826ED89C)
	// 826ED830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED83C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED844: 38EB8820  addi r7, r11, -0x77e0
	ctx.r[7].s64 = ctx.r[11].s64 + -30688;
	// 826ED848: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826ED84C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826ED850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED854: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED858: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED85C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED860: 386A64B4  addi r3, r10, 0x64b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25780;
	// 826ED864: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED888: 4BD79599  bl 0x82466e20
	ctx.lr = 0x826ED88C;
	sub_82466E20(ctx, base);
	// 826ED88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826ED8A0 size=24
    let mut pc: u32 = 0x826ED8A0;
    'dispatch: loop {
        match pc {
            0x826ED8A0 => {
    //   block [0x826ED8A0..0x826ED8B8)
	// 826ED8A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED8A4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826ED8A8: 394A0E60  addi r10, r10, 0xe60
	ctx.r[10].s64 = ctx.r[10].s64 + 3680;
	// 826ED8AC: 816B8898  lwz r11, -0x7768(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30568 as u32) ) } as u64;
	// 826ED8B0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826ED8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED8B8 size=112
    let mut pc: u32 = 0x826ED8B8;
    'dispatch: loop {
        match pc {
            0x826ED8B8 => {
    //   block [0x826ED8B8..0x826ED928)
	// 826ED8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED8C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826ED8C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED8CC: 392A9414  addi r9, r10, -0x6bec
	ctx.r[9].s64 = ctx.r[10].s64 + -27628;
	// 826ED8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED8D4: 390B0E60  addi r8, r11, 0xe60
	ctx.r[8].s64 = ctx.r[11].s64 + 3680;
	// 826ED8D8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826ED8DC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826ED8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED8E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED8F0: 386A64E4  addi r3, r10, 0x64e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25828;
	// 826ED8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ED8F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ED8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED914: 4BD7950D  bl 0x82466e20
	ctx.lr = 0x826ED918;
	sub_82466E20(ctx, base);
	// 826ED918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED928 size=108
    let mut pc: u32 = 0x826ED928;
    'dispatch: loop {
        match pc {
            0x826ED928 => {
    //   block [0x826ED928..0x826ED994)
	// 826ED928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED934: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED93C: 38EB889C  addi r7, r11, -0x7764
	ctx.r[7].s64 = ctx.r[11].s64 + -30564;
	// 826ED940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ED944: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826ED948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED958: 386A6514  addi r3, r10, 0x6514
	ctx.r[3].s64 = ctx.r[10].s64 + 25876;
	// 826ED95C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED97C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED980: 4BD794A1  bl 0x82466e20
	ctx.lr = 0x826ED984;
	sub_82466E20(ctx, base);
	// 826ED984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED998 size=108
    let mut pc: u32 = 0x826ED998;
    'dispatch: loop {
        match pc {
            0x826ED998 => {
    //   block [0x826ED998..0x826EDA04)
	// 826ED998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED9A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED9AC: 38EB88CC  addi r7, r11, -0x7734
	ctx.r[7].s64 = ctx.r[11].s64 + -30516;
	// 826ED9B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ED9B4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826ED9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED9C8: 386A6544  addi r3, r10, 0x6544
	ctx.r[3].s64 = ctx.r[10].s64 + 25924;
	// 826ED9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED9F0: 4BD79431  bl 0x82466e20
	ctx.lr = 0x826ED9F4;
	sub_82466E20(ctx, base);
	// 826ED9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EDA08 size=24
    let mut pc: u32 = 0x826EDA08;
    'dispatch: loop {
        match pc {
            0x826EDA08 => {
    //   block [0x826EDA08..0x826EDA20)
	// 826EDA08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDA0C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EDA10: 394A0EA8  addi r10, r10, 0xea8
	ctx.r[10].s64 = ctx.r[10].s64 + 3752;
	// 826EDA14: 816B88FC  lwz r11, -0x7704(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30468 as u32) ) } as u64;
	// 826EDA18: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826EDA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDA20 size=116
    let mut pc: u32 = 0x826EDA20;
    'dispatch: loop {
        match pc {
            0x826EDA20 => {
    //   block [0x826EDA20..0x826EDA94)
	// 826EDA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDA2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDA30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EDA34: 390B0EA8  addi r8, r11, 0xea8
	ctx.r[8].s64 = ctx.r[11].s64 + 3752;
	// 826EDA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDA3C: 392A9448  addi r9, r10, -0x6bb8
	ctx.r[9].s64 = ctx.r[10].s64 + -27576;
	// 826EDA40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDA44: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826EDA48: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EDA4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDA54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDA64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EDA68: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826EDA6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EDA70: 386B6574  addi r3, r11, 0x6574
	ctx.r[3].s64 = ctx.r[11].s64 + 25972;
	// 826EDA74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EDA78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDA80: 4BD793A1  bl 0x82466e20
	ctx.lr = 0x826EDA84;
	sub_82466E20(ctx, base);
	// 826EDA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDA98 size=108
    let mut pc: u32 = 0x826EDA98;
    'dispatch: loop {
        match pc {
            0x826EDA98 => {
    //   block [0x826EDA98..0x826EDB04)
	// 826EDA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDAA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDAAC: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 826EDAB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDAB4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826EDAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDAC8: 386A65A4  addi r3, r10, 0x65a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26020;
	// 826EDACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDAF0: 4BD79331  bl 0x82466e20
	ctx.lr = 0x826EDAF4;
	sub_82466E20(ctx, base);
	// 826EDAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDB08 size=112
    let mut pc: u32 = 0x826EDB08;
    'dispatch: loop {
        match pc {
            0x826EDB08 => {
    //   block [0x826EDB08..0x826EDB78)
	// 826EDB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDB14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDB18: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDB1C: 38AA6574  addi r5, r10, 0x6574
	ctx.r[5].s64 = ctx.r[10].s64 + 25972;
	// 826EDB20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDB24: 390B8990  addi r8, r11, -0x7670
	ctx.r[8].s64 = ctx.r[11].s64 + -30320;
	// 826EDB28: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826EDB2C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826EDB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDB34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDB38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDB40: 386A65D4  addi r3, r10, 0x65d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26068;
	// 826EDB44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EDB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDB54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDB64: 4BD792BD  bl 0x82466e20
	ctx.lr = 0x826EDB68;
	sub_82466E20(ctx, base);
	// 826EDB68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDB78 size=112
    let mut pc: u32 = 0x826EDB78;
    'dispatch: loop {
        match pc {
            0x826EDB78 => {
    //   block [0x826EDB78..0x826EDBE8)
	// 826EDB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDB84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDB88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDB8C: 38AA6574  addi r5, r10, 0x6574
	ctx.r[5].s64 = ctx.r[10].s64 + 25972;
	// 826EDB90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDB94: 390B8AB0  addi r8, r11, -0x7550
	ctx.r[8].s64 = ctx.r[11].s64 + -30032;
	// 826EDB98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EDB9C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826EDBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDBA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDBB0: 386A6604  addi r3, r10, 0x6604
	ctx.r[3].s64 = ctx.r[10].s64 + 26116;
	// 826EDBB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EDBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDBC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDBD4: 4BD7924D  bl 0x82466e20
	ctx.lr = 0x826EDBD8;
	sub_82466E20(ctx, base);
	// 826EDBD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDBE8 size=108
    let mut pc: u32 = 0x826EDBE8;
    'dispatch: loop {
        match pc {
            0x826EDBE8 => {
    //   block [0x826EDBE8..0x826EDC54)
	// 826EDBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDBF4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDBFC: 38EB8AC8  addi r7, r11, -0x7538
	ctx.r[7].s64 = ctx.r[11].s64 + -30008;
	// 826EDC00: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDC04: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826EDC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDC0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDC10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDC18: 386A6634  addi r3, r10, 0x6634
	ctx.r[3].s64 = ctx.r[10].s64 + 26164;
	// 826EDC1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDC24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDC3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDC40: 4BD791E1  bl 0x82466e20
	ctx.lr = 0x826EDC44;
	sub_82466E20(ctx, base);
	// 826EDC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDC58 size=112
    let mut pc: u32 = 0x826EDC58;
    'dispatch: loop {
        match pc {
            0x826EDC58 => {
    //   block [0x826EDC58..0x826EDCC8)
	// 826EDC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDC64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDC68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDC6C: 38AA6574  addi r5, r10, 0x6574
	ctx.r[5].s64 = ctx.r[10].s64 + 25972;
	// 826EDC70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDC74: 390B8B58  addi r8, r11, -0x74a8
	ctx.r[8].s64 = ctx.r[11].s64 + -29864;
	// 826EDC78: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826EDC7C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826EDC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDC84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDC88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDC90: 386A6664  addi r3, r10, 0x6664
	ctx.r[3].s64 = ctx.r[10].s64 + 26212;
	// 826EDC94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EDC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDCA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDCA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDCAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDCB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDCB4: 4BD7916D  bl 0x82466e20
	ctx.lr = 0x826EDCB8;
	sub_82466E20(ctx, base);
	// 826EDCB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDCC8 size=108
    let mut pc: u32 = 0x826EDCC8;
    'dispatch: loop {
        match pc {
            0x826EDCC8 => {
    //   block [0x826EDCC8..0x826EDD34)
	// 826EDCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDCD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDCDC: 38EB8C48  addi r7, r11, -0x73b8
	ctx.r[7].s64 = ctx.r[11].s64 + -29624;
	// 826EDCE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EDCE4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826EDCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDCEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDCF8: 386A6694  addi r3, r10, 0x6694
	ctx.r[3].s64 = ctx.r[10].s64 + 26260;
	// 826EDCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDD20: 4BD79101  bl 0x82466e20
	ctx.lr = 0x826EDD24;
	sub_82466E20(ctx, base);
	// 826EDD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDD38 size=108
    let mut pc: u32 = 0x826EDD38;
    'dispatch: loop {
        match pc {
            0x826EDD38 => {
    //   block [0x826EDD38..0x826EDDA4)
	// 826EDD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDD44: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDD48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDD4C: 38EB8C60  addi r7, r11, -0x73a0
	ctx.r[7].s64 = ctx.r[11].s64 + -29600;
	// 826EDD50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EDD54: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826EDD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDD5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDD60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDD68: 386A66C4  addi r3, r10, 0x66c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26308;
	// 826EDD6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDD74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDD84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDD8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDD90: 4BD79091  bl 0x82466e20
	ctx.lr = 0x826EDD94;
	sub_82466E20(ctx, base);
	// 826EDD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDDA8 size=116
    let mut pc: u32 = 0x826EDDA8;
    'dispatch: loop {
        match pc {
            0x826EDDA8 => {
    //   block [0x826EDDA8..0x826EDE1C)
	// 826EDDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDDB4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDDB8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EDDBC: 390B8CC4  addi r8, r11, -0x733c
	ctx.r[8].s64 = ctx.r[11].s64 + -29500;
	// 826EDDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDDC4: 392A9474  addi r9, r10, -0x6b8c
	ctx.r[9].s64 = ctx.r[10].s64 + -27532;
	// 826EDDC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDDCC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826EDDD0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EDDD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDDDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDDEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EDDF0: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826EDDF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EDDF8: 386B66F4  addi r3, r11, 0x66f4
	ctx.r[3].s64 = ctx.r[11].s64 + 26356;
	// 826EDDFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EDE00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDE08: 4BD79019  bl 0x82466e20
	ctx.lr = 0x826EDE0C;
	sub_82466E20(ctx, base);
	// 826EDE0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDE20 size=108
    let mut pc: u32 = 0x826EDE20;
    'dispatch: loop {
        match pc {
            0x826EDE20 => {
    //   block [0x826EDE20..0x826EDE8C)
	// 826EDE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDE2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDE30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDE34: 38EB8CE0  addi r7, r11, -0x7320
	ctx.r[7].s64 = ctx.r[11].s64 + -29472;
	// 826EDE38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826EDE3C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826EDE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDE48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDE50: 386A6724  addi r3, r10, 0x6724
	ctx.r[3].s64 = ctx.r[10].s64 + 26404;
	// 826EDE54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDE74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDE78: 4BD78FA9  bl 0x82466e20
	ctx.lr = 0x826EDE7C;
	sub_82466E20(ctx, base);
	// 826EDE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDE90 size=108
    let mut pc: u32 = 0x826EDE90;
    'dispatch: loop {
        match pc {
            0x826EDE90 => {
    //   block [0x826EDE90..0x826EDEFC)
	// 826EDE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDE9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDEA4: 38EB8D28  addi r7, r11, -0x72d8
	ctx.r[7].s64 = ctx.r[11].s64 + -29400;
	// 826EDEA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDEAC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826EDEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDEB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDEB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDEC0: 386A6754  addi r3, r10, 0x6754
	ctx.r[3].s64 = ctx.r[10].s64 + 26452;
	// 826EDEC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDEE8: 4BD78F39  bl 0x82466e20
	ctx.lr = 0x826EDEEC;
	sub_82466E20(ctx, base);
	// 826EDEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDF00 size=108
    let mut pc: u32 = 0x826EDF00;
    'dispatch: loop {
        match pc {
            0x826EDF00 => {
    //   block [0x826EDF00..0x826EDF6C)
	// 826EDF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDF0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDF10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDF14: 38EB8DB8  addi r7, r11, -0x7248
	ctx.r[7].s64 = ctx.r[11].s64 + -29256;
	// 826EDF18: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDF1C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826EDF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDF24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDF28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDF30: 386A6784  addi r3, r10, 0x6784
	ctx.r[3].s64 = ctx.r[10].s64 + 26500;
	// 826EDF34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDF3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDF54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDF58: 4BD78EC9  bl 0x82466e20
	ctx.lr = 0x826EDF5C;
	sub_82466E20(ctx, base);
	// 826EDF5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDF70 size=100
    let mut pc: u32 = 0x826EDF70;
    'dispatch: loop {
        match pc {
            0x826EDF70 => {
    //   block [0x826EDF70..0x826EDFD4)
	// 826EDF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDF7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDF84: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EDF88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDF90: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826EDF94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDFA4: 386A67B4  addi r3, r10, 0x67b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26548;
	// 826EDFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDFB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EDFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDFB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EDFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDFC0: 4BD78E61  bl 0x82466e20
	ctx.lr = 0x826EDFC4;
	sub_82466E20(ctx, base);
	// 826EDFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDFD8 size=112
    let mut pc: u32 = 0x826EDFD8;
    'dispatch: loop {
        match pc {
            0x826EDFD8 => {
    //   block [0x826EDFD8..0x826EE048)
	// 826EDFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDFE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDFE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDFEC: 38AA67B4  addi r5, r10, 0x67b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26548;
	// 826EDFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDFF4: 390B8E48  addi r8, r11, -0x71b8
	ctx.r[8].s64 = ctx.r[11].s64 + -29112;
	// 826EDFF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826EDFFC: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826EE000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE010: 386A67E4  addi r3, r10, 0x67e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26596;
	// 826EE014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE034: 4BD78DED  bl 0x82466e20
	ctx.lr = 0x826EE038;
	sub_82466E20(ctx, base);
	// 826EE038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE048 size=108
    let mut pc: u32 = 0x826EE048;
    'dispatch: loop {
        match pc {
            0x826EE048 => {
    //   block [0x826EE048..0x826EE0B4)
	// 826EE048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE054: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE05C: 38EB8EA8  addi r7, r11, -0x7158
	ctx.r[7].s64 = ctx.r[11].s64 + -29016;
	// 826EE060: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EE064: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826EE068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE06C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE078: 386A6814  addi r3, r10, 0x6814
	ctx.r[3].s64 = ctx.r[10].s64 + 26644;
	// 826EE07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE0A0: 4BD78D81  bl 0x82466e20
	ctx.lr = 0x826EE0A4;
	sub_82466E20(ctx, base);
	// 826EE0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE0B8 size=108
    let mut pc: u32 = 0x826EE0B8;
    'dispatch: loop {
        match pc {
            0x826EE0B8 => {
    //   block [0x826EE0B8..0x826EE124)
	// 826EE0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE0C4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE0C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE0CC: 38EB8ED8  addi r7, r11, -0x7128
	ctx.r[7].s64 = ctx.r[11].s64 + -28968;
	// 826EE0D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EE0D4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826EE0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE0DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE0E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE0E8: 386A6844  addi r3, r10, 0x6844
	ctx.r[3].s64 = ctx.r[10].s64 + 26692;
	// 826EE0EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE110: 4BD78D11  bl 0x82466e20
	ctx.lr = 0x826EE114;
	sub_82466E20(ctx, base);
	// 826EE114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE128 size=108
    let mut pc: u32 = 0x826EE128;
    'dispatch: loop {
        match pc {
            0x826EE128 => {
    //   block [0x826EE128..0x826EE194)
	// 826EE128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE134: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE13C: 38EB8F38  addi r7, r11, -0x70c8
	ctx.r[7].s64 = ctx.r[11].s64 + -28872;
	// 826EE140: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EE144: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826EE148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE14C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE158: 386A6874  addi r3, r10, 0x6874
	ctx.r[3].s64 = ctx.r[10].s64 + 26740;
	// 826EE15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE180: 4BD78CA1  bl 0x82466e20
	ctx.lr = 0x826EE184;
	sub_82466E20(ctx, base);
	// 826EE184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EE198 size=24
    let mut pc: u32 = 0x826EE198;
    'dispatch: loop {
        match pc {
            0x826EE198 => {
    //   block [0x826EE198..0x826EE1B0)
	// 826EE198: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE19C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EE1A0: 394A0F20  addi r10, r10, 0xf20
	ctx.r[10].s64 = ctx.r[10].s64 + 3872;
	// 826EE1A4: 816B8CDC  lwz r11, -0x7324(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29476 as u32) ) } as u64;
	// 826EE1A8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 826EE1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE1B0 size=116
    let mut pc: u32 = 0x826EE1B0;
    'dispatch: loop {
        match pc {
            0x826EE1B0 => {
    //   block [0x826EE1B0..0x826EE224)
	// 826EE1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE1BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE1C0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EE1C4: 390B0F20  addi r8, r11, 0xf20
	ctx.r[8].s64 = ctx.r[11].s64 + 3872;
	// 826EE1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE1CC: 392A94A8  addi r9, r10, -0x6b58
	ctx.r[9].s64 = ctx.r[10].s64 + -27480;
	// 826EE1D0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE1D4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826EE1D8: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE1DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE1E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE1F4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EE1F8: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826EE1FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE200: 386B68A4  addi r3, r11, 0x68a4
	ctx.r[3].s64 = ctx.r[11].s64 + 26788;
	// 826EE204: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE208: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE210: 4BD78C11  bl 0x82466e20
	ctx.lr = 0x826EE214;
	sub_82466E20(ctx, base);
	// 826EE214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE228 size=112
    let mut pc: u32 = 0x826EE228;
    'dispatch: loop {
        match pc {
            0x826EE228 => {
    //   block [0x826EE228..0x826EE298)
	// 826EE228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE234: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE238: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE23C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE244: 390B8F98  addi r8, r11, -0x7068
	ctx.r[8].s64 = ctx.r[11].s64 + -28776;
	// 826EE248: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EE24C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 826EE250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE260: 386A68D4  addi r3, r10, 0x68d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26836;
	// 826EE264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE284: 4BD78B9D  bl 0x82466e20
	ctx.lr = 0x826EE288;
	sub_82466E20(ctx, base);
	// 826EE288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE298 size=112
    let mut pc: u32 = 0x826EE298;
    'dispatch: loop {
        match pc {
            0x826EE298 => {
    //   block [0x826EE298..0x826EE308)
	// 826EE298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE2A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE2A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE2AC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE2B4: 390B8FE0  addi r8, r11, -0x7020
	ctx.r[8].s64 = ctx.r[11].s64 + -28704;
	// 826EE2B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EE2BC: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 826EE2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE2C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE2D0: 386A6904  addi r3, r10, 0x6904
	ctx.r[3].s64 = ctx.r[10].s64 + 26884;
	// 826EE2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE2F4: 4BD78B2D  bl 0x82466e20
	ctx.lr = 0x826EE2F8;
	sub_82466E20(ctx, base);
	// 826EE2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE308 size=112
    let mut pc: u32 = 0x826EE308;
    'dispatch: loop {
        match pc {
            0x826EE308 => {
    //   block [0x826EE308..0x826EE378)
	// 826EE308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE318: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE31C: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EE320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE324: 390B9028  addi r8, r11, -0x6fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -28632;
	// 826EE328: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826EE32C: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826EE330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE340: 386A6934  addi r3, r10, 0x6934
	ctx.r[3].s64 = ctx.r[10].s64 + 26932;
	// 826EE344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE364: 4BD78ABD  bl 0x82466e20
	ctx.lr = 0x826EE368;
	sub_82466E20(ctx, base);
	// 826EE368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE378 size=108
    let mut pc: u32 = 0x826EE378;
    'dispatch: loop {
        match pc {
            0x826EE378 => {
    //   block [0x826EE378..0x826EE3E4)
	// 826EE378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE384: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE38C: 38EB9118  addi r7, r11, -0x6ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -28392;
	// 826EE390: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EE394: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826EE398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE39C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE3A8: 386A6964  addi r3, r10, 0x6964
	ctx.r[3].s64 = ctx.r[10].s64 + 26980;
	// 826EE3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE3D0: 4BD78A51  bl 0x82466e20
	ctx.lr = 0x826EE3D4;
	sub_82466E20(ctx, base);
	// 826EE3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE3E8 size=108
    let mut pc: u32 = 0x826EE3E8;
    'dispatch: loop {
        match pc {
            0x826EE3E8 => {
    //   block [0x826EE3E8..0x826EE454)
	// 826EE3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE3F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE3F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE3FC: 38EB9178  addi r7, r11, -0x6e88
	ctx.r[7].s64 = ctx.r[11].s64 + -28296;
	// 826EE400: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826EE404: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826EE408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE40C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE418: 386A6994  addi r3, r10, 0x6994
	ctx.r[3].s64 = ctx.r[10].s64 + 27028;
	// 826EE41C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE43C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE440: 4BD789E1  bl 0x82466e20
	ctx.lr = 0x826EE444;
	sub_82466E20(ctx, base);
	// 826EE444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE458 size=112
    let mut pc: u32 = 0x826EE458;
    'dispatch: loop {
        match pc {
            0x826EE458 => {
    //   block [0x826EE458..0x826EE4C8)
	// 826EE458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE464: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EE468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE46C: 392B94DC  addi r9, r11, -0x6b24
	ctx.r[9].s64 = ctx.r[11].s64 + -27428;
	// 826EE470: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826EE474: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826EE478: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE47C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826EE480: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE484: 396B9228  addi r11, r11, -0x6dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -28120;
	// 826EE488: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EE48C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE490: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EE494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE498: 386A69C4  addi r3, r10, 0x69c4
	ctx.r[3].s64 = ctx.r[10].s64 + 27076;
	// 826EE49C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE4A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EE4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE4A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EE4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE4B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EE4B4: 4BD7896D  bl 0x82466e20
	ctx.lr = 0x826EE4B8;
	sub_82466E20(ctx, base);
	// 826EE4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE4C8 size=112
    let mut pc: u32 = 0x826EE4C8;
    'dispatch: loop {
        match pc {
            0x826EE4C8 => {
    //   block [0x826EE4C8..0x826EE538)
	// 826EE4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE4D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE4D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE4DC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE4E4: 390B9378  addi r8, r11, -0x6c88
	ctx.r[8].s64 = ctx.r[11].s64 + -27784;
	// 826EE4E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826EE4EC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826EE4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE4F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE500: 386A69F4  addi r3, r10, 0x69f4
	ctx.r[3].s64 = ctx.r[10].s64 + 27124;
	// 826EE504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE524: 4BD788FD  bl 0x82466e20
	ctx.lr = 0x826EE528;
	sub_82466E20(ctx, base);
	// 826EE528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE538 size=112
    let mut pc: u32 = 0x826EE538;
    'dispatch: loop {
        match pc {
            0x826EE538 => {
    //   block [0x826EE538..0x826EE5A8)
	// 826EE538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE548: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE54C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE554: 390B9420  addi r8, r11, -0x6be0
	ctx.r[8].s64 = ctx.r[11].s64 + -27616;
	// 826EE558: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826EE55C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826EE560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE570: 386A6A24  addi r3, r10, 0x6a24
	ctx.r[3].s64 = ctx.r[10].s64 + 27172;
	// 826EE574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE594: 4BD7888D  bl 0x82466e20
	ctx.lr = 0x826EE598;
	sub_82466E20(ctx, base);
	// 826EE598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE5A8 size=112
    let mut pc: u32 = 0x826EE5A8;
    'dispatch: loop {
        match pc {
            0x826EE5A8 => {
    //   block [0x826EE5A8..0x826EE618)
	// 826EE5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE5B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE5B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE5BC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE5C4: 390B94B0  addi r8, r11, -0x6b50
	ctx.r[8].s64 = ctx.r[11].s64 + -27472;
	// 826EE5C8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826EE5CC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826EE5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE5D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE5E0: 386A6A54  addi r3, r10, 0x6a54
	ctx.r[3].s64 = ctx.r[10].s64 + 27220;
	// 826EE5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE604: 4BD7881D  bl 0x82466e20
	ctx.lr = 0x826EE608;
	sub_82466E20(ctx, base);
	// 826EE608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE618 size=108
    let mut pc: u32 = 0x826EE618;
    'dispatch: loop {
        match pc {
            0x826EE618 => {
    //   block [0x826EE618..0x826EE684)
	// 826EE618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE624: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE62C: 38EB9528  addi r7, r11, -0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -27352;
	// 826EE630: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826EE634: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826EE638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE63C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE648: 386A6A84  addi r3, r10, 0x6a84
	ctx.r[3].s64 = ctx.r[10].s64 + 27268;
	// 826EE64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE670: 4BD787B1  bl 0x82466e20
	ctx.lr = 0x826EE674;
	sub_82466E20(ctx, base);
	// 826EE674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE688 size=112
    let mut pc: u32 = 0x826EE688;
    'dispatch: loop {
        match pc {
            0x826EE688 => {
    //   block [0x826EE688..0x826EE6F8)
	// 826EE688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE694: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EE698: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE69C: 392A9538  addi r9, r10, -0x6ac8
	ctx.r[9].s64 = ctx.r[10].s64 + -27336;
	// 826EE6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE6A4: 390B95D0  addi r8, r11, -0x6a30
	ctx.r[8].s64 = ctx.r[11].s64 + -27184;
	// 826EE6A8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826EE6AC: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826EE6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE6B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE6BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE6C0: 386A6AB4  addi r3, r10, 0x6ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 27316;
	// 826EE6C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE6C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE6E4: 4BD7873D  bl 0x82466e20
	ctx.lr = 0x826EE6E8;
	sub_82466E20(ctx, base);
	// 826EE6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE6F8 size=100
    let mut pc: u32 = 0x826EE6F8;
    'dispatch: loop {
        match pc {
            0x826EE6F8 => {
    //   block [0x826EE6F8..0x826EE75C)
	// 826EE6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE704: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE70C: 38AA7294  addi r5, r10, 0x7294
	ctx.r[5].s64 = ctx.r[10].s64 + 29332;
	// 826EE710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE718: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826EE71C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE72C: 386A6AE4  addi r3, r10, 0x6ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 27364;
	// 826EE730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EE73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EE744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE748: 4BD786D9  bl 0x82466e20
	ctx.lr = 0x826EE74C;
	sub_82466E20(ctx, base);
	// 826EE74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE760 size=108
    let mut pc: u32 = 0x826EE760;
    'dispatch: loop {
        match pc {
            0x826EE760 => {
    //   block [0x826EE760..0x826EE7CC)
	// 826EE760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE76C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE774: 38EB9604  addi r7, r11, -0x69fc
	ctx.r[7].s64 = ctx.r[11].s64 + -27132;
	// 826EE778: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EE77C: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 826EE780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE790: 386A6B14  addi r3, r10, 0x6b14
	ctx.r[3].s64 = ctx.r[10].s64 + 27412;
	// 826EE794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE7B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE7B8: 4BD78669  bl 0x82466e20
	ctx.lr = 0x826EE7BC;
	sub_82466E20(ctx, base);
	// 826EE7BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE7D0 size=112
    let mut pc: u32 = 0x826EE7D0;
    'dispatch: loop {
        match pc {
            0x826EE7D0 => {
    //   block [0x826EE7D0..0x826EE840)
	// 826EE7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE7DC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EE7E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE7E4: 392A9598  addi r9, r10, -0x6a68
	ctx.r[9].s64 = ctx.r[10].s64 + -27240;
	// 826EE7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE7EC: 390B9638  addi r8, r11, -0x69c8
	ctx.r[8].s64 = ctx.r[11].s64 + -27080;
	// 826EE7F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826EE7F4: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826EE7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE7FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE808: 386A6B44  addi r3, r10, 0x6b44
	ctx.r[3].s64 = ctx.r[10].s64 + 27460;
	// 826EE80C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE810: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE82C: 4BD785F5  bl 0x82466e20
	ctx.lr = 0x826EE830;
	sub_82466E20(ctx, base);
	// 826EE830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE840 size=112
    let mut pc: u32 = 0x826EE840;
    'dispatch: loop {
        match pc {
            0x826EE840 => {
    //   block [0x826EE840..0x826EE8B0)
	// 826EE840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE84C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE850: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE854: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EE858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE85C: 390B96B0  addi r8, r11, -0x6950
	ctx.r[8].s64 = ctx.r[11].s64 + -26960;
	// 826EE860: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EE864: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826EE868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE878: 386A6B74  addi r3, r10, 0x6b74
	ctx.r[3].s64 = ctx.r[10].s64 + 27508;
	// 826EE87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE89C: 4BD78585  bl 0x82466e20
	ctx.lr = 0x826EE8A0;
	sub_82466E20(ctx, base);
	// 826EE8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE8B0 size=116
    let mut pc: u32 = 0x826EE8B0;
    'dispatch: loop {
        match pc {
            0x826EE8B0 => {
    //   block [0x826EE8B0..0x826EE924)
	// 826EE8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE8BC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EE8C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826EE8C4: 390A96E0  addi r8, r10, -0x6920
	ctx.r[8].s64 = ctx.r[10].s64 + -26912;
	// 826EE8C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE8CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EE8D0: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EE8D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE8D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE8E4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826EE8E8: 396B95AC  addi r11, r11, -0x6a54
	ctx.r[11].s64 = ctx.r[11].s64 + -27220;
	// 826EE8EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE8F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE8F4: 386A6BA4  addi r3, r10, 0x6ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 27556;
	// 826EE8F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EE8FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE900: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EE904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE910: 4BD78511  bl 0x82466e20
	ctx.lr = 0x826EE914;
	sub_82466E20(ctx, base);
	// 826EE914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE928 size=100
    let mut pc: u32 = 0x826EE928;
    'dispatch: loop {
        match pc {
            0x826EE928 => {
    //   block [0x826EE928..0x826EE98C)
	// 826EE928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE93C: 38AA6BA4  addi r5, r10, 0x6ba4
	ctx.r[5].s64 = ctx.r[10].s64 + 27556;
	// 826EE940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE948: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826EE94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE95C: 386A6BD4  addi r3, r10, 0x6bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27604;
	// 826EE960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE968: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EE96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EE974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE978: 4BD784A9  bl 0x82466e20
	ctx.lr = 0x826EE97C;
	sub_82466E20(ctx, base);
	// 826EE97C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EE990 size=24
    let mut pc: u32 = 0x826EE990;
    'dispatch: loop {
        match pc {
            0x826EE990 => {
    //   block [0x826EE990..0x826EE9A8)
	// 826EE990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE994: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EE998: 394A1058  addi r10, r10, 0x1058
	ctx.r[10].s64 = ctx.r[10].s64 + 4184;
	// 826EE99C: 816B9634  lwz r11, -0x69cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27084 as u32) ) } as u64;
	// 826EE9A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826EE9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE9A8 size=116
    let mut pc: u32 = 0x826EE9A8;
    'dispatch: loop {
        match pc {
            0x826EE9A8 => {
    //   block [0x826EE9A8..0x826EEA1C)
	// 826EE9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE9B4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EE9B8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE9BC: 392B95E8  addi r9, r11, -0x6a18
	ctx.r[9].s64 = ctx.r[11].s64 + -27160;
	// 826EE9C0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE9C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE9C8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826EE9CC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826EE9D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE9D4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826EE9D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE9DC: 396B1058  addi r11, r11, 0x1058
	ctx.r[11].s64 = ctx.r[11].s64 + 4184;
	// 826EE9E0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EE9E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE9E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EE9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE9F0: 386A6C04  addi r3, r10, 0x6c04
	ctx.r[3].s64 = ctx.r[10].s64 + 27652;
	// 826EE9F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE9F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EE9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEA00: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EEA04: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EEA08: 4BD78419  bl 0x82466e20
	ctx.lr = 0x826EEA0C;
	sub_82466E20(ctx, base);
	// 826EEA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEA20 size=116
    let mut pc: u32 = 0x826EEA20;
    'dispatch: loop {
        match pc {
            0x826EEA20 => {
    //   block [0x826EEA20..0x826EEA94)
	// 826EEA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEA2C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EEA30: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEA34: 392B963C  addi r9, r11, -0x69c4
	ctx.r[9].s64 = ctx.r[11].s64 + -27076;
	// 826EEA38: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EEA3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEA40: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826EEA44: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826EEA48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEA4C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826EEA50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEA54: 396B9790  addi r11, r11, -0x6870
	ctx.r[11].s64 = ctx.r[11].s64 + -26736;
	// 826EEA58: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EEA5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEA60: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EEA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEA68: 386A6C34  addi r3, r10, 0x6c34
	ctx.r[3].s64 = ctx.r[10].s64 + 27700;
	// 826EEA6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EEA70: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EEA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEA78: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EEA7C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EEA80: 4BD783A1  bl 0x82466e20
	ctx.lr = 0x826EEA84;
	sub_82466E20(ctx, base);
	// 826EEA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEA98 size=108
    let mut pc: u32 = 0x826EEA98;
    'dispatch: loop {
        match pc {
            0x826EEA98 => {
    //   block [0x826EEA98..0x826EEB04)
	// 826EEA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEAA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEAAC: 38EB9868  addi r7, r11, -0x6798
	ctx.r[7].s64 = ctx.r[11].s64 + -26520;
	// 826EEAB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EEAB4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826EEAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEAC8: 386A6C64  addi r3, r10, 0x6c64
	ctx.r[3].s64 = ctx.r[10].s64 + 27748;
	// 826EEACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEAF0: 4BD78331  bl 0x82466e20
	ctx.lr = 0x826EEAF4;
	sub_82466E20(ctx, base);
	// 826EEAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EEB08 size=24
    let mut pc: u32 = 0x826EEB08;
    'dispatch: loop {
        match pc {
            0x826EEB08 => {
    //   block [0x826EEB08..0x826EEB20)
	// 826EEB08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEB0C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EEB10: 394A1100  addi r10, r10, 0x1100
	ctx.r[10].s64 = ctx.r[10].s64 + 4352;
	// 826EEB14: 816B978C  lwz r11, -0x6874(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26740 as u32) ) } as u64;
	// 826EEB18: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826EEB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEB20 size=116
    let mut pc: u32 = 0x826EEB20;
    'dispatch: loop {
        match pc {
            0x826EEB20 => {
    //   block [0x826EEB20..0x826EEB94)
	// 826EEB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEB2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEB30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EEB34: 390B1100  addi r8, r11, 0x1100
	ctx.r[8].s64 = ctx.r[11].s64 + 4352;
	// 826EEB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEB3C: 392A96B0  addi r9, r10, -0x6950
	ctx.r[9].s64 = ctx.r[10].s64 + -26960;
	// 826EEB40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEB44: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826EEB48: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EEB4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEB54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEB64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EEB68: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826EEB6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EEB70: 386B6C94  addi r3, r11, 0x6c94
	ctx.r[3].s64 = ctx.r[11].s64 + 27796;
	// 826EEB74: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826EEB78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEB80: 4BD782A1  bl 0x82466e20
	ctx.lr = 0x826EEB84;
	sub_82466E20(ctx, base);
	// 826EEB84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEB98 size=112
    let mut pc: u32 = 0x826EEB98;
    'dispatch: loop {
        match pc {
            0x826EEB98 => {
    //   block [0x826EEB98..0x826EEC08)
	// 826EEB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEBA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEBAC: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EEBB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEBB4: 390B98CC  addi r8, r11, -0x6734
	ctx.r[8].s64 = ctx.r[11].s64 + -26420;
	// 826EEBB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EEBBC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826EEBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEBC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEBD0: 386A6CC4  addi r3, r10, 0x6cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 27844;
	// 826EEBD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EEBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEBF4: 4BD7822D  bl 0x82466e20
	ctx.lr = 0x826EEBF8;
	sub_82466E20(ctx, base);
	// 826EEBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EEC08 size=24
    let mut pc: u32 = 0x826EEC08;
    'dispatch: loop {
        match pc {
            0x826EEC08 => {
    //   block [0x826EEC08..0x826EEC20)
	// 826EEC08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEC0C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EEC10: 394A1298  addi r10, r10, 0x1298
	ctx.r[10].s64 = ctx.r[10].s64 + 4760;
	// 826EEC14: 816B98FC  lwz r11, -0x6704(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26372 as u32) ) } as u64;
	// 826EEC18: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826EEC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEC20 size=116
    let mut pc: u32 = 0x826EEC20;
    'dispatch: loop {
        match pc {
            0x826EEC20 => {
    //   block [0x826EEC20..0x826EEC94)
	// 826EEC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEC2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEC30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EEC34: 390B1298  addi r8, r11, 0x1298
	ctx.r[8].s64 = ctx.r[11].s64 + 4760;
	// 826EEC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEC3C: 392A96E8  addi r9, r10, -0x6918
	ctx.r[9].s64 = ctx.r[10].s64 + -26904;
	// 826EEC40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEC44: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826EEC48: 38AA6C34  addi r5, r10, 0x6c34
	ctx.r[5].s64 = ctx.r[10].s64 + 27700;
	// 826EEC4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEC54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEC64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EEC68: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826EEC6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EEC70: 386B6CF4  addi r3, r11, 0x6cf4
	ctx.r[3].s64 = ctx.r[11].s64 + 27892;
	// 826EEC74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EEC78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEC80: 4BD781A1  bl 0x82466e20
	ctx.lr = 0x826EEC84;
	sub_82466E20(ctx, base);
	// 826EEC84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEC98 size=112
    let mut pc: u32 = 0x826EEC98;
    'dispatch: loop {
        match pc {
            0x826EEC98 => {
    //   block [0x826EEC98..0x826EED08)
	// 826EEC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EECA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EECA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EECA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EECAC: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EECB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EECB4: 390B9900  addi r8, r11, -0x6700
	ctx.r[8].s64 = ctx.r[11].s64 + -26368;
	// 826EECB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EECBC: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826EECC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EECC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EECC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EECCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EECD0: 386A6D24  addi r3, r10, 0x6d24
	ctx.r[3].s64 = ctx.r[10].s64 + 27940;
	// 826EECD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EECD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EECDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EECE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EECE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EECE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EECEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EECF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EECF4: 4BD7812D  bl 0x82466e20
	ctx.lr = 0x826EECF8;
	sub_82466E20(ctx, base);
	// 826EECF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EECFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EED00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EED04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EED08 size=100
    let mut pc: u32 = 0x826EED08;
    'dispatch: loop {
        match pc {
            0x826EED08 => {
    //   block [0x826EED08..0x826EED6C)
	// 826EED08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EED0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EED10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EED14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EED18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EED1C: 38AA7294  addi r5, r10, 0x7294
	ctx.r[5].s64 = ctx.r[10].s64 + 29332;
	// 826EED20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EED24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EED28: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826EED2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EED30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EED34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EED38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EED3C: 386A6D54  addi r3, r10, 0x6d54
	ctx.r[3].s64 = ctx.r[10].s64 + 27988;
	// 826EED40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EED44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EED48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EED4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EED50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EED54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EED58: 4BD780C9  bl 0x82466e20
	ctx.lr = 0x826EED5C;
	sub_82466E20(ctx, base);
	// 826EED5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EED60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EED64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EED68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EED70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EED70 size=108
    let mut pc: u32 = 0x826EED70;
    'dispatch: loop {
        match pc {
            0x826EED70 => {
    //   block [0x826EED70..0x826EEDDC)
	// 826EED70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EED74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EED78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EED7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EED80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EED84: 38EB9918  addi r7, r11, -0x66e8
	ctx.r[7].s64 = ctx.r[11].s64 + -26344;
	// 826EED88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826EED8C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826EED90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EED94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EED98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EED9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEDA0: 386A6D84  addi r3, r10, 0x6d84
	ctx.r[3].s64 = ctx.r[10].s64 + 28036;
	// 826EEDA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEDB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEDB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEDC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEDC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEDC8: 4BD78059  bl 0x82466e20
	ctx.lr = 0x826EEDCC;
	sub_82466E20(ctx, base);
	// 826EEDCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEDE0 size=112
    let mut pc: u32 = 0x826EEDE0;
    'dispatch: loop {
        match pc {
            0x826EEDE0 => {
    //   block [0x826EEDE0..0x826EEE50)
	// 826EEDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEDEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEDF0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEDF4: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EEDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEDFC: 390B99F0  addi r8, r11, -0x6610
	ctx.r[8].s64 = ctx.r[11].s64 + -26128;
	// 826EEE00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EEE04: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826EEE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEE0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEE18: 386A6DB4  addi r3, r10, 0x6db4
	ctx.r[3].s64 = ctx.r[10].s64 + 28084;
	// 826EEE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EEE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEE3C: 4BD77FE5  bl 0x82466e20
	ctx.lr = 0x826EEE40;
	sub_82466E20(ctx, base);
	// 826EEE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEE50 size=108
    let mut pc: u32 = 0x826EEE50;
    'dispatch: loop {
        match pc {
            0x826EEE50 => {
    //   block [0x826EEE50..0x826EEEBC)
	// 826EEE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEE5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEE64: 38EB9A20  addi r7, r11, -0x65e0
	ctx.r[7].s64 = ctx.r[11].s64 + -26080;
	// 826EEE68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EEE6C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826EEE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEE74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEE80: 386A6DE4  addi r3, r10, 0x6de4
	ctx.r[3].s64 = ctx.r[10].s64 + 28132;
	// 826EEE84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEEA8: 4BD77F79  bl 0x82466e20
	ctx.lr = 0x826EEEAC;
	sub_82466E20(ctx, base);
	// 826EEEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEEC0 size=112
    let mut pc: u32 = 0x826EEEC0;
    'dispatch: loop {
        match pc {
            0x826EEEC0 => {
    //   block [0x826EEEC0..0x826EEF30)
	// 826EEEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEED0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEED4: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EEED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEEDC: 390B9A50  addi r8, r11, -0x65b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26032;
	// 826EEEE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EEEE4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826EEEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEEEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEEF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEEF8: 386A6E14  addi r3, r10, 0x6e14
	ctx.r[3].s64 = ctx.r[10].s64 + 28180;
	// 826EEEFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EEF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEF1C: 4BD77F05  bl 0x82466e20
	ctx.lr = 0x826EEF20;
	sub_82466E20(ctx, base);
	// 826EEF20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEF30 size=112
    let mut pc: u32 = 0x826EEF30;
    'dispatch: loop {
        match pc {
            0x826EEF30 => {
    //   block [0x826EEF30..0x826EEFA0)
	// 826EEF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEF3C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EEF40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826EEF44: 38EA9A68  addi r7, r10, -0x6598
	ctx.r[7].s64 = ctx.r[10].s64 + -26008;
	// 826EEF48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEF4C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EEF50: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826EEF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEF58: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEF5C: 396B96FC  addi r11, r11, -0x6904
	ctx.r[11].s64 = ctx.r[11].s64 + -26884;
	// 826EEF60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEF64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEF68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEF6C: 386A6E44  addi r3, r10, 0x6e44
	ctx.r[3].s64 = ctx.r[10].s64 + 28228;
	// 826EEF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEF74: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EEF78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEF7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EEF80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEF84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEF88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEF8C: 4BD77E95  bl 0x82466e20
	ctx.lr = 0x826EEF90;
	sub_82466E20(ctx, base);
	// 826EEF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEFA0 size=108
    let mut pc: u32 = 0x826EEFA0;
    'dispatch: loop {
        match pc {
            0x826EEFA0 => {
    //   block [0x826EEFA0..0x826EF00C)
	// 826EEFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEFAC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEFB4: 38EB9B40  addi r7, r11, -0x64c0
	ctx.r[7].s64 = ctx.r[11].s64 + -25792;
	// 826EEFB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EEFBC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826EEFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEFC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEFD0: 386A6E74  addi r3, r10, 0x6e74
	ctx.r[3].s64 = ctx.r[10].s64 + 28276;
	// 826EEFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEFF8: 4BD77E29  bl 0x82466e20
	ctx.lr = 0x826EEFFC;
	sub_82466E20(ctx, base);
	// 826EEFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF010 size=108
    let mut pc: u32 = 0x826EF010;
    'dispatch: loop {
        match pc {
            0x826EF010 => {
    //   block [0x826EF010..0x826EF07C)
	// 826EF010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF01C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF024: 38EB9B58  addi r7, r11, -0x64a8
	ctx.r[7].s64 = ctx.r[11].s64 + -25768;
	// 826EF028: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826EF02C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826EF030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF034: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF040: 386A6EA4  addi r3, r10, 0x6ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 28324;
	// 826EF044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF068: 4BD77DB9  bl 0x82466e20
	ctx.lr = 0x826EF06C;
	sub_82466E20(ctx, base);
	// 826EF06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF080 size=108
    let mut pc: u32 = 0x826EF080;
    'dispatch: loop {
        match pc {
            0x826EF080 => {
    //   block [0x826EF080..0x826EF0EC)
	// 826EF080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF08C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF094: 38EB9C60  addi r7, r11, -0x63a0
	ctx.r[7].s64 = ctx.r[11].s64 + -25504;
	// 826EF098: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EF09C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826EF0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF0A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF0A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF0B0: 386A6ED4  addi r3, r10, 0x6ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 28372;
	// 826EF0B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF0D8: 4BD77D49  bl 0x82466e20
	ctx.lr = 0x826EF0DC;
	sub_82466E20(ctx, base);
	// 826EF0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF0F0 size=112
    let mut pc: u32 = 0x826EF0F0;
    'dispatch: loop {
        match pc {
            0x826EF0F0 => {
    //   block [0x826EF0F0..0x826EF160)
	// 826EF0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF0FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF100: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF104: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF10C: 390B9CC0  addi r8, r11, -0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + -25408;
	// 826EF110: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826EF114: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826EF118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF11C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF128: 386A6F04  addi r3, r10, 0x6f04
	ctx.r[3].s64 = ctx.r[10].s64 + 28420;
	// 826EF12C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF14C: 4BD77CD5  bl 0x82466e20
	ctx.lr = 0x826EF150;
	sub_82466E20(ctx, base);
	// 826EF150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF160 size=112
    let mut pc: u32 = 0x826EF160;
    'dispatch: loop {
        match pc {
            0x826EF160 => {
    //   block [0x826EF160..0x826EF1D0)
	// 826EF160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF16C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF170: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF174: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF17C: 390B9DE0  addi r8, r11, -0x6220
	ctx.r[8].s64 = ctx.r[11].s64 + -25120;
	// 826EF180: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF184: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826EF188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF18C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF190: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF198: 386A6F34  addi r3, r10, 0x6f34
	ctx.r[3].s64 = ctx.r[10].s64 + 28468;
	// 826EF19C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF1A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF1A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF1B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF1BC: 4BD77C65  bl 0x82466e20
	ctx.lr = 0x826EF1C0;
	sub_82466E20(ctx, base);
	// 826EF1C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF1D0 size=116
    let mut pc: u32 = 0x826EF1D0;
    'dispatch: loop {
        match pc {
            0x826EF1D0 => {
    //   block [0x826EF1D0..0x826EF244)
	// 826EF1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF1DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF1E0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826EF1E4: 390A9DF8  addi r8, r10, -0x6208
	ctx.r[8].s64 = ctx.r[10].s64 + -25096;
	// 826EF1E8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF1EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF1F0: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF1F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF1F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF204: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826EF208: 396B972C  addi r11, r11, -0x68d4
	ctx.r[11].s64 = ctx.r[11].s64 + -26836;
	// 826EF20C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF214: 386A6F64  addi r3, r10, 0x6f64
	ctx.r[3].s64 = ctx.r[10].s64 + 28516;
	// 826EF218: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF21C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF220: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF230: 4BD77BF1  bl 0x82466e20
	ctx.lr = 0x826EF234;
	sub_82466E20(ctx, base);
	// 826EF234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF248 size=108
    let mut pc: u32 = 0x826EF248;
    'dispatch: loop {
        match pc {
            0x826EF248 => {
    //   block [0x826EF248..0x826EF2B4)
	// 826EF248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF254: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EF25C: 38EB9E58  addi r7, r11, -0x61a8
	ctx.r[7].s64 = ctx.r[11].s64 + -25000;
	// 826EF260: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826EF264: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826EF268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF26C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF278: 386A6F94  addi r3, r10, 0x6f94
	ctx.r[3].s64 = ctx.r[10].s64 + 28564;
	// 826EF27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF2A0: 4BD77B81  bl 0x82466e20
	ctx.lr = 0x826EF2A4;
	sub_82466E20(ctx, base);
	// 826EF2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF2B8 size=112
    let mut pc: u32 = 0x826EF2B8;
    'dispatch: loop {
        match pc {
            0x826EF2B8 => {
    //   block [0x826EF2B8..0x826EF328)
	// 826EF2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF2C4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF2C8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826EF2CC: 38EA9F00  addi r7, r10, -0x6100
	ctx.r[7].s64 = ctx.r[10].s64 + -24832;
	// 826EF2D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EF2D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF2D8: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826EF2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF2E0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF2E4: 396B9740  addi r11, r11, -0x68c0
	ctx.r[11].s64 = ctx.r[11].s64 + -26816;
	// 826EF2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF2EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF2F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF2F4: 386A6FC4  addi r3, r10, 0x6fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28612;
	// 826EF2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF2FC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF304: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF308: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF30C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF314: 4BD77B0D  bl 0x82466e20
	ctx.lr = 0x826EF318;
	sub_82466E20(ctx, base);
	// 826EF318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF328 size=112
    let mut pc: u32 = 0x826EF328;
    'dispatch: loop {
        match pc {
            0x826EF328 => {
    //   block [0x826EF328..0x826EF398)
	// 826EF328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF338: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF33C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF344: 390B9F78  addi r8, r11, -0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + -24712;
	// 826EF348: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EF34C: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826EF350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF360: 386A6FF4  addi r3, r10, 0x6ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 28660;
	// 826EF364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF384: 4BD77A9D  bl 0x82466e20
	ctx.lr = 0x826EF388;
	sub_82466E20(ctx, base);
	// 826EF388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF398 size=112
    let mut pc: u32 = 0x826EF398;
    'dispatch: loop {
        match pc {
            0x826EF398 => {
    //   block [0x826EF398..0x826EF408)
	// 826EF398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF3A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF3A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF3AC: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF3B4: 390B9FC0  addi r8, r11, -0x6040
	ctx.r[8].s64 = ctx.r[11].s64 + -24640;
	// 826EF3B8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826EF3BC: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826EF3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF3C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF3D0: 386A7024  addi r3, r10, 0x7024
	ctx.r[3].s64 = ctx.r[10].s64 + 28708;
	// 826EF3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF3F4: 4BD77A2D  bl 0x82466e20
	ctx.lr = 0x826EF3F8;
	sub_82466E20(ctx, base);
	// 826EF3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF408 size=112
    let mut pc: u32 = 0x826EF408;
    'dispatch: loop {
        match pc {
            0x826EF408 => {
    //   block [0x826EF408..0x826EF478)
	// 826EF408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF418: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF41C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF424: 390BA0C8  addi r8, r11, -0x5f38
	ctx.r[8].s64 = ctx.r[11].s64 + -24376;
	// 826EF428: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF42C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826EF430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF434: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF440: 386A7054  addi r3, r10, 0x7054
	ctx.r[3].s64 = ctx.r[10].s64 + 28756;
	// 826EF444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF464: 4BD779BD  bl 0x82466e20
	ctx.lr = 0x826EF468;
	sub_82466E20(ctx, base);
	// 826EF468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF478 size=112
    let mut pc: u32 = 0x826EF478;
    'dispatch: loop {
        match pc {
            0x826EF478 => {
    //   block [0x826EF478..0x826EF4E8)
	// 826EF478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF488: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF48C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EF490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF494: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 826EF498: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EF49C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826EF4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF4A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF4B0: 386A7084  addi r3, r10, 0x7084
	ctx.r[3].s64 = ctx.r[10].s64 + 28804;
	// 826EF4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF4D4: 4BD7794D  bl 0x82466e20
	ctx.lr = 0x826EF4D8;
	sub_82466E20(ctx, base);
	// 826EF4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF4E8 size=108
    let mut pc: u32 = 0x826EF4E8;
    'dispatch: loop {
        match pc {
            0x826EF4E8 => {
    //   block [0x826EF4E8..0x826EF554)
	// 826EF4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF4F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF4FC: 38EBA110  addi r7, r11, -0x5ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -24304;
	// 826EF500: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826EF504: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826EF508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF50C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF518: 386A70B4  addi r3, r10, 0x70b4
	ctx.r[3].s64 = ctx.r[10].s64 + 28852;
	// 826EF51C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF53C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF540: 4BD778E1  bl 0x82466e20
	ctx.lr = 0x826EF544;
	sub_82466E20(ctx, base);
	// 826EF544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EF558 size=24
    let mut pc: u32 = 0x826EF558;
    'dispatch: loop {
        match pc {
            0x826EF558 => {
    //   block [0x826EF558..0x826EF570)
	// 826EF558: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF55C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF560: 394A1388  addi r10, r10, 0x1388
	ctx.r[10].s64 = ctx.r[10].s64 + 5000;
	// 826EF564: 816BA188  lwz r11, -0x5e78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24184 as u32) ) } as u64;
	// 826EF568: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826EF56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF570 size=116
    let mut pc: u32 = 0x826EF570;
    'dispatch: loop {
        match pc {
            0x826EF570 => {
    //   block [0x826EF570..0x826EF5E4)
	// 826EF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF57C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF580: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EF584: 390B1388  addi r8, r11, 0x1388
	ctx.r[8].s64 = ctx.r[11].s64 + 5000;
	// 826EF588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF58C: 392A9778  addi r9, r10, -0x6888
	ctx.r[9].s64 = ctx.r[10].s64 + -26760;
	// 826EF590: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF594: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826EF598: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF59C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF5A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF5B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EF5B8: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826EF5BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF5C0: 386B70E4  addi r3, r11, 0x70e4
	ctx.r[3].s64 = ctx.r[11].s64 + 28900;
	// 826EF5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EF5C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF5D0: 4BD77851  bl 0x82466e20
	ctx.lr = 0x826EF5D4;
	sub_82466E20(ctx, base);
	// 826EF5D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF5E8 size=112
    let mut pc: u32 = 0x826EF5E8;
    'dispatch: loop {
        match pc {
            0x826EF5E8 => {
    //   block [0x826EF5E8..0x826EF658)
	// 826EF5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF5F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF5F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF5FC: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF604: 390BA18C  addi r8, r11, -0x5e74
	ctx.r[8].s64 = ctx.r[11].s64 + -24180;
	// 826EF608: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EF60C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826EF610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF620: 386A7114  addi r3, r10, 0x7114
	ctx.r[3].s64 = ctx.r[10].s64 + 28948;
	// 826EF624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF644: 4BD777DD  bl 0x82466e20
	ctx.lr = 0x826EF648;
	sub_82466E20(ctx, base);
	// 826EF648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF658 size=116
    let mut pc: u32 = 0x826EF658;
    'dispatch: loop {
        match pc {
            0x826EF658 => {
    //   block [0x826EF658..0x826EF6CC)
	// 826EF658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF664: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF668: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826EF66C: 390AA1C0  addi r8, r10, -0x5e40
	ctx.r[8].s64 = ctx.r[10].s64 + -24128;
	// 826EF670: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF674: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF678: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF67C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF680: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF68C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826EF690: 396B978C  addi r11, r11, -0x6874
	ctx.r[11].s64 = ctx.r[11].s64 + -26740;
	// 826EF694: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF698: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF69C: 386A7144  addi r3, r10, 0x7144
	ctx.r[3].s64 = ctx.r[10].s64 + 28996;
	// 826EF6A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF6A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF6A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF6B8: 4BD77769  bl 0x82466e20
	ctx.lr = 0x826EF6BC;
	sub_82466E20(ctx, base);
	// 826EF6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF6D0 size=112
    let mut pc: u32 = 0x826EF6D0;
    'dispatch: loop {
        match pc {
            0x826EF6D0 => {
    //   block [0x826EF6D0..0x826EF740)
	// 826EF6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF6DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF6E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF6E4: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF6E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF6EC: 390BA280  addi r8, r11, -0x5d80
	ctx.r[8].s64 = ctx.r[11].s64 + -23936;
	// 826EF6F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF6F4: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826EF6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF6FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF708: 386A7174  addi r3, r10, 0x7174
	ctx.r[3].s64 = ctx.r[10].s64 + 29044;
	// 826EF70C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF72C: 4BD776F5  bl 0x82466e20
	ctx.lr = 0x826EF730;
	sub_82466E20(ctx, base);
	// 826EF730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF740 size=108
    let mut pc: u32 = 0x826EF740;
    'dispatch: loop {
        match pc {
            0x826EF740 => {
    //   block [0x826EF740..0x826EF7AC)
	// 826EF740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF74C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF754: 38EBA298  addi r7, r11, -0x5d68
	ctx.r[7].s64 = ctx.r[11].s64 + -23912;
	// 826EF758: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826EF75C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826EF760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF770: 386A71A4  addi r3, r10, 0x71a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29092;
	// 826EF774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF798: 4BD77689  bl 0x82466e20
	ctx.lr = 0x826EF79C;
	sub_82466E20(ctx, base);
	// 826EF79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF7B0 size=116
    let mut pc: u32 = 0x826EF7B0;
    'dispatch: loop {
        match pc {
            0x826EF7B0 => {
    //   block [0x826EF7B0..0x826EF824)
	// 826EF7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF7BC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF7C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826EF7C4: 390AA3D0  addi r8, r10, -0x5c30
	ctx.r[8].s64 = ctx.r[10].s64 + -23600;
	// 826EF7C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF7CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF7D0: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF7D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF7D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF7E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF7E4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826EF7E8: 396B97B0  addi r11, r11, -0x6850
	ctx.r[11].s64 = ctx.r[11].s64 + -26704;
	// 826EF7EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF7F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF7F4: 386A71D4  addi r3, r10, 0x71d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29140;
	// 826EF7F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF7FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF800: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF810: 4BD77611  bl 0x82466e20
	ctx.lr = 0x826EF814;
	sub_82466E20(ctx, base);
	// 826EF814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF828 size=112
    let mut pc: u32 = 0x826EF828;
    'dispatch: loop {
        match pc {
            0x826EF828 => {
    //   block [0x826EF828..0x826EF898)
	// 826EF828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF834: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF838: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF83C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF844: 390BA478  addi r8, r11, -0x5b88
	ctx.r[8].s64 = ctx.r[11].s64 + -23432;
	// 826EF848: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF84C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826EF850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF854: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF860: 386A7204  addi r3, r10, 0x7204
	ctx.r[3].s64 = ctx.r[10].s64 + 29188;
	// 826EF864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF884: 4BD7759D  bl 0x82466e20
	ctx.lr = 0x826EF888;
	sub_82466E20(ctx, base);
	// 826EF888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF898 size=112
    let mut pc: u32 = 0x826EF898;
    'dispatch: loop {
        match pc {
            0x826EF898 => {
    //   block [0x826EF898..0x826EF908)
	// 826EF898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF8A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF8A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF8AC: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF8B4: 390BA490  addi r8, r11, -0x5b70
	ctx.r[8].s64 = ctx.r[11].s64 + -23408;
	// 826EF8B8: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826EF8BC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826EF8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF8C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF8D0: 386A7234  addi r3, r10, 0x7234
	ctx.r[3].s64 = ctx.r[10].s64 + 29236;
	// 826EF8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF8F4: 4BD7752D  bl 0x82466e20
	ctx.lr = 0x826EF8F8;
	sub_82466E20(ctx, base);
	// 826EF8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF908 size=112
    let mut pc: u32 = 0x826EF908;
    'dispatch: loop {
        match pc {
            0x826EF908 => {
    //   block [0x826EF908..0x826EF978)
	// 826EF908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF914: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF918: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF91C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF924: 390BA5C8  addi r8, r11, -0x5a38
	ctx.r[8].s64 = ctx.r[11].s64 + -23096;
	// 826EF928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF92C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826EF930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF940: 386A7264  addi r3, r10, 0x7264
	ctx.r[3].s64 = ctx.r[10].s64 + 29284;
	// 826EF944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF964: 4BD774BD  bl 0x82466e20
	ctx.lr = 0x826EF968;
	sub_82466E20(ctx, base);
	// 826EF968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF978 size=116
    let mut pc: u32 = 0x826EF978;
    'dispatch: loop {
        match pc {
            0x826EF978 => {
    //   block [0x826EF978..0x826EF9EC)
	// 826EF978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF984: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF988: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EF98C: 390BA5E0  addi r8, r11, -0x5a20
	ctx.r[8].s64 = ctx.r[11].s64 + -23072;
	// 826EF990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF994: 392A97E8  addi r9, r10, -0x6818
	ctx.r[9].s64 = ctx.r[10].s64 + -26648;
	// 826EF998: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF99C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826EF9A0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EF9A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF9AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF9BC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EF9C0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826EF9C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF9C8: 386B7294  addi r3, r11, 0x7294
	ctx.r[3].s64 = ctx.r[11].s64 + 29332;
	// 826EF9CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EF9D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF9D8: 4BD77449  bl 0x82466e20
	ctx.lr = 0x826EF9DC;
	sub_82466E20(ctx, base);
	// 826EF9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF9F0 size=100
    let mut pc: u32 = 0x826EF9F0;
    'dispatch: loop {
        match pc {
            0x826EF9F0 => {
    //   block [0x826EF9F0..0x826EFA54)
	// 826EF9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF9FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFA04: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFA10: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826EFA14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFA24: 386A72C4  addi r3, r10, 0x72c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29380;
	// 826EFA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFA2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFA30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EFA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFA38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EFA3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFA40: 4BD773E1  bl 0x82466e20
	ctx.lr = 0x826EFA44;
	sub_82466E20(ctx, base);
	// 826EFA44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFA48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFA4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFA58 size=112
    let mut pc: u32 = 0x826EFA58;
    'dispatch: loop {
        match pc {
            0x826EFA58 => {
    //   block [0x826EFA58..0x826EFAC8)
	// 826EFA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFA64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFA6C: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 826EFA70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFA74: 390BA610  addi r8, r11, -0x59f0
	ctx.r[8].s64 = ctx.r[11].s64 + -23024;
	// 826EFA78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EFA7C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826EFA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFA84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFA90: 386A72F4  addi r3, r10, 0x72f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29428;
	// 826EFA94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFA98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFAAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFAB4: 4BD7736D  bl 0x82466e20
	ctx.lr = 0x826EFAB8;
	sub_82466E20(ctx, base);
	// 826EFAB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFAC8 size=112
    let mut pc: u32 = 0x826EFAC8;
    'dispatch: loop {
        match pc {
            0x826EFAC8 => {
    //   block [0x826EFAC8..0x826EFB38)
	// 826EFAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFAD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFAD8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFADC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFAE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFAE4: 390BA628  addi r8, r11, -0x59d8
	ctx.r[8].s64 = ctx.r[11].s64 + -23000;
	// 826EFAE8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826EFAEC: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826EFAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFAF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFAF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFB00: 386A7324  addi r3, r10, 0x7324
	ctx.r[3].s64 = ctx.r[10].s64 + 29476;
	// 826EFB04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFB08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFB1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFB24: 4BD772FD  bl 0x82466e20
	ctx.lr = 0x826EFB28;
	sub_82466E20(ctx, base);
	// 826EFB28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFB38 size=112
    let mut pc: u32 = 0x826EFB38;
    'dispatch: loop {
        match pc {
            0x826EFB38 => {
    //   block [0x826EFB38..0x826EFBA8)
	// 826EFB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFB44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFB48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFB4C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFB50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFB54: 390BA6D0  addi r8, r11, -0x5930
	ctx.r[8].s64 = ctx.r[11].s64 + -22832;
	// 826EFB58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EFB5C: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826EFB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFB64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFB68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFB70: 386A7354  addi r3, r10, 0x7354
	ctx.r[3].s64 = ctx.r[10].s64 + 29524;
	// 826EFB74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFB78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFB80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFB88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFB8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFB90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFB94: 4BD7728D  bl 0x82466e20
	ctx.lr = 0x826EFB98;
	sub_82466E20(ctx, base);
	// 826EFB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFBA8 size=112
    let mut pc: u32 = 0x826EFBA8;
    'dispatch: loop {
        match pc {
            0x826EFBA8 => {
    //   block [0x826EFBA8..0x826EFC18)
	// 826EFBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFBB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFBB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFBBC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFBC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFBC4: 390BA718  addi r8, r11, -0x58e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22760;
	// 826EFBC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EFBCC: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826EFBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFBD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFBD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFBE0: 386A7384  addi r3, r10, 0x7384
	ctx.r[3].s64 = ctx.r[10].s64 + 29572;
	// 826EFBE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFBE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFBF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFBF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFC04: 4BD7721D  bl 0x82466e20
	ctx.lr = 0x826EFC08;
	sub_82466E20(ctx, base);
	// 826EFC08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFC18 size=116
    let mut pc: u32 = 0x826EFC18;
    'dispatch: loop {
        match pc {
            0x826EFC18 => {
    //   block [0x826EFC18..0x826EFC8C)
	// 826EFC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFC24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EFC28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826EFC2C: 390AA748  addi r8, r10, -0x58b8
	ctx.r[8].s64 = ctx.r[10].s64 + -22712;
	// 826EFC30: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFC34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EFC38: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EFC3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFC40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EFC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFC48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFC4C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826EFC50: 396B97FC  addi r11, r11, -0x6804
	ctx.r[11].s64 = ctx.r[11].s64 + -26628;
	// 826EFC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFC58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFC5C: 386A73B4  addi r3, r10, 0x73b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29620;
	// 826EFC60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EFC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFC68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EFC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFC70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFC78: 4BD771A9  bl 0x82466e20
	ctx.lr = 0x826EFC7C;
	sub_82466E20(ctx, base);
	// 826EFC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFC90 size=100
    let mut pc: u32 = 0x826EFC90;
    'dispatch: loop {
        match pc {
            0x826EFC90 => {
    //   block [0x826EFC90..0x826EFCF4)
	// 826EFC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFCA4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFCB0: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826EFCB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFCC4: 386A73E4  addi r3, r10, 0x73e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29668;
	// 826EFCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EFCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EFCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFCE0: 4BD77141  bl 0x82466e20
	ctx.lr = 0x826EFCE4;
	sub_82466E20(ctx, base);
	// 826EFCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFCF8 size=108
    let mut pc: u32 = 0x826EFCF8;
    'dispatch: loop {
        match pc {
            0x826EFCF8 => {
    //   block [0x826EFCF8..0x826EFD64)
	// 826EFCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFD04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFD0C: 38EBA808  addi r7, r11, -0x57f8
	ctx.r[7].s64 = ctx.r[11].s64 + -22520;
	// 826EFD10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFD14: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826EFD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFD1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFD20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFD28: 386A7414  addi r3, r10, 0x7414
	ctx.r[3].s64 = ctx.r[10].s64 + 29716;
	// 826EFD2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFD4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFD50: 4BD770D1  bl 0x82466e20
	ctx.lr = 0x826EFD54;
	sub_82466E20(ctx, base);
	// 826EFD54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFD68 size=112
    let mut pc: u32 = 0x826EFD68;
    'dispatch: loop {
        match pc {
            0x826EFD68 => {
    //   block [0x826EFD68..0x826EFDD8)
	// 826EFD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFD74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFD78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFD7C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826EFD80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFD84: 390BA838  addi r8, r11, -0x57c8
	ctx.r[8].s64 = ctx.r[11].s64 + -22472;
	// 826EFD88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EFD8C: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826EFD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFD94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFD98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFDA0: 386A7444  addi r3, r10, 0x7444
	ctx.r[3].s64 = ctx.r[10].s64 + 29764;
	// 826EFDA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFDB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFDB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFDC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFDC4: 4BD7705D  bl 0x82466e20
	ctx.lr = 0x826EFDC8;
	sub_82466E20(ctx, base);
	// 826EFDC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFDD8 size=108
    let mut pc: u32 = 0x826EFDD8;
    'dispatch: loop {
        match pc {
            0x826EFDD8 => {
    //   block [0x826EFDD8..0x826EFE44)
	// 826EFDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFDE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFDE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFDEC: 38EBA868  addi r7, r11, -0x5798
	ctx.r[7].s64 = ctx.r[11].s64 + -22424;
	// 826EFDF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFDF4: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 826EFDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFDFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFE00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFE08: 386A7474  addi r3, r10, 0x7474
	ctx.r[3].s64 = ctx.r[10].s64 + 29812;
	// 826EFE0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFE2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFE30: 4BD76FF1  bl 0x82466e20
	ctx.lr = 0x826EFE34;
	sub_82466E20(ctx, base);
	// 826EFE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFE48 size=112
    let mut pc: u32 = 0x826EFE48;
    'dispatch: loop {
        match pc {
            0x826EFE48 => {
    //   block [0x826EFE48..0x826EFEB8)
	// 826EFE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFE54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFE58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFE5C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826EFE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFE64: 390BA898  addi r8, r11, -0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + -22376;
	// 826EFE68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EFE6C: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826EFE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFE74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFE78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFE80: 386A74A4  addi r3, r10, 0x74a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29860;
	// 826EFE84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFEA4: 4BD76F7D  bl 0x82466e20
	ctx.lr = 0x826EFEA8;
	sub_82466E20(ctx, base);
	// 826EFEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFEB8 size=108
    let mut pc: u32 = 0x826EFEB8;
    'dispatch: loop {
        match pc {
            0x826EFEB8 => {
    //   block [0x826EFEB8..0x826EFF24)
	// 826EFEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFEC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFECC: 38EBA8E0  addi r7, r11, -0x5720
	ctx.r[7].s64 = ctx.r[11].s64 + -22304;
	// 826EFED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFED4: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 826EFED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFEDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFEE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFEE8: 386A74D4  addi r3, r10, 0x74d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29908;
	// 826EFEEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFF0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFF10: 4BD76F11  bl 0x82466e20
	ctx.lr = 0x826EFF14;
	sub_82466E20(ctx, base);
	// 826EFF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFF28 size=112
    let mut pc: u32 = 0x826EFF28;
    'dispatch: loop {
        match pc {
            0x826EFF28 => {
    //   block [0x826EFF28..0x826EFF98)
	// 826EFF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFF38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFF3C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826EFF40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFF44: 390BA910  addi r8, r11, -0x56f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22256;
	// 826EFF48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EFF4C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826EFF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFF54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFF58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFF60: 386A7504  addi r3, r10, 0x7504
	ctx.r[3].s64 = ctx.r[10].s64 + 29956;
	// 826EFF64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFF68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFF84: 4BD76E9D  bl 0x82466e20
	ctx.lr = 0x826EFF88;
	sub_82466E20(ctx, base);
	// 826EFF88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFF98 size=108
    let mut pc: u32 = 0x826EFF98;
    'dispatch: loop {
        match pc {
            0x826EFF98 => {
    //   block [0x826EFF98..0x826F0004)
	// 826EFF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFFA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFFAC: 38EBA958  addi r7, r11, -0x56a8
	ctx.r[7].s64 = ctx.r[11].s64 + -22184;
	// 826EFFB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFFB4: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 826EFFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFFBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFFC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFFC8: 386A7534  addi r3, r10, 0x7534
	ctx.r[3].s64 = ctx.r[10].s64 + 30004;
	// 826EFFCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFFEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFFF0: 4BD76E31  bl 0x82466e20
	ctx.lr = 0x826EFFF4;
	sub_82466E20(ctx, base);
	// 826EFFF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFFF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFFFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0008 size=112
    let mut pc: u32 = 0x826F0008;
    'dispatch: loop {
        match pc {
            0x826F0008 => {
    //   block [0x826F0008..0x826F0078)
	// 826F0008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0014: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0018: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F001C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826F0020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0024: 390BA988  addi r8, r11, -0x5678
	ctx.r[8].s64 = ctx.r[11].s64 + -22136;
	// 826F0028: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F002C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826F0030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0034: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F003C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0040: 386A7564  addi r3, r10, 0x7564
	ctx.r[3].s64 = ctx.r[10].s64 + 30052;
	// 826F0044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F004C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0064: 4BD76DBD  bl 0x82466e20
	ctx.lr = 0x826F0068;
	sub_82466E20(ctx, base);
	// 826F0068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F006C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0078 size=108
    let mut pc: u32 = 0x826F0078;
    'dispatch: loop {
        match pc {
            0x826F0078 => {
    //   block [0x826F0078..0x826F00E4)
	// 826F0078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F007C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0084: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F008C: 38EBA9D8  addi r7, r11, -0x5628
	ctx.r[7].s64 = ctx.r[11].s64 + -22056;
	// 826F0090: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F0094: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826F0098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F009C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F00A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F00A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F00A8: 386A7594  addi r3, r10, 0x7594
	ctx.r[3].s64 = ctx.r[10].s64 + 30100;
	// 826F00AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F00B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F00B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F00B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F00BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F00C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F00C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F00C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F00CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F00D0: 4BD76D51  bl 0x82466e20
	ctx.lr = 0x826F00D4;
	sub_82466E20(ctx, base);
	// 826F00D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F00D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F00DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F00E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F00E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F00E8 size=112
    let mut pc: u32 = 0x826F00E8;
    'dispatch: loop {
        match pc {
            0x826F00E8 => {
    //   block [0x826F00E8..0x826F0158)
	// 826F00E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F00EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F00F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F00F4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F00F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F00FC: 392A98B8  addi r9, r10, -0x6748
	ctx.r[9].s64 = ctx.r[10].s64 + -26440;
	// 826F0100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0104: 390BAA38  addi r8, r11, -0x55c8
	ctx.r[8].s64 = ctx.r[11].s64 + -21960;
	// 826F0108: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F010C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826F0110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0114: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F011C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0120: 386A75C4  addi r3, r10, 0x75c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30148;
	// 826F0124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F012C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F013C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0144: 4BD76CDD  bl 0x82466e20
	ctx.lr = 0x826F0148;
	sub_82466E20(ctx, base);
	// 826F0148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0158 size=108
    let mut pc: u32 = 0x826F0158;
    'dispatch: loop {
        match pc {
            0x826F0158 => {
    //   block [0x826F0158..0x826F01C4)
	// 826F0158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F015C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0164: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F016C: 38EBAAF8  addi r7, r11, -0x5508
	ctx.r[7].s64 = ctx.r[11].s64 + -21768;
	// 826F0170: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F0174: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826F0178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F017C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0188: 386A75F4  addi r3, r10, 0x75f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30196;
	// 826F018C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F019C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F01A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F01A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F01A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F01AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F01B0: 4BD76C71  bl 0x82466e20
	ctx.lr = 0x826F01B4;
	sub_82466E20(ctx, base);
	// 826F01B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F01B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F01BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F01C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F01C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F01C8 size=108
    let mut pc: u32 = 0x826F01C8;
    'dispatch: loop {
        match pc {
            0x826F01C8 => {
    //   block [0x826F01C8..0x826F0234)
	// 826F01C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F01CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F01D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F01D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F01D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F01DC: 38EBAB40  addi r7, r11, -0x54c0
	ctx.r[7].s64 = ctx.r[11].s64 + -21696;
	// 826F01E0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F01E4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826F01E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F01EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F01F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F01F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F01F8: 386A7624  addi r3, r10, 0x7624
	ctx.r[3].s64 = ctx.r[10].s64 + 30244;
	// 826F01FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F020C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F021C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0220: 4BD76C01  bl 0x82466e20
	ctx.lr = 0x826F0224;
	sub_82466E20(ctx, base);
	// 826F0224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F022C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0238 size=24
    let mut pc: u32 = 0x826F0238;
    'dispatch: loop {
        match pc {
            0x826F0238 => {
    //   block [0x826F0238..0x826F0250)
	// 826F0238: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F023C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0240: 394A1490  addi r10, r10, 0x1490
	ctx.r[10].s64 = ctx.r[10].s64 + 5264;
	// 826F0244: 816BA9D0  lwz r11, -0x5630(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22064 as u32) ) } as u64;
	// 826F0248: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 826F024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0250 size=116
    let mut pc: u32 = 0x826F0250;
    'dispatch: loop {
        match pc {
            0x826F0250 => {
    //   block [0x826F0250..0x826F02C4)
	// 826F0250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F025C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F0260: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0264: 392B9844  addi r9, r11, -0x67bc
	ctx.r[9].s64 = ctx.r[11].s64 + -26556;
	// 826F0268: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826F026C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0270: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 826F0274: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 826F0278: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F027C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826F0280: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0284: 396B1490  addi r11, r11, 0x1490
	ctx.r[11].s64 = ctx.r[11].s64 + 5264;
	// 826F0288: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F028C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0290: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F0294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0298: 386A7654  addi r3, r10, 0x7654
	ctx.r[3].s64 = ctx.r[10].s64 + 30292;
	// 826F029C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F02A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F02A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F02A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F02AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F02B0: 4BD76B71  bl 0x82466e20
	ctx.lr = 0x826F02B4;
	sub_82466E20(ctx, base);
	// 826F02B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F02B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F02BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F02C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F02C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F02C8 size=100
    let mut pc: u32 = 0x826F02C8;
    'dispatch: loop {
        match pc {
            0x826F02C8 => {
    //   block [0x826F02C8..0x826F032C)
	// 826F02C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F02CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F02D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F02D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F02D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F02DC: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826F02E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F02E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F02E8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826F02EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F02F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F02F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F02F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F02FC: 386A7684  addi r3, r10, 0x7684
	ctx.r[3].s64 = ctx.r[10].s64 + 30340;
	// 826F0300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0304: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0308: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F030C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F0314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0318: 4BD76B09  bl 0x82466e20
	ctx.lr = 0x826F031C;
	sub_82466E20(ctx, base);
	// 826F031C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0330 size=24
    let mut pc: u32 = 0x826F0330;
    'dispatch: loop {
        match pc {
            0x826F0330 => {
    //   block [0x826F0330..0x826F0348)
	// 826F0330: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0334: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0338: 394A1640  addi r10, r10, 0x1640
	ctx.r[10].s64 = ctx.r[10].s64 + 5696;
	// 826F033C: 816BABEC  lwz r11, -0x5414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21524 as u32) ) } as u64;
	// 826F0340: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F0344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0348 size=116
    let mut pc: u32 = 0x826F0348;
    'dispatch: loop {
        match pc {
            0x826F0348 => {
    //   block [0x826F0348..0x826F03BC)
	// 826F0348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0354: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0358: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F035C: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 826F0360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0364: 392A9950  addi r9, r10, -0x66b0
	ctx.r[9].s64 = ctx.r[10].s64 + -26288;
	// 826F0368: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F036C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826F0370: 38AA7684  addi r5, r10, 0x7684
	ctx.r[5].s64 = ctx.r[10].s64 + 30340;
	// 826F0374: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F037C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F038C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826F0390: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826F0394: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0398: 386B76B4  addi r3, r11, 0x76b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30388;
	// 826F039C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F03A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F03A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F03A8: 4BD76A79  bl 0x82466e20
	ctx.lr = 0x826F03AC;
	sub_82466E20(ctx, base);
	// 826F03AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F03B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F03B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F03B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F03C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F03C0 size=112
    let mut pc: u32 = 0x826F03C0;
    'dispatch: loop {
        match pc {
            0x826F03C0 => {
    //   block [0x826F03C0..0x826F0430)
	// 826F03C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F03C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F03C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F03CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F03D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F03D4: 38AA7684  addi r5, r10, 0x7684
	ctx.r[5].s64 = ctx.r[10].s64 + 30340;
	// 826F03D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F03DC: 390BABF0  addi r8, r11, -0x5410
	ctx.r[8].s64 = ctx.r[11].s64 + -21520;
	// 826F03E0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826F03E4: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826F03E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F03EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F03F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F03F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F03F8: 386A76E4  addi r3, r10, 0x76e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30436;
	// 826F03FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F040C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F041C: 4BD76A05  bl 0x82466e20
	ctx.lr = 0x826F0420;
	sub_82466E20(ctx, base);
	// 826F0420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F042C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0430 size=112
    let mut pc: u32 = 0x826F0430;
    'dispatch: loop {
        match pc {
            0x826F0430 => {
    //   block [0x826F0430..0x826F04A0)
	// 826F0430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F043C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0440: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0444: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F0448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F044C: 390BACC8  addi r8, r11, -0x5338
	ctx.r[8].s64 = ctx.r[11].s64 + -21304;
	// 826F0450: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826F0454: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826F0458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F045C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0468: 386A7714  addi r3, r10, 0x7714
	ctx.r[3].s64 = ctx.r[10].s64 + 30484;
	// 826F046C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F047C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F048C: 4BD76995  bl 0x82466e20
	ctx.lr = 0x826F0490;
	sub_82466E20(ctx, base);
	// 826F0490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F049C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F04A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F04A0 size=108
    let mut pc: u32 = 0x826F04A0;
    'dispatch: loop {
        match pc {
            0x826F04A0 => {
    //   block [0x826F04A0..0x826F050C)
	// 826F04A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F04A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F04A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F04AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F04B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F04B4: 38EBADA0  addi r7, r11, -0x5260
	ctx.r[7].s64 = ctx.r[11].s64 + -21088;
	// 826F04B8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F04BC: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826F04C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F04C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F04C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F04CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F04D0: 386A7744  addi r3, r10, 0x7744
	ctx.r[3].s64 = ctx.r[10].s64 + 30532;
	// 826F04D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F04D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F04DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F04E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F04E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F04E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F04EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F04F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F04F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F04F8: 4BD76929  bl 0x82466e20
	ctx.lr = 0x826F04FC;
	sub_82466E20(ctx, base);
	// 826F04FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0510 size=108
    let mut pc: u32 = 0x826F0510;
    'dispatch: loop {
        match pc {
            0x826F0510 => {
    //   block [0x826F0510..0x826F057C)
	// 826F0510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F051C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0524: 38EBAE18  addi r7, r11, -0x51e8
	ctx.r[7].s64 = ctx.r[11].s64 + -20968;
	// 826F0528: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F052C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826F0530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0534: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F053C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0540: 386A7774  addi r3, r10, 0x7774
	ctx.r[3].s64 = ctx.r[10].s64 + 30580;
	// 826F0544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F055C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0568: 4BD768B9  bl 0x82466e20
	ctx.lr = 0x826F056C;
	sub_82466E20(ctx, base);
	// 826F056C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0580 size=112
    let mut pc: u32 = 0x826F0580;
    'dispatch: loop {
        match pc {
            0x826F0580 => {
    //   block [0x826F0580..0x826F05F0)
	// 826F0580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F058C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0590: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0594: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F0598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F059C: 390BAE60  addi r8, r11, -0x51a0
	ctx.r[8].s64 = ctx.r[11].s64 + -20896;
	// 826F05A0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 826F05A4: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826F05A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F05AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F05B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F05B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F05B8: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 826F05BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F05C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F05C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F05C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F05CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F05D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F05D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F05D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F05DC: 4BD76845  bl 0x82466e20
	ctx.lr = 0x826F05E0;
	sub_82466E20(ctx, base);
	// 826F05E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F05E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F05E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F05EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F05F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F05F0 size=108
    let mut pc: u32 = 0x826F05F0;
    'dispatch: loop {
        match pc {
            0x826F05F0 => {
    //   block [0x826F05F0..0x826F065C)
	// 826F05F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F05F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F05F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F05FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0604: 38EBB040  addi r7, r11, -0x4fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -20416;
	// 826F0608: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F060C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826F0610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F061C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0620: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 826F0624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F062C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F063C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0648: 4BD767D9  bl 0x82466e20
	ctx.lr = 0x826F064C;
	sub_82466E20(ctx, base);
	// 826F064C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0660 size=24
    let mut pc: u32 = 0x826F0660;
    'dispatch: loop {
        match pc {
            0x826F0660 => {
    //   block [0x826F0660..0x826F0678)
	// 826F0660: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0664: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0668: 394A1760  addi r10, r10, 0x1760
	ctx.r[10].s64 = ctx.r[10].s64 + 5984;
	// 826F066C: 816BB058  lwz r11, -0x4fa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20392 as u32) ) } as u64;
	// 826F0670: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F0674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0678 size=112
    let mut pc: u32 = 0x826F0678;
    'dispatch: loop {
        match pc {
            0x826F0678 => {
    //   block [0x826F0678..0x826F06E8)
	// 826F0678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F067C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0684: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0688: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F068C: 392A99A8  addi r9, r10, -0x6658
	ctx.r[9].s64 = ctx.r[10].s64 + -26200;
	// 826F0690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0694: 390B1760  addi r8, r11, 0x1760
	ctx.r[8].s64 = ctx.r[11].s64 + 5984;
	// 826F0698: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F069C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826F06A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F06A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F06A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F06AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F06B0: 386A7804  addi r3, r10, 0x7804
	ctx.r[3].s64 = ctx.r[10].s64 + 30724;
	// 826F06B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F06B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F06BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F06C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F06C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F06C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F06CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F06D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F06D4: 4BD7674D  bl 0x82466e20
	ctx.lr = 0x826F06D8;
	sub_82466E20(ctx, base);
	// 826F06D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F06DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F06E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F06E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F06E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F06E8 size=112
    let mut pc: u32 = 0x826F06E8;
    'dispatch: loop {
        match pc {
            0x826F06E8 => {
    //   block [0x826F06E8..0x826F0758)
	// 826F06E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F06EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F06F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F06F4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F06F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F06FC: 38EAB060  addi r7, r10, -0x4fa0
	ctx.r[7].s64 = ctx.r[10].s64 + -20384;
	// 826F0700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0704: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F0708: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826F070C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0710: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0714: 396B99BC  addi r11, r11, -0x6644
	ctx.r[11].s64 = ctx.r[11].s64 + -26180;
	// 826F0718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F071C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0724: 386A7834  addi r3, r10, 0x7834
	ctx.r[3].s64 = ctx.r[10].s64 + 30772;
	// 826F0728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F072C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F0730: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0734: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F0738: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F073C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0740: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0744: 4BD766DD  bl 0x82466e20
	ctx.lr = 0x826F0748;
	sub_82466E20(ctx, base);
	// 826F0748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F074C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0758 size=112
    let mut pc: u32 = 0x826F0758;
    'dispatch: loop {
        match pc {
            0x826F0758 => {
    //   block [0x826F0758..0x826F07C8)
	// 826F0758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0768: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F076C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F0770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0774: 390BB0F0  addi r8, r11, -0x4f10
	ctx.r[8].s64 = ctx.r[11].s64 + -20240;
	// 826F0778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F077C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826F0780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F078C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0790: 386A7864  addi r3, r10, 0x7864
	ctx.r[3].s64 = ctx.r[10].s64 + 30820;
	// 826F0794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F079C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F07A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F07A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F07A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F07AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F07B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F07B4: 4BD7666D  bl 0x82466e20
	ctx.lr = 0x826F07B8;
	sub_82466E20(ctx, base);
	// 826F07B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F07BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F07C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F07C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F07C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F07C8 size=108
    let mut pc: u32 = 0x826F07C8;
    'dispatch: loop {
        match pc {
            0x826F07C8 => {
    //   block [0x826F07C8..0x826F0834)
	// 826F07C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F07CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F07D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F07D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F07D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F07DC: 38EBB108  addi r7, r11, -0x4ef8
	ctx.r[7].s64 = ctx.r[11].s64 + -20216;
	// 826F07E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F07E4: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826F07E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F07EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F07F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F07F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F07F8: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 826F07FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F080C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F081C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0820: 4BD76601  bl 0x82466e20
	ctx.lr = 0x826F0824;
	sub_82466E20(ctx, base);
	// 826F0824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F082C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0838 size=108
    let mut pc: u32 = 0x826F0838;
    'dispatch: loop {
        match pc {
            0x826F0838 => {
    //   block [0x826F0838..0x826F08A4)
	// 826F0838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0844: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F084C: 38EBB168  addi r7, r11, -0x4e98
	ctx.r[7].s64 = ctx.r[11].s64 + -20120;
	// 826F0850: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0854: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826F0858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F085C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0868: 386A78C4  addi r3, r10, 0x78c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30916;
	// 826F086C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F087C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F088C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0890: 4BD76591  bl 0x82466e20
	ctx.lr = 0x826F0894;
	sub_82466E20(ctx, base);
	// 826F0894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F089C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F08A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F08A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F08A8 size=116
    let mut pc: u32 = 0x826F08A8;
    'dispatch: loop {
        match pc {
            0x826F08A8 => {
    //   block [0x826F08A8..0x826F091C)
	// 826F08A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F08AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F08B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F08B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F08B8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F08BC: 390BB198  addi r8, r11, -0x4e68
	ctx.r[8].s64 = ctx.r[11].s64 + -20072;
	// 826F08C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F08C4: 392A99F0  addi r9, r10, -0x6610
	ctx.r[9].s64 = ctx.r[10].s64 + -26128;
	// 826F08C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F08CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F08D0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F08D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F08D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F08DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F08E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F08E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F08E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F08EC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826F08F0: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826F08F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F08F8: 386B78F4  addi r3, r11, 0x78f4
	ctx.r[3].s64 = ctx.r[11].s64 + 30964;
	// 826F08FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0900: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0908: 4BD76519  bl 0x82466e20
	ctx.lr = 0x826F090C;
	sub_82466E20(ctx, base);
	// 826F090C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0920 size=96
    let mut pc: u32 = 0x826F0920;
    'dispatch: loop {
        match pc {
            0x826F0920 => {
    //   block [0x826F0920..0x826F0980)
	// 826F0920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F092C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0934: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826F0938: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F093C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0940: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 826F0944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F094C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F0950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F095C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0960: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F0964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0968: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F096C: 4BD764B5  bl 0x82466e20
	ctx.lr = 0x826F0970;
	sub_82466E20(ctx, base);
	// 826F0970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F097C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0980 size=112
    let mut pc: u32 = 0x826F0980;
    'dispatch: loop {
        match pc {
            0x826F0980 => {
    //   block [0x826F0980..0x826F09F0)
	// 826F0980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F098C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0994: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 826F0998: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F099C: 390BB1B0  addi r8, r11, -0x4e50
	ctx.r[8].s64 = ctx.r[11].s64 + -20048;
	// 826F09A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F09A4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826F09A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F09AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F09B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F09B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F09B8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 826F09BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F09C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F09C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F09C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F09CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F09D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F09D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F09D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F09DC: 4BD76445  bl 0x82466e20
	ctx.lr = 0x826F09E0;
	sub_82466E20(ctx, base);
	// 826F09E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F09E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F09E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F09EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F09F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F09F0 size=112
    let mut pc: u32 = 0x826F09F0;
    'dispatch: loop {
        match pc {
            0x826F09F0 => {
    //   block [0x826F09F0..0x826F0A60)
	// 826F09F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F09F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F09F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F09FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0A00: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0A04: 392A9A0C  addi r9, r10, -0x65f4
	ctx.r[9].s64 = ctx.r[10].s64 + -26100;
	// 826F0A08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0A0C: 390BB1E8  addi r8, r11, -0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + -19992;
	// 826F0A10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F0A14: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826F0A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0A1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0A20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0A28: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 826F0A2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0A30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0A44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0A4C: 4BD763D5  bl 0x82466e20
	ctx.lr = 0x826F0A50;
	sub_82466E20(ctx, base);
	// 826F0A50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0A60 size=108
    let mut pc: u32 = 0x826F0A60;
    'dispatch: loop {
        match pc {
            0x826F0A60 => {
    //   block [0x826F0A60..0x826F0ACC)
	// 826F0A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0A6C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0A70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0A74: 38EBB290  addi r7, r11, -0x4d70
	ctx.r[7].s64 = ctx.r[11].s64 + -19824;
	// 826F0A78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0A7C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826F0A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0A88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0A90: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 826F0A94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0AB8: 4BD76369  bl 0x82466e20
	ctx.lr = 0x826F0ABC;
	sub_82466E20(ctx, base);
	// 826F0ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0AD0 size=108
    let mut pc: u32 = 0x826F0AD0;
    'dispatch: loop {
        match pc {
            0x826F0AD0 => {
    //   block [0x826F0AD0..0x826F0B3C)
	// 826F0AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0ADC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0AE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0AE4: 38EBB2C0  addi r7, r11, -0x4d40
	ctx.r[7].s64 = ctx.r[11].s64 + -19776;
	// 826F0AE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0AEC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826F0AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0AF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0AF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0B00: 386A79E4  addi r3, r10, 0x79e4
	ctx.r[3].s64 = ctx.r[10].s64 + 31204;
	// 826F0B04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0B24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0B28: 4BD762F9  bl 0x82466e20
	ctx.lr = 0x826F0B2C;
	sub_82466E20(ctx, base);
	// 826F0B2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0B40 size=28
    let mut pc: u32 = 0x826F0B40;
    'dispatch: loop {
        match pc {
            0x826F0B40 => {
    //   block [0x826F0B40..0x826F0B5C)
	// 826F0B40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0B44: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0B48: 394A1790  addi r10, r10, 0x1790
	ctx.r[10].s64 = ctx.r[10].s64 + 6032;
	// 826F0B4C: 816BB1E4  lwz r11, -0x4e1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19996 as u32) ) } as u64;
	// 826F0B50: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F0B54: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F0B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0B60 size=112
    let mut pc: u32 = 0x826F0B60;
    'dispatch: loop {
        match pc {
            0x826F0B60 => {
    //   block [0x826F0B60..0x826F0BD0)
	// 826F0B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0B6C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0B70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0B74: 392A9BB8  addi r9, r10, -0x6448
	ctx.r[9].s64 = ctx.r[10].s64 + -25672;
	// 826F0B78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0B7C: 390B1790  addi r8, r11, 0x1790
	ctx.r[8].s64 = ctx.r[11].s64 + 6032;
	// 826F0B80: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F0B84: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826F0B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0B8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0B98: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 826F0B9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0BA0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826F0BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0BBC: 4BD76265  bl 0x82466e20
	ctx.lr = 0x826F0BC0;
	sub_82466E20(ctx, base);
	// 826F0BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0BD0 size=108
    let mut pc: u32 = 0x826F0BD0;
    'dispatch: loop {
        match pc {
            0x826F0BD0 => {
    //   block [0x826F0BD0..0x826F0C3C)
	// 826F0BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0BDC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0BE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0BE4: 38EBB2F8  addi r7, r11, -0x4d08
	ctx.r[7].s64 = ctx.r[11].s64 + -19720;
	// 826F0BE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0BEC: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 826F0BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0BF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0C00: 386A7A44  addi r3, r10, 0x7a44
	ctx.r[3].s64 = ctx.r[10].s64 + 31300;
	// 826F0C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0C28: 4BD761F9  bl 0x82466e20
	ctx.lr = 0x826F0C2C;
	sub_82466E20(ctx, base);
	// 826F0C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0C40 size=108
    let mut pc: u32 = 0x826F0C40;
    'dispatch: loop {
        match pc {
            0x826F0C40 => {
    //   block [0x826F0C40..0x826F0CAC)
	// 826F0C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0C4C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0C50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0C54: 38EBB328  addi r7, r11, -0x4cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -19672;
	// 826F0C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0C5C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826F0C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0C64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0C70: 386A7A74  addi r3, r10, 0x7a74
	ctx.r[3].s64 = ctx.r[10].s64 + 31348;
	// 826F0C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0C98: 4BD76189  bl 0x82466e20
	ctx.lr = 0x826F0C9C;
	sub_82466E20(ctx, base);
	// 826F0C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0CB0 size=108
    let mut pc: u32 = 0x826F0CB0;
    'dispatch: loop {
        match pc {
            0x826F0CB0 => {
    //   block [0x826F0CB0..0x826F0D1C)
	// 826F0CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0CBC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0CC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0CC4: 38EBB358  addi r7, r11, -0x4ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -19624;
	// 826F0CC8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F0CCC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826F0CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0CD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0CE0: 386A7AA4  addi r3, r10, 0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 31396;
	// 826F0CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0D08: 4BD76119  bl 0x82466e20
	ctx.lr = 0x826F0D0C;
	sub_82466E20(ctx, base);
	// 826F0D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0D20 size=24
    let mut pc: u32 = 0x826F0D20;
    'dispatch: loop {
        match pc {
            0x826F0D20 => {
    //   block [0x826F0D20..0x826F0D38)
	// 826F0D20: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0D24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0D28: 394A1850  addi r10, r10, 0x1850
	ctx.r[10].s64 = ctx.r[10].s64 + 6224;
	// 826F0D2C: 816BB370  lwz r11, -0x4c90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19600 as u32) ) } as u64;
	// 826F0D30: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F0D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0D38 size=112
    let mut pc: u32 = 0x826F0D38;
    'dispatch: loop {
        match pc {
            0x826F0D38 => {
    //   block [0x826F0D38..0x826F0DA8)
	// 826F0D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0D44: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0D48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0D4C: 392A9C0C  addi r9, r10, -0x63f4
	ctx.r[9].s64 = ctx.r[10].s64 + -25588;
	// 826F0D50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0D54: 390B1850  addi r8, r11, 0x1850
	ctx.r[8].s64 = ctx.r[11].s64 + 6224;
	// 826F0D58: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826F0D5C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826F0D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0D70: 386A7AD4  addi r3, r10, 0x7ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 31444;
	// 826F0D74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0D78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0D94: 4BD7608D  bl 0x82466e20
	ctx.lr = 0x826F0D98;
	sub_82466E20(ctx, base);
	// 826F0D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0DA8 size=112
    let mut pc: u32 = 0x826F0DA8;
    'dispatch: loop {
        match pc {
            0x826F0DA8 => {
    //   block [0x826F0DA8..0x826F0E18)
	// 826F0DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0DB4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0DB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0DBC: 392A9C48  addi r9, r10, -0x63b8
	ctx.r[9].s64 = ctx.r[10].s64 + -25528;
	// 826F0DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0DC4: 390BB380  addi r8, r11, -0x4c80
	ctx.r[8].s64 = ctx.r[11].s64 + -19584;
	// 826F0DC8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F0DCC: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826F0DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0DD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0DE0: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 826F0DE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0DE8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F0DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0E04: 4BD7601D  bl 0x82466e20
	ctx.lr = 0x826F0E08;
	sub_82466E20(ctx, base);
	// 826F0E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0E18 size=108
    let mut pc: u32 = 0x826F0E18;
    'dispatch: loop {
        match pc {
            0x826F0E18 => {
    //   block [0x826F0E18..0x826F0E84)
	// 826F0E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0E24: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0E28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0E2C: 38EBB3C8  addi r7, r11, -0x4c38
	ctx.r[7].s64 = ctx.r[11].s64 + -19512;
	// 826F0E30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0E34: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826F0E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0E48: 386A7B34  addi r3, r10, 0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + 31540;
	// 826F0E4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0E70: 4BD75FB1  bl 0x82466e20
	ctx.lr = 0x826F0E74;
	sub_82466E20(ctx, base);
	// 826F0E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0E88 size=108
    let mut pc: u32 = 0x826F0E88;
    'dispatch: loop {
        match pc {
            0x826F0E88 => {
    //   block [0x826F0E88..0x826F0EF4)
	// 826F0E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0E94: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0E98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0E9C: 38EBB3F8  addi r7, r11, -0x4c08
	ctx.r[7].s64 = ctx.r[11].s64 + -19464;
	// 826F0EA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0EA4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826F0EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0EB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0EB8: 386A7B64  addi r3, r10, 0x7b64
	ctx.r[3].s64 = ctx.r[10].s64 + 31588;
	// 826F0EBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0EE0: 4BD75F41  bl 0x82466e20
	ctx.lr = 0x826F0EE4;
	sub_82466E20(ctx, base);
	// 826F0EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0EF8 size=112
    let mut pc: u32 = 0x826F0EF8;
    'dispatch: loop {
        match pc {
            0x826F0EF8 => {
    //   block [0x826F0EF8..0x826F0F68)
	// 826F0EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0F04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0F08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0F0C: 392A9C88  addi r9, r10, -0x6378
	ctx.r[9].s64 = ctx.r[10].s64 + -25464;
	// 826F0F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0F14: 390BB428  addi r8, r11, -0x4bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -19416;
	// 826F0F18: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F0F1C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826F0F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0F30: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 826F0F34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0F38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0F54: 4BD75ECD  bl 0x82466e20
	ctx.lr = 0x826F0F58;
	sub_82466E20(ctx, base);
	// 826F0F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0F68 size=108
    let mut pc: u32 = 0x826F0F68;
    'dispatch: loop {
        match pc {
            0x826F0F68 => {
    //   block [0x826F0F68..0x826F0FD4)
	// 826F0F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0F74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0F78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0F7C: 38EBB4A0  addi r7, r11, -0x4b60
	ctx.r[7].s64 = ctx.r[11].s64 + -19296;
	// 826F0F80: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826F0F84: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826F0F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0F8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0F98: 386A7BC4  addi r3, r10, 0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31684;
	// 826F0F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0FC0: 4BD75E61  bl 0x82466e20
	ctx.lr = 0x826F0FC4;
	sub_82466E20(ctx, base);
	// 826F0FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


